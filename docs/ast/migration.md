# AST Migration Notes

AST shape changes and migration steps for downstream Rust users.

This guide is for crates that construct, inspect, or transform swc AST nodes.
The new AST and parser are still being rewritten, so some sections describe
changes available today while others describe upcoming migration targets.

## Status labels

- `Landed`: migrate to this shape when using the current AST.
- `Parser WIP`: the AST shape exists, but parser coverage may still be
  incomplete.
- `Planned`: the old shape will move in this direction, but the replacement AST
  shape may not exist yet.

When a section is marked `Planned`, do not assume the referenced replacement is
available as a public Rust type yet.

## 1. `Function.body` is required

Status: `Landed` for `Function`; `Planned` for body-less TypeScript function
declarations.

`Function.body` changed from `Option<Box<BlockStmt>>>` to `Box<BlockStmt>`.

- Replace `body: Some(body)` with `body`.
- Replace `function.body.as_ref()` / `as_mut()` / `if let Some(body)` code with
  direct access to `function.body`.
- Treat `None` from an older AST as "not representable as `Function`". Preserve
  it in a dedicated TypeScript declaration or overload AST shape once that shape
  exists.

```rust
// Before
let function = Function {
    span,
    params,
    decorators,
    body: Some(body),
    is_generator,
    is_async,
};

// After
let function = Function {
    span,
    params,
    decorators,
    body,
    is_generator,
    is_async,
};
```

```rust
// Before
if let Some(body) = function.body.as_mut() {
    visit_block_stmt(body);
}

// After
visit_block_stmt(&mut function.body);
```

## 2. `ArrowExpr.is_generator` was removed

Status: `Landed`.

`ArrowExpr` no longer has `is_generator`. Generator arrow functions are invalid
JavaScript syntax.

- Remove `is_generator` field initializers.
- Treat old `ArrowExpr { is_generator: true, .. }` values as invalid input;
  there is no replacement field.
- Converters from an older AST should reject `is_generator: true`, or normalize
  it only if the conversion is explicitly lossy.

```rust
// Before
let arrow = ArrowExpr {
    span,
    params,
    body,
    is_async,
    is_generator: false,
};

// After
let arrow = ArrowExpr {
    span,
    params,
    body,
    is_async,
};
```

## 3. `Ident.optional` was removed

Status: `Landed` for `Ident`; `Parser WIP` / `Planned` for some TypeScript
syntax owners.

`Ident` now describes only the identifier token. Optional syntax belongs to the
AST node that owns the `?` token.

- Remove `optional` field initializers.
- Move optional state to the owning AST node when that node exists.
- Do not use `Ident` as a carrier for TypeScript optional syntax.

```rust
// Before
let ident = Ident {
    span,
    sym,
    optional: false,
    symbol_id: Default::default(),
};

// After
let ident = Ident {
    span,
    sym,
    symbol_id: Default::default(),
};
```

## 4. `UpdateExpr.arg` is an assignment target

Status: `Landed`.

`UpdateExpr.arg` changed from `Expr` to `SimpleAssignTarget`.

This makes invalid update expressions such as `++(a + b)` unrepresentable in
the typed AST shape.

- Replace expression arguments with the matching `SimpleAssignTarget` variant.
- If old code accepted any `Expr`, validate and reject expressions that are not
  simple assignment targets.
- Use `SimpleAssignTarget::try_from_expr(expr, allocator)` when migrating from
  a legacy expression.
- For identifiers, use `SimpleAssignTarget::Ident`.
- For member expressions, use `SimpleAssignTarget::Member`.

```rust
// Before
let update = UpdateExpr {
    span,
    op,
    prefix,
    arg: Expr::Ident(ident),
};

// After
let arg = SimpleAssignTarget::try_from_expr(expr, allocator)?;
let update = UpdateExpr {
    span,
    op,
    prefix,
    arg,
};
```

## 5. `NewExpr.args` is required

Status: `Landed`.

`NewExpr.args` changed from `Option<Vec<ExprOrSpread>>` to
`Vec<ExprOrSpread>`.

`new Foo` and `new Foo()` now both use an argument vector. Use an empty vector
when the source has no argument list.

- Replace `args: Some(args)` with `args`.
- Replace `args: None` with an empty vector.
- Replace `if let Some(args)` checks with direct iteration over `new_expr.args`.

```rust
// Before
let new_expr = NewExpr {
    span,
    callee,
    args: None,
};

// After
let new_expr = NewExpr {
    span,
    callee,
    args: Vec::new_in(allocator),
};
```

## 6. Dynamic import is `Expr::Import`

Status: `Landed`.

Dynamic import now has its own expression node:

```rust
pub struct ImportExpr<'a> {
    pub span: Span,
    pub source: Expr<'a>,
    pub options: Option<Expr<'a>>,
    pub phase: ImportPhase,
}
```

`import(source, options)` is no longer represented as `CallExpr` with
`Callee::Import`.

- Match `Expr::Import(import_expr)` for dynamic import.
- Read the imported module from `import_expr.source`.
- Read import attributes/options from `import_expr.options`.
- Read source/defer/evaluation phase from `import_expr.phase`.
- Keep `CallExpr` handling for ordinary calls only.

```rust
// Before
if let Callee::Import(import) = &call.callee {
    let source = call.args.first();
    let phase = import.phase;
}

// After
if let Expr::Import(import) = expr {
    let source = &import.source;
    let options = import.options.as_ref();
    let phase = import.phase;
}
```

## 7. `ImportNamedSpecifier.imported` is required

Status: `Landed`.

`ImportNamedSpecifier.imported` changed from `Option<ModuleExportName>` to
`ModuleExportName`.

For shorthand imports such as `import { foo } from "mod"`, `imported` stores the
same name as `local`. Use `ImportNamedSpecifier::is_shorthand()` to detect the
shorthand form.

- Replace `imported: Some(name)` with `imported: name`.
- Replace `imported: None` with a `ModuleExportName` matching `local`.
- Replace `specifier.imported.is_none()` checks with `specifier.is_shorthand()`.

```rust
// Before
let specifier = ImportNamedSpecifier {
    span,
    local,
    imported: None,
    is_type_only,
};

// After
let imported = ModuleExportName::Ident(local.clone_in(allocator));
let specifier = ImportNamedSpecifier {
    span,
    local,
    imported,
    is_type_only,
};
```

## 8. `ExportNamedSpecifier.exported` is required

Status: `Landed`.

`ExportNamedSpecifier.exported` changed from `Option<ModuleExportName>` to
`ModuleExportName`.

For shorthand exports such as `export { foo }`, `exported` stores the same name
as `orig`. Use `ExportNamedSpecifier::is_shorthand()` to detect the shorthand
form.

- Replace `exported: Some(name)` with `exported: name`.
- Replace `exported: None` with a `ModuleExportName` matching `orig`.
- Replace `specifier.exported.is_none()` checks with
  `specifier.is_shorthand()`.

```rust
// Before
let specifier = ExportNamedSpecifier {
    span,
    orig,
    exported: None,
    is_type_only,
};

// After
let exported = orig.clone_in(allocator);
let specifier = ExportNamedSpecifier {
    span,
    orig,
    exported,
    is_type_only,
};
```
