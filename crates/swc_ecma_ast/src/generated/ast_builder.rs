use crate::{Ast, AstNode, ExtraData, NodeData, NodeKind, ast::*, node_id::*};
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
    ) -> TypedNodeId<Module> {
        let _f0 = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: shebang.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Module,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn script(
        &mut self,
        span: Span,
        body: TypedSubRange<Stmt>,
        shebang: OptionalAtomRef,
    ) -> TypedNodeId<Script> {
        let _f0 = self.add_extra(ExtraData {
            sub_range: body.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: shebang.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Script,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn module_item_module_decl_import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: TypedNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::Import(
            self.import_decl(span, specifiers, src, type_only, with)
                .into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_decl(
        &mut self,
        span: Span,
        decl: TypedNodeId<Decl>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(self.export_decl(span, decl).into()))
    }
    #[inline]
    pub fn module_item_module_decl_named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: TypedOptionalNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
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
        decl: TypedNodeId<DefaultDecl>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(
            self.export_default_decl(span, decl).into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_default_expr(
        &mut self,
        span: Span,
        expr: TypedNodeId<Expr>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(
            self.export_default_expr(span, expr).into(),
        ))
    }
    #[inline]
    pub fn module_item_module_decl_export_all(
        &mut self,
        span: Span,
        src: TypedNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::ExportAll(
            self.export_all(span, src, type_only, with).into(),
        ))
    }
    #[inline]
    pub fn module_decl_import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: TypedNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::Import(
            self.import_decl(span, specifiers, src, type_only, with)
                .into(),
        )
    }
    #[inline]
    pub fn module_decl_export_decl(&mut self, span: Span, decl: TypedNodeId<Decl>) -> ModuleDecl {
        ModuleDecl::ExportDecl(self.export_decl(span, decl).into())
    }
    #[inline]
    pub fn module_decl_named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: TypedOptionalNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::ExportNamed(
            self.named_export(span, specifiers, src, type_only, with)
                .into(),
        )
    }
    #[inline]
    pub fn module_decl_export_default_decl(
        &mut self,
        span: Span,
        decl: TypedNodeId<DefaultDecl>,
    ) -> ModuleDecl {
        ModuleDecl::ExportDefaultDecl(self.export_default_decl(span, decl).into())
    }
    #[inline]
    pub fn module_decl_export_default_expr(
        &mut self,
        span: Span,
        expr: TypedNodeId<Expr>,
    ) -> ModuleDecl {
        ModuleDecl::ExportDefaultExpr(self.export_default_expr(span, expr).into())
    }
    #[inline]
    pub fn module_decl_export_all(
        &mut self,
        span: Span,
        src: TypedNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> ModuleDecl {
        ModuleDecl::ExportAll(self.export_all(span, src, type_only, with).into())
    }
    #[inline]
    pub fn import_decl(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ImportSpecifier>,
        src: TypedNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> TypedNodeId<ImportDecl> {
        let _f0 = self.add_extra(ExtraData {
            sub_range: specifiers.into(),
        });
        let _f1 = self.add_extra(ExtraData { node: src.into() });
        let _f2 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            optional_node: with.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ImportDecl,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn import_specifier_import_named_specifier(
        &mut self,
        span: Span,
        local: TypedNodeId<Ident>,
        imported: TypedOptionalNodeId<ModuleExportName>,
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
        local: TypedNodeId<Ident>,
    ) -> ImportSpecifier {
        ImportSpecifier::Default(self.import_default_specifier(span, local).into())
    }
    #[inline]
    pub fn import_specifier_import_star_as_specifier(
        &mut self,
        span: Span,
        local: TypedNodeId<Ident>,
    ) -> ImportSpecifier {
        ImportSpecifier::Namespace(self.import_star_as_specifier(span, local).into())
    }
    #[inline]
    pub fn import_named_specifier(
        &mut self,
        span: Span,
        local: TypedNodeId<Ident>,
        imported: TypedOptionalNodeId<ModuleExportName>,
        is_type_only: bool,
    ) -> TypedNodeId<ImportNamedSpecifier> {
        let _f0 = self.add_extra(ExtraData { node: local.into() });
        let _f1 = self.add_extra(ExtraData {
            optional_node: imported.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_type_only.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ImportNamedSpecifier,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn import_default_specifier(
        &mut self,
        span: Span,
        local: TypedNodeId<Ident>,
    ) -> TypedNodeId<ImportDefaultSpecifier> {
        let _f0 = self.add_extra(ExtraData { node: local.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ImportDefaultSpecifier,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn import_star_as_specifier(
        &mut self,
        span: Span,
        local: TypedNodeId<Ident>,
    ) -> TypedNodeId<ImportStarAsSpecifier> {
        let _f0 = self.add_extra(ExtraData { node: local.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ImportStarAsSpecifier,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn export_decl(&mut self, span: Span, decl: TypedNodeId<Decl>) -> TypedNodeId<ExportDecl> {
        let _f0 = self.add_extra(ExtraData { node: decl.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ExportDecl,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn named_export(
        &mut self,
        span: Span,
        specifiers: TypedSubRange<ExportSpecifier>,
        src: TypedOptionalNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> TypedNodeId<NamedExport> {
        let _f0 = self.add_extra(ExtraData {
            sub_range: specifiers.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_node: src.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f3 = self.add_extra(ExtraData {
            optional_node: with.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::NamedExport,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn export_specifier_export_namespace_specifier(
        &mut self,
        span: Span,
        name: TypedNodeId<ModuleExportName>,
    ) -> ExportSpecifier {
        ExportSpecifier::Namespace(self.export_namespace_specifier(span, name).into())
    }
    #[inline]
    pub fn export_specifier_export_default_specifier(
        &mut self,
        span: Span,
        exported: TypedNodeId<Ident>,
    ) -> ExportSpecifier {
        ExportSpecifier::Default(self.export_default_specifier(span, exported).into())
    }
    #[inline]
    pub fn export_specifier_export_named_specifier(
        &mut self,
        span: Span,
        orig: TypedNodeId<ModuleExportName>,
        exported: TypedOptionalNodeId<ModuleExportName>,
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
        name: TypedNodeId<ModuleExportName>,
    ) -> TypedNodeId<ExportNamespaceSpecifier> {
        let _f0 = self.add_extra(ExtraData { node: name.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ExportNamespaceSpecifier,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
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
        exported: TypedNodeId<Ident>,
    ) -> TypedNodeId<ExportDefaultSpecifier> {
        let _f0 = self.add_extra(ExtraData {
            node: exported.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ExportDefaultSpecifier,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn export_named_specifier(
        &mut self,
        span: Span,
        orig: TypedNodeId<ModuleExportName>,
        exported: TypedOptionalNodeId<ModuleExportName>,
        is_type_only: bool,
    ) -> TypedNodeId<ExportNamedSpecifier> {
        let _f0 = self.add_extra(ExtraData { node: orig.into() });
        let _f1 = self.add_extra(ExtraData {
            optional_node: exported.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            bool: is_type_only.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ExportNamedSpecifier,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn export_default_decl(
        &mut self,
        span: Span,
        decl: TypedNodeId<DefaultDecl>,
    ) -> TypedNodeId<ExportDefaultDecl> {
        let _f0 = self.add_extra(ExtraData { node: decl.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ExportDefaultDecl,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
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
    pub fn export_default_expr(
        &mut self,
        span: Span,
        expr: TypedNodeId<Expr>,
    ) -> TypedNodeId<ExportDefaultExpr> {
        let _f0 = self.add_extra(ExtraData { node: expr.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ExportDefaultExpr,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn export_all(
        &mut self,
        span: Span,
        src: TypedNodeId<Str>,
        type_only: bool,
        with: TypedOptionalNodeId<ObjectLit>,
    ) -> TypedNodeId<ExportAll> {
        let _f0 = self.add_extra(ExtraData { node: src.into() });
        let _f1 = self.add_extra(ExtraData {
            bool: type_only.into(),
        });
        let _f2 = self.add_extra(ExtraData {
            optional_node: with.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ExportAll,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
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
    pub fn expr_lit_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Expr {
        Expr::Lit(Lit::Num(self.number(span, value, raw).into()))
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
    pub fn object_lit(&mut self, span: Span) -> TypedNodeId<ObjectLit> {
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ObjectLit,
                data: NodeData { empty: () },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn class_expr(&mut self, span: Span) -> TypedNodeId<ClassExpr> {
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::ClassExpr,
                data: NodeData { empty: () },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn fn_expr(&mut self, span: Span) -> TypedNodeId<FnExpr> {
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::FnExpr,
                data: NodeData { empty: () },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn ident(&mut self, span: Span, sym: AtomRef, optional: bool) -> TypedNodeId<Ident> {
        let _f0 = self.add_extra(ExtraData { atom: sym.into() });
        let _f1 = self.add_extra(ExtraData {
            bool: optional.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Ident,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn ident_name(&mut self, span: Span, sym: AtomRef) -> TypedNodeId<IdentName> {
        let _f0 = self.add_extra(ExtraData { atom: sym.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::IdentName,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn private_name(&mut self, span: Span, name: AtomRef) -> TypedNodeId<PrivateName> {
        let _f0 = self.add_extra(ExtraData { atom: name.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::PrivateName,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn binding_ident(
        &mut self,
        span: Span,
        id: TypedNodeId<Ident>,
    ) -> TypedNodeId<BindingIdent> {
        let _f0 = self.add_extra(ExtraData { node: id.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::BindingIdent,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
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
    pub fn lit_number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> Lit {
        Lit::Num(self.number(span, value, raw).into())
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
    pub fn str(&mut self, span: Span, value: AtomRef, raw: OptionalAtomRef) -> TypedNodeId<Str> {
        let _f0 = self.add_extra(ExtraData { atom: value.into() });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Str,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn bool(&mut self, span: Span, value: bool) -> TypedNodeId<Bool> {
        let _f0 = self.add_extra(ExtraData { bool: value.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Bool,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn null(&mut self, span: Span) -> TypedNodeId<Null> {
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Null,
                data: NodeData { empty: () },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn number(&mut self, span: Span, value: f64, raw: OptionalAtomRef) -> TypedNodeId<Number> {
        let _f0 = self.add_extra(ExtraData {
            number: value.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Number,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn big_int(
        &mut self,
        span: Span,
        value: BigIntId,
        raw: OptionalAtomRef,
    ) -> TypedNodeId<BigInt> {
        let _f0 = self.add_extra(ExtraData {
            bigint: value.into(),
        });
        let _f1 = self.add_extra(ExtraData {
            optional_atom: raw.into(),
        });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::BigInt,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn regex(&mut self, span: Span, exp: AtomRef, flags: AtomRef) -> TypedNodeId<Regex> {
        let _f0 = self.add_extra(ExtraData { atom: exp.into() });
        let _f1 = self.add_extra(ExtraData { atom: flags.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::Regex,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
    #[inline]
    pub fn jsx_text(&mut self, span: Span, value: AtomRef, raw: AtomRef) -> TypedNodeId<JSXText> {
        let _f0 = self.add_extra(ExtraData { atom: value.into() });
        let _f1 = self.add_extra(ExtraData { atom: raw.into() });
        unsafe {
            self.add_node(AstNode {
                span,
                kind: NodeKind::JSXText,
                data: NodeData {
                    extra_data_start: _f0,
                },
            })
            .cast_to_typed()
        }
    }
}
