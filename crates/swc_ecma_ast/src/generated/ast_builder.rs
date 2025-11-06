use crate::{ast::*, node_id::*, Ast, AstNode, ExtraData, NodeData, NodeKind};
use swc_common::Span;
impl Ast {
    #[inline]
    pub fn program_module(
        &mut self,
        span: Span,
        body: TypedSubRange<ModuleItem>,
        shebang: OptionalAtomRef,
    ) -> Program {
        Program::Module(self.module(span, body, shebang).into())
    }
    #[inline]
    pub fn program_script(
        &mut self,
        span: Span,
        body: TypedSubRange<Stmt>,
        shebang: OptionalAtomRef,
    ) -> Program {
        Program::Script(self.script(span, body, shebang).into())
    }
    #[inline]
    pub fn module(
        &mut self,
        span: Span,
        body: TypedSubRange<ModuleItem>,
        shebang: OptionalAtomRef,
    ) -> Module {
        let _f0 = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: shebang.into(),
        });
        Module(self.add_node(AstNode {
            span,
            kind: NodeKind::Module,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn script(
        &mut self,
        span: Span,
        body: TypedSubRange<Stmt>,
        shebang: OptionalAtomRef,
    ) -> Script {
        let _f0 = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: shebang.into(),
        });
        Script(self.add_node(AstNode {
            span,
            kind: NodeKind::Script,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn module_item_module_decl_import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::Import(
            self.import_decl(span, specifiers, src, type_only, with)
                .into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_decl(&mut self, span: Span, decl: Decl) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(self.export_decl(span, decl).into()))
    }
    #[inline]
    pub fn module_item_module_decl_named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: Option<Str>,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(
            self.named_export(span, specifiers, src, type_only, with)
                .into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_default_decl(
        &mut self,
        span: Span,
        decl: DefaultDecl,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(
            self.export_default_decl(span, decl).into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_default_expr(
        &mut self,
        span: Span,
        expr: Expr,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
            self.export_default_expr(span, expr).into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_all(
        &mut self,
        span: Span,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportAll(
            self.export_all(span, src, type_only, with).into(),
        ))
    }
    #[inline]
    pub fn module_item_stmt_empty_stmt(&mut self, span: Span) -> ModuleItem {
        ModuleItem::Stmt(Stmt::Empty(self.empty_stmt(span).into()))
    }
    #[inline]
    pub fn module_decl_import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::Import(
            self.import_decl(span, specifiers, src, type_only, with)
                .into(),
        )
    }
    #[inline]
    pub fn module_decl_export_decl(&mut self, span: Span, decl: Decl) -> ModuleDecl {
        ModuleDecl::ExportDecl(self.export_decl(span, decl).into())
    }
    #[inline]
    pub fn module_decl_named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: Option<Str>,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::ExportNamed(
            self.named_export(span, specifiers, src, type_only, with)
                .into(),
        )
    }
    #[inline]
    pub fn module_decl_export_default_decl(&mut self, span: Span, decl: DefaultDecl) -> ModuleDecl {
        ModuleDecl::ExportDefaultDecl(self.export_default_decl(span, decl).into())
    }
    #[inline]
    pub fn module_decl_export_default_expr(&mut self, span: Span, expr: Expr) -> ModuleDecl {
        ModuleDecl::ExportDefaultExpr(self.export_default_expr(span, expr).into())
    }
    #[inline]
    pub fn module_decl_export_all(
        &mut self,
        span: Span,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::ExportAll(self.export_all(span, src, type_only, with).into())
    }
    #[inline]
    pub fn import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ImportDecl {
        let _f0 = self.add_extra(ExtraData {
            sub_range: specifiers.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            node: src.node_id().into(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            optional_node: with.optional_node_id().into(),
        });
        ImportDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn import_specifier_import_named_specifier(
        &mut self,
        span: Span,
        local: Ident,
        imported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ImportSpecifier {
        ImportSpecifier::Named(
            self.import_named_specifier(span, local, imported, is_type_only)
                .into(),
        )
    }
    #[inline]
    pub fn import_specifier_import_default_specifier(
        &mut self,
        span: Span,
        local: Ident,
    ) -> ImportSpecifier {
        ImportSpecifier::Default(self.import_default_specifier(span, local).into())
    }
    #[inline]
    pub fn import_specifier_import_star_as_specifier(
        &mut self,
        span: Span,
        local: Ident,
    ) -> ImportSpecifier {
        ImportSpecifier::Namespace(self.import_star_as_specifier(span, local).into())
    }
    #[inline]
    pub fn import_named_specifier(
        &mut self,
        span: Span,
        local: Ident,
        imported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ImportNamedSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: local.node_id().into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: imported.optional_node_id().into(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_type_only.into(),
        });
        ImportNamedSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportNamedSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn import_default_specifier(&mut self, span: Span, local: Ident) -> ImportDefaultSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: local.node_id().into(),
        });
        ImportDefaultSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportDefaultSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn import_star_as_specifier(&mut self, span: Span, local: Ident) -> ImportStarAsSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: local.node_id().into(),
        });
        ImportStarAsSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ImportStarAsSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_decl(&mut self, span: Span, decl: Decl) -> ExportDecl {
        let _f0 = self.add_extra(ExtraData {
            node: decl.node_id().into(),
        });
        ExportDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: Option<Str>,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> NamedExport {
        let _f0 = self.add_extra(ExtraData {
            sub_range: specifiers.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: src.optional_node_id().into(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            optional_node: with.optional_node_id().into(),
        });
        NamedExport(self.add_node(AstNode {
            span,
            kind: NodeKind::NamedExport,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_specifier_export_namespace_specifier(
        &mut self,
        span: Span,
        name: ModuleExportName,
    ) -> ExportSpecifier {
        ExportSpecifier::Namespace(self.export_namespace_specifier(span, name).into())
    }
    #[inline]
    pub fn export_specifier_export_default_specifier(
        &mut self,
        span: Span,
        exported: Ident,
    ) -> ExportSpecifier {
        ExportSpecifier::Default(self.export_default_specifier(span, exported).into())
    }
    #[inline]
    pub fn export_specifier_export_named_specifier(
        &mut self,
        span: Span,
        orig: ModuleExportName,
        exported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ExportSpecifier {
        ExportSpecifier::Named(
            self.export_named_specifier(span, orig, exported, is_type_only)
                .into(),
        )
    }
    #[inline]
    pub fn export_namespace_specifier(
        &mut self,
        span: Span,
        name: ModuleExportName,
    ) -> ExportNamespaceSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: name.node_id().into(),
        });
        ExportNamespaceSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportNamespaceSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn module_export_name_ident(
        &mut self,
        span: Span,
        sym: AtomRef,
        optional: bool,
    ) -> ModuleExportName {
        ModuleExportName::Ident(self.ident(span, sym, optional).into())
    }
    #[inline]
    pub fn module_export_name_str(
        &mut self,
        span: Span,
        value: AtomRef,
        raw: OptionalAtomRef,
    ) -> ModuleExportName {
        ModuleExportName::Str(self.str(span, value, raw).into())
    }
    #[inline]
    pub fn export_default_specifier(
        &mut self,
        span: Span,
        exported: Ident,
    ) -> ExportDefaultSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: exported.node_id().into(),
        });
        ExportDefaultSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDefaultSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_named_specifier(
        &mut self,
        span: Span,
        orig: ModuleExportName,
        exported: Option<ModuleExportName>,
        is_type_only: bool,
    ) -> ExportNamedSpecifier {
        let _f0 = self.add_extra(ExtraData {
            node: orig.node_id().into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: exported.optional_node_id().into(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_type_only.into(),
        });
        ExportNamedSpecifier(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportNamedSpecifier,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_default_decl(&mut self, span: Span, decl: DefaultDecl) -> ExportDefaultDecl {
        let _f0 = self.add_extra(ExtraData {
            node: decl.node_id().into(),
        });
        ExportDefaultDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDefaultDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn default_decl_class_expr(&mut self, span: Span) -> DefaultDecl {
        DefaultDecl::Class(self.class_expr(span).into())
    }
    #[inline]
    pub fn default_decl_fn_expr(&mut self, span: Span) -> DefaultDecl {
        DefaultDecl::Fn(self.fn_expr(span).into())
    }
    #[inline]
    pub fn export_default_expr(&mut self, span: Span, expr: Expr) -> ExportDefaultExpr {
        let _f0 = self.add_extra(ExtraData {
            node: expr.node_id().into(),
        });
        ExportDefaultExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportDefaultExpr,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn export_all(
        &mut self,
        span: Span,
        src: Str,
        type_only: bool,
        with: Option<ObjectLit>,
    ) -> ExportAll {
        let _f0 = self.add_extra(ExtraData {
            node: src.node_id().into(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: with.optional_node_id().into(),
        });
        ExportAll(self.add_node(AstNode {
            span,
            kind: NodeKind::ExportAll,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn stmt_empty_stmt(&mut self, span: Span) -> Stmt {
        Stmt::Empty(self.empty_stmt(span).into())
    }
    #[inline]
    pub fn empty_stmt(&mut self, span: Span) -> EmptyStmt {
        EmptyStmt(self.add_node(AstNode {
            span,
            kind: NodeKind::EmptyStmt,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn decl_class_decl(&mut self, span: Span, ident: Ident, declare: bool) -> Decl {
        Decl::Class(self.class_decl(span, ident, declare).into())
    }
    #[inline]
    pub fn class_decl(&mut self, span: Span, ident: Ident, declare: bool) -> ClassDecl {
        let _f0 = self.add_extra(ExtraData {
            node: ident.node_id().into(),
        });
        let _f1 = self.add_extra(ExtraData {
            bool: declare.into(),
        });
        ClassDecl(self.add_node(AstNode {
            span,
            kind: NodeKind::ClassDecl,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn expr_lit_str(&mut self, span: Span, value: AtomRef, raw: OptionalAtomRef) -> Expr {
        Expr::Lit(Lit::Str(self.str(span, value, raw).into()))
    }
    #[inline]
    pub fn expr_lit_bool(&mut self, span: Span, value: bool) -> Expr {
        Expr::Lit(Lit::Bool(self.bool(span, value).into()))
    }
    #[inline]
    pub fn expr_lit_null(&mut self, span: Span) -> Expr {
        Expr::Lit(Lit::Null(self.null(span).into()))
    }
    #[inline]
    pub fn expr_lit_num(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Expr {
        Expr::Lit(Lit::Num(self.num(span, value, raw).into()))
    }
    #[inline]
    pub fn expr_lit_big_int(&mut self, span: Span, value: BigIntId, raw: OptionalAtomRef) -> Expr {
        Expr::Lit(Lit::BigInt(self.big_int(span, value, raw).into()))
    }
    #[inline]
    pub fn expr_lit_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Expr {
        Expr::Lit(Lit::Regex(self.regex(span, exp, flags).into()))
    }
    #[inline]
    pub fn expr_lit_jsx_text(&mut self, span: Span, value: AtomRef, raw: AtomRef) -> Expr {
        Expr::Lit(Lit::JSXText(self.jsx_text(span, value, raw).into()))
    }
    #[inline]
    pub fn object_lit(&mut self, span: Span) -> ObjectLit {
        ObjectLit(self.add_node(AstNode {
            span,
            kind: NodeKind::ObjectLit,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn class_expr(&mut self, span: Span) -> ClassExpr {
        ClassExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::ClassExpr,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn fn_expr(&mut self, span: Span) -> FnExpr {
        FnExpr(self.add_node(AstNode {
            span,
            kind: NodeKind::FnExpr,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> Ident {
        let _f0 = self.add_extra(ExtraData { atom: sym.into() });
        let _f1 = self.add_extra(ExtraData {
            bool: optional.into(),
        });
        Ident(self.add_node(AstNode {
            span,
            kind: NodeKind::Ident,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn ident_name(&mut self, span: Span, sym: AtomRef) -> IdentName {
        let _f0 = self.add_extra(ExtraData { atom: sym.into() });
        IdentName(self.add_node(AstNode {
            span,
            kind: NodeKind::IdentName,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn private_name(&mut self, span: Span, name: AtomRef) -> PrivateName {
        let _f0 = self.add_extra(ExtraData { atom: name.into() });
        PrivateName(self.add_node(AstNode {
            span,
            kind: NodeKind::PrivateName,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn binding_ident(&mut self, span: Span, id: Ident) -> BindingIdent {
        let _f0 = self.add_extra(ExtraData {
            node: id.node_id().into(),
        });
        BindingIdent(self.add_node(AstNode {
            span,
            kind: NodeKind::BindingIdent,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn lit_str(&mut self, span: Span, value: AtomRef, raw: OptionalAtomRef) -> Lit {
        Lit::Str(self.str(span, value, raw).into())
    }
    #[inline]
    pub fn lit_bool(&mut self, span: Span, value: bool) -> Lit {
        Lit::Bool(self.bool(span, value).into())
    }
    #[inline]
    pub fn lit_null(&mut self, span: Span) -> Lit {
        Lit::Null(self.null(span).into())
    }
    #[inline]
    pub fn lit_num(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Lit {
        Lit::Num(self.num(span, value, raw).into())
    }
    #[inline]
    pub fn lit_big_int(&mut self, span: Span, value: BigIntId, raw: OptionalAtomRef) -> Lit {
        Lit::BigInt(self.big_int(span, value, raw).into())
    }
    #[inline]
    pub fn lit_regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Lit {
        Lit::Regex(self.regex(span, exp, flags).into())
    }
    #[inline]
    pub fn lit_jsx_text(&mut self, span: Span, value: AtomRef, raw: AtomRef) -> Lit {
        Lit::JSXText(self.jsx_text(span, value, raw).into())
    }
    #[inline]
    pub fn str(&mut self, span: Span, value: AtomRef, raw: OptionalAtomRef) -> Str {
        let _f0 = self.add_extra(ExtraData { atom: value.into() });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        Str(self.add_node(AstNode {
            span,
            kind: NodeKind::Str,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn bool(&mut self, span: Span, value: bool) -> Bool {
        let _f0 = self.add_extra(ExtraData { bool: value.into() });
        Bool(self.add_node(AstNode {
            span,
            kind: NodeKind::Bool,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn null(&mut self, span: Span) -> Null {
        Null(self.add_node(AstNode {
            span,
            kind: NodeKind::Null,
            data: NodeData { empty: () },
        }))
    }
    #[inline]
    pub fn num(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Num {
        let _f0 = self.add_extra(ExtraData {
            number: value.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        Num(self.add_node(AstNode {
            span,
            kind: NodeKind::Num,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn big_int(&mut self, span: Span, value: BigIntId, raw: OptionalAtomRef) -> BigInt {
        let _f0 = self.add_extra(ExtraData {
            bigint: value.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        BigInt(self.add_node(AstNode {
            span,
            kind: NodeKind::BigInt,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> Regex {
        let _f0 = self.add_extra(ExtraData { atom: exp.into() });
        let _f1 = self.add_extra(ExtraData { atom: flags.into() });
        Regex(self.add_node(AstNode {
            span,
            kind: NodeKind::Regex,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
    #[inline]
    pub fn jsx_text(&mut self, span: Span, value: AtomRef, raw: AtomRef) -> JSXText {
        let _f0 = self.add_extra(ExtraData { atom: value.into() });
        let _f1 = self.add_extra(ExtraData { atom: raw.into() });
        JSXText(self.add_node(AstNode {
            span,
            kind: NodeKind::JSXText,
            data: NodeData {
                extra_data_start: _f0,
            },
        }))
    }
}
