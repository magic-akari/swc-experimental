use crate::{ast::*, node_id::*};
impl Module {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> TypedSubRange<ModuleItem> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn shebang(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: TypedSubRange<ModuleItem>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = body.into();
    }
    #[inline]
    pub fn set_shebang(&self, ast: &mut crate::Ast, shebang: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = shebang.into();
    }
}
impl Script {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> TypedSubRange<Stmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn shebang(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: TypedSubRange<Stmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = body.into();
    }
    #[inline]
    pub fn set_shebang(&self, ast: &mut crate::Ast, shebang: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = shebang.into();
    }
}
impl ImportDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn specifiers(&self, ast: &crate::Ast) -> TypedSubRange<ImportSpecifier> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn src(&self, ast: &crate::Ast) -> Str {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Str::from_node_id(ret, ast)
    }
    #[inline]
    pub fn type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn with(&self, ast: &crate::Ast) -> Option<ObjectLit> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ObjectLit::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_specifiers(&self, ast: &mut crate::Ast, specifiers: TypedSubRange<ImportSpecifier>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = specifiers.into();
    }
    #[inline]
    pub fn set_src(&self, ast: &mut crate::Ast, src: Str) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = src.node_id().into();
    }
    #[inline]
    pub fn set_type_only(&self, ast: &mut crate::Ast, type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = type_only.into();
    }
    #[inline]
    pub fn set_with(&self, ast: &mut crate::Ast, with: Option<ObjectLit>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].optional_node = with.optional_node_id().into();
    }
}
impl ImportNamedSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn local(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn imported(&self, ast: &crate::Ast) -> Option<ModuleExportName> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ModuleExportName::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_local(&self, ast: &mut crate::Ast, local: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = local.node_id().into();
    }
    #[inline]
    pub fn set_imported(&self, ast: &mut crate::Ast, imported: Option<ModuleExportName>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = imported.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_type_only(&self, ast: &mut crate::Ast, is_type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_type_only.into();
    }
}
impl ImportDefaultSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn local(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_local(&self, ast: &mut crate::Ast, local: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = local.node_id().into();
    }
}
impl ImportStarAsSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn local(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_local(&self, ast: &mut crate::Ast, local: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = local.node_id().into();
    }
}
impl ExportDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn decl(&self, ast: &crate::Ast) -> Decl {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Decl::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_decl(&self, ast: &mut crate::Ast, decl: Decl) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = decl.node_id().into();
    }
}
impl NamedExport {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn specifiers(&self, ast: &crate::Ast) -> TypedSubRange<ExportSpecifier> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn src(&self, ast: &crate::Ast) -> Option<Str> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Str::from_node_id(id, ast))
    }
    #[inline]
    pub fn type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn with(&self, ast: &crate::Ast) -> Option<ObjectLit> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ObjectLit::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_specifiers(&self, ast: &mut crate::Ast, specifiers: TypedSubRange<ExportSpecifier>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = specifiers.into();
    }
    #[inline]
    pub fn set_src(&self, ast: &mut crate::Ast, src: Option<Str>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = src.optional_node_id().into();
    }
    #[inline]
    pub fn set_type_only(&self, ast: &mut crate::Ast, type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = type_only.into();
    }
    #[inline]
    pub fn set_with(&self, ast: &mut crate::Ast, with: Option<ObjectLit>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].optional_node = with.optional_node_id().into();
    }
}
impl ExportNamespaceSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn name(&self, ast: &crate::Ast) -> ModuleExportName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ModuleExportName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_name(&self, ast: &mut crate::Ast, name: ModuleExportName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = name.node_id().into();
    }
}
impl ExportDefaultSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exported(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exported(&self, ast: &mut crate::Ast, exported: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = exported.node_id().into();
    }
}
impl ExportNamedSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn orig(&self, ast: &crate::Ast) -> ModuleExportName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ModuleExportName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn exported(&self, ast: &crate::Ast) -> Option<ModuleExportName> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ModuleExportName::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_orig(&self, ast: &mut crate::Ast, orig: ModuleExportName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = orig.node_id().into();
    }
    #[inline]
    pub fn set_exported(&self, ast: &mut crate::Ast, exported: Option<ModuleExportName>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = exported.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_type_only(&self, ast: &mut crate::Ast, is_type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_type_only.into();
    }
}
impl ExportDefaultDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn decl(&self, ast: &crate::Ast) -> DefaultDecl {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        DefaultDecl::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_decl(&self, ast: &mut crate::Ast, decl: DefaultDecl) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = decl.node_id().into();
    }
}
impl ExportDefaultExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn expr(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_expr(&self, ast: &mut crate::Ast, expr: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = expr.node_id().into();
    }
}
impl ExportAll {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn src(&self, ast: &crate::Ast) -> Str {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Str::from_node_id(ret, ast)
    }
    #[inline]
    pub fn type_only(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn with(&self, ast: &crate::Ast) -> Option<ObjectLit> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| ObjectLit::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_src(&self, ast: &mut crate::Ast, src: Str) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = src.node_id().into();
    }
    #[inline]
    pub fn set_type_only(&self, ast: &mut crate::Ast, type_only: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = type_only.into();
    }
    #[inline]
    pub fn set_with(&self, ast: &mut crate::Ast, with: Option<ObjectLit>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = with.optional_node_id().into();
    }
}
impl EmptyStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl ClassDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn ident(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn declare(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_ident(&self, ast: &mut crate::Ast, ident: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = ident.node_id().into();
    }
    #[inline]
    pub fn set_declare(&self, ast: &mut crate::Ast, declare: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = declare.into();
    }
}
impl ObjectLit {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl ClassExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl FnExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl Ident {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn sym(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn optional(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_sym(&self, ast: &mut crate::Ast, sym: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = sym.into();
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = optional.into();
    }
}
impl IdentName {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn sym(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_sym(&self, ast: &mut crate::Ast, sym: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = sym.into();
    }
}
impl PrivateName {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn name(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_name(&self, ast: &mut crate::Ast, name: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = name.into();
    }
}
impl BindingIdent {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn id(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_id(&self, ast: &mut crate::Ast, id: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = id.node_id().into();
    }
}
impl Str {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = value.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = raw.into();
    }
}
impl Bool {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = value.into();
    }
}
impl Null {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl Num {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> f64 {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].number };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: f64) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].number = value.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = raw.into();
    }
}
impl BigInt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> BigIntId {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bigint };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> OptionalAtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: BigIntId) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bigint = value.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = raw.into();
    }
}
impl Regex {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exp(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn flags(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exp(&self, ast: &mut crate::Ast, exp: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = exp.into();
    }
    #[inline]
    pub fn set_flags(&self, ast: &mut crate::Ast, flags: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].atom = flags.into();
    }
}
impl JSXText {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].atom = value.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].atom = raw.into();
    }
}
