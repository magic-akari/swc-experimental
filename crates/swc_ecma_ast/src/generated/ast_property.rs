#![allow(unused)]
use crate::{ast::*, node_id::*};
impl Program {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Module(it) => it.span(ast),
            Self::Script(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Module(it) => it.set_span(ast, span),
            Self::Script(it) => it.set_span(ast, span),
        }
    }
}
impl Module {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> ModuleItem {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ModuleItem::from_node_id(ret, ast)
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
    pub fn set_body(&self, ast: &mut crate::Ast, body: ModuleItem) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = body.node_id().into();
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
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
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
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
    #[inline]
    pub fn set_shebang(&self, ast: &mut crate::Ast, shebang: OptionalAtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_atom = shebang.into();
    }
}
impl ModuleItem {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::ModuleDecl(it) => it.span(ast),
            Self::Stmt(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::ModuleDecl(it) => it.set_span(ast, span),
            Self::Stmt(it) => it.set_span(ast, span),
        }
    }
}
impl ModuleDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Import(it) => it.span(ast),
            Self::ExportDecl(it) => it.span(ast),
            Self::ExportNamed(it) => it.span(ast),
            Self::ExportDefaultDecl(it) => it.span(ast),
            Self::ExportDefaultExpr(it) => it.span(ast),
            Self::ExportAll(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Import(it) => it.set_span(ast, span),
            Self::ExportDecl(it) => it.set_span(ast, span),
            Self::ExportNamed(it) => it.set_span(ast, span),
            Self::ExportDefaultDecl(it) => it.set_span(ast, span),
            Self::ExportDefaultExpr(it) => it.set_span(ast, span),
            Self::ExportAll(it) => it.set_span(ast, span),
        }
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
    pub fn phase(&self, ast: &crate::Ast) -> ImportPhase {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 4usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ImportPhase::from_extra_data(ret)
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
    #[inline]
    pub fn set_phase(&self, ast: &mut crate::Ast, phase: ImportPhase) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 4usize;
        ast.extra_data[offset].other = phase.to_extra_data();
    }
}
impl ImportSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Named(it) => it.span(ast),
            Self::Default(it) => it.span(ast),
            Self::Namespace(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Named(it) => it.set_span(ast, span),
            Self::Default(it) => it.set_span(ast, span),
            Self::Namespace(it) => it.set_span(ast, span),
        }
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
impl ExportSpecifier {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Namespace(it) => it.span(ast),
            Self::Default(it) => it.span(ast),
            Self::Named(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Namespace(it) => it.set_span(ast, span),
            Self::Default(it) => it.set_span(ast, span),
            Self::Named(it) => it.set_span(ast, span),
        }
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
impl ModuleExportName {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Ident(it) => it.span(ast),
            Self::Str(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Ident(it) => it.set_span(ast, span),
            Self::Str(it) => it.set_span(ast, span),
        }
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
impl DefaultDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Class(it) => it.span(ast),
            Self::Fn(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Class(it) => it.set_span(ast, span),
            Self::Fn(it) => it.set_span(ast, span),
        }
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
impl BlockStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn stmts(&self, ast: &crate::Ast) -> TypedSubRange<Stmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_stmts(&self, ast: &mut crate::Ast, stmts: TypedSubRange<Stmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = stmts.into();
    }
}
impl Stmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Block(it) => it.span(ast),
            Self::Empty(it) => it.span(ast),
            Self::Debugger(it) => it.span(ast),
            Self::With(it) => it.span(ast),
            Self::Return(it) => it.span(ast),
            Self::Labeled(it) => it.span(ast),
            Self::Break(it) => it.span(ast),
            Self::Continue(it) => it.span(ast),
            Self::If(it) => it.span(ast),
            Self::Switch(it) => it.span(ast),
            Self::Throw(it) => it.span(ast),
            Self::Try(it) => it.span(ast),
            Self::While(it) => it.span(ast),
            Self::DoWhile(it) => it.span(ast),
            Self::For(it) => it.span(ast),
            Self::ForIn(it) => it.span(ast),
            Self::ForOf(it) => it.span(ast),
            Self::Decl(it) => it.span(ast),
            Self::Expr(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Block(it) => it.set_span(ast, span),
            Self::Empty(it) => it.set_span(ast, span),
            Self::Debugger(it) => it.set_span(ast, span),
            Self::With(it) => it.set_span(ast, span),
            Self::Return(it) => it.set_span(ast, span),
            Self::Labeled(it) => it.set_span(ast, span),
            Self::Break(it) => it.set_span(ast, span),
            Self::Continue(it) => it.set_span(ast, span),
            Self::If(it) => it.set_span(ast, span),
            Self::Switch(it) => it.set_span(ast, span),
            Self::Throw(it) => it.set_span(ast, span),
            Self::Try(it) => it.set_span(ast, span),
            Self::While(it) => it.set_span(ast, span),
            Self::DoWhile(it) => it.set_span(ast, span),
            Self::For(it) => it.set_span(ast, span),
            Self::ForIn(it) => it.set_span(ast, span),
            Self::ForOf(it) => it.set_span(ast, span),
            Self::Decl(it) => it.set_span(ast, span),
            Self::Expr(it) => it.set_span(ast, span),
        }
    }
}
impl ExprStmt {
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
impl DebuggerStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl WithStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn obj(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_obj(&self, ast: &mut crate::Ast, obj: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = obj.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ReturnStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = arg.optional_node_id().into();
    }
}
impl LabeledStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn label(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_label(&self, ast: &mut crate::Ast, label: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = label.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl BreakStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn label(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_label(&self, ast: &mut crate::Ast, label: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = label.optional_node_id().into();
    }
}
impl ContinueStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn label(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_label(&self, ast: &mut crate::Ast, label: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = label.optional_node_id().into();
    }
}
impl IfStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn cons(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn alt(&self, ast: &crate::Ast) -> Option<Stmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Stmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_cons(&self, ast: &mut crate::Ast, cons: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = cons.node_id().into();
    }
    #[inline]
    pub fn set_alt(&self, ast: &mut crate::Ast, alt: Option<Stmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = alt.optional_node_id().into();
    }
}
impl SwitchStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn discriminant(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn cases(&self, ast: &crate::Ast) -> TypedSubRange<SwitchCase> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_discriminant(&self, ast: &mut crate::Ast, discriminant: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = discriminant.node_id().into();
    }
    #[inline]
    pub fn set_cases(&self, ast: &mut crate::Ast, cases: TypedSubRange<SwitchCase>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = cases.into();
    }
}
impl ThrowStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl TryStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn block(&self, ast: &crate::Ast) -> BlockStmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn handler(&self, ast: &crate::Ast) -> Option<CatchClause> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| CatchClause::from_node_id(id, ast))
    }
    #[inline]
    pub fn finalizer(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_block(&self, ast: &mut crate::Ast, block: BlockStmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = block.node_id().into();
    }
    #[inline]
    pub fn set_handler(&self, ast: &mut crate::Ast, handler: Option<CatchClause>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = handler.optional_node_id().into();
    }
    #[inline]
    pub fn set_finalizer(&self, ast: &mut crate::Ast, finalizer: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = finalizer.optional_node_id().into();
    }
}
impl WhileStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl DoWhileStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ForStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn init(&self, ast: &crate::Ast) -> Option<VarDeclOrExpr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| VarDeclOrExpr::from_node_id(id, ast))
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn update(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_init(&self, ast: &mut crate::Ast, init: Option<VarDeclOrExpr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = init.optional_node_id().into();
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = test.optional_node_id().into();
    }
    #[inline]
    pub fn set_update(&self, ast: &mut crate::Ast, update: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = update.optional_node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ForInStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> ForHead {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ForHead::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: ForHead) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ForOfStmt {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn is_await(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> ForHead {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        ForHead::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Stmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Stmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_is_await(&self, ast: &mut crate::Ast, is_await: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = is_await.into();
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: ForHead) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Stmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl SwitchCase {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn cons(&self, ast: &crate::Ast) -> TypedSubRange<Stmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = test.optional_node_id().into();
    }
    #[inline]
    pub fn set_cons(&self, ast: &mut crate::Ast, cons: TypedSubRange<Stmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = cons.into();
    }
}
impl CatchClause {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn param(&self, ast: &crate::Ast) -> Option<Pat> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Pat::from_node_id(id, ast))
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> BlockStmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_param(&self, ast: &mut crate::Ast, param: Option<Pat>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = param.optional_node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: BlockStmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl ForHead {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::VarDecl(it) => it.span(ast),
            Self::UsingDecl(it) => it.span(ast),
            Self::Pat(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::VarDecl(it) => it.set_span(ast, span),
            Self::UsingDecl(it) => it.set_span(ast, span),
            Self::Pat(it) => it.set_span(ast, span),
        }
    }
}
impl VarDeclOrExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::VarDecl(it) => it.span(ast),
            Self::Expr(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::VarDecl(it) => it.set_span(ast, span),
            Self::Expr(it) => it.set_span(ast, span),
        }
    }
}
impl Decl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Class(it) => it.span(ast),
            Self::Fn(it) => it.span(ast),
            Self::Var(it) => it.span(ast),
            Self::Using(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Class(it) => it.set_span(ast, span),
            Self::Fn(it) => it.set_span(ast, span),
            Self::Var(it) => it.set_span(ast, span),
            Self::Using(it) => it.set_span(ast, span),
        }
    }
}
impl FnDecl {
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
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
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
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = function.node_id().into();
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
    pub fn class(&self, ast: &crate::Ast) -> Class {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Class::from_node_id(ret, ast)
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
    #[inline]
    pub fn set_class(&self, ast: &mut crate::Ast, class: Class) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = class.node_id().into();
    }
}
impl VarDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> VarDeclKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        VarDeclKind::from_extra_data(ret)
    }
    #[inline]
    pub fn declare(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn decls(&self, ast: &crate::Ast) -> TypedSubRange<VarDeclarator> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: VarDeclKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = kind.to_extra_data();
    }
    #[inline]
    pub fn set_declare(&self, ast: &mut crate::Ast, declare: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = declare.into();
    }
    #[inline]
    pub fn set_decls(&self, ast: &mut crate::Ast, decls: TypedSubRange<VarDeclarator>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].sub_range = decls.into();
    }
}
impl VarDeclarator {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn name(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn init(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_name(&self, ast: &mut crate::Ast, name: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = name.node_id().into();
    }
    #[inline]
    pub fn set_init(&self, ast: &mut crate::Ast, init: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = init.optional_node_id().into();
    }
}
impl UsingDecl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn is_await(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn decls(&self, ast: &crate::Ast) -> TypedSubRange<VarDeclarator> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_is_await(&self, ast: &mut crate::Ast, is_await: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = is_await.into();
    }
    #[inline]
    pub fn set_decls(&self, ast: &mut crate::Ast, decls: TypedSubRange<VarDeclarator>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = decls.into();
    }
}
impl Expr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::This(it) => it.span(ast),
            Self::Array(it) => it.span(ast),
            Self::Object(it) => it.span(ast),
            Self::Fn(it) => it.span(ast),
            Self::Unary(it) => it.span(ast),
            Self::Update(it) => it.span(ast),
            Self::Bin(it) => it.span(ast),
            Self::Assign(it) => it.span(ast),
            Self::Member(it) => it.span(ast),
            Self::SuperProp(it) => it.span(ast),
            Self::Cond(it) => it.span(ast),
            Self::Call(it) => it.span(ast),
            Self::New(it) => it.span(ast),
            Self::Seq(it) => it.span(ast),
            Self::Ident(it) => it.span(ast),
            Self::Lit(it) => it.span(ast),
            Self::Tpl(it) => it.span(ast),
            Self::TaggedTpl(it) => it.span(ast),
            Self::Arrow(it) => it.span(ast),
            Self::Class(it) => it.span(ast),
            Self::Yield(it) => it.span(ast),
            Self::MetaProp(it) => it.span(ast),
            Self::Await(it) => it.span(ast),
            Self::Paren(it) => it.span(ast),
            Self::PrivateName(it) => it.span(ast),
            Self::OptChain(it) => it.span(ast),
            Self::Invalid(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::This(it) => it.set_span(ast, span),
            Self::Array(it) => it.set_span(ast, span),
            Self::Object(it) => it.set_span(ast, span),
            Self::Fn(it) => it.set_span(ast, span),
            Self::Unary(it) => it.set_span(ast, span),
            Self::Update(it) => it.set_span(ast, span),
            Self::Bin(it) => it.set_span(ast, span),
            Self::Assign(it) => it.set_span(ast, span),
            Self::Member(it) => it.set_span(ast, span),
            Self::SuperProp(it) => it.set_span(ast, span),
            Self::Cond(it) => it.set_span(ast, span),
            Self::Call(it) => it.set_span(ast, span),
            Self::New(it) => it.set_span(ast, span),
            Self::Seq(it) => it.set_span(ast, span),
            Self::Ident(it) => it.set_span(ast, span),
            Self::Lit(it) => it.set_span(ast, span),
            Self::Tpl(it) => it.set_span(ast, span),
            Self::TaggedTpl(it) => it.set_span(ast, span),
            Self::Arrow(it) => it.set_span(ast, span),
            Self::Class(it) => it.set_span(ast, span),
            Self::Yield(it) => it.set_span(ast, span),
            Self::MetaProp(it) => it.set_span(ast, span),
            Self::Await(it) => it.set_span(ast, span),
            Self::Paren(it) => it.set_span(ast, span),
            Self::PrivateName(it) => it.set_span(ast, span),
            Self::OptChain(it) => it.set_span(ast, span),
            Self::Invalid(it) => it.set_span(ast, span),
        }
    }
}
impl ThisExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl ArrayLit {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn elems(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_elems(&self, ast: &mut crate::Ast, elems: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = elems.into();
    }
}
impl ObjectLit {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn props(&self, ast: &crate::Ast) -> TypedSubRange<PropOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_props(&self, ast: &mut crate::Ast, props: TypedSubRange<PropOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = props.into();
    }
}
impl PropOrSpread {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::SpreadElement(it) => it.span(ast),
            Self::Prop(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::SpreadElement(it) => it.set_span(ast, span),
            Self::Prop(it) => it.set_span(ast, span),
        }
    }
}
impl SpreadElement {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn dot_3_token(&self, ast: &crate::Ast) -> Span {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].span };
        ret.into()
    }
    #[inline]
    pub fn expr(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_dot3_token(&self, ast: &mut crate::Ast, dot3_token: Span) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].span = dot3_token.into();
    }
    #[inline]
    pub fn set_expr(&self, ast: &mut crate::Ast, expr: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = expr.node_id().into();
    }
}
impl UnaryExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> UnaryOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        UnaryOp::from_extra_data(ret)
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: UnaryOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.to_extra_data();
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl UpdateExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> UpdateOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        UpdateOp::from_extra_data(ret)
    }
    #[inline]
    pub fn prefix(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: UpdateOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.to_extra_data();
    }
    #[inline]
    pub fn set_prefix(&self, ast: &mut crate::Ast, prefix: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = prefix.into();
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl BinExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> BinaryOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        BinaryOp::from_extra_data(ret)
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: BinaryOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.to_extra_data();
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
}
impl FnExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn ident(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_ident(&self, ast: &mut crate::Ast, ident: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = ident.optional_node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
}
impl ClassExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn ident(&self, ast: &crate::Ast) -> Option<Ident> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Ident::from_node_id(id, ast))
    }
    #[inline]
    pub fn class(&self, ast: &crate::Ast) -> Class {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Class::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_ident(&self, ast: &mut crate::Ast, ident: Option<Ident>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = ident.optional_node_id().into();
    }
    #[inline]
    pub fn set_class(&self, ast: &mut crate::Ast, class: Class) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = class.node_id().into();
    }
}
impl AssignExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn op(&self, ast: &crate::Ast) -> AssignOp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        AssignOp::from_extra_data(ret)
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> AssignTarget {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        AssignTarget::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_op(&self, ast: &mut crate::Ast, op: AssignOp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = op.to_extra_data();
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: AssignTarget) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
}
impl MemberExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn obj(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn prop(&self, ast: &crate::Ast) -> MemberProp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        MemberProp::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_obj(&self, ast: &mut crate::Ast, obj: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = obj.node_id().into();
    }
    #[inline]
    pub fn set_prop(&self, ast: &mut crate::Ast, prop: MemberProp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = prop.node_id().into();
    }
}
impl MemberProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Ident(it) => it.span(ast),
            Self::PrivateName(it) => it.span(ast),
            Self::Computed(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Ident(it) => it.set_span(ast, span),
            Self::PrivateName(it) => it.set_span(ast, span),
            Self::Computed(it) => it.set_span(ast, span),
        }
    }
}
impl SuperPropExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn obj(&self, ast: &crate::Ast) -> Super {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Super::from_node_id(ret, ast)
    }
    #[inline]
    pub fn prop(&self, ast: &crate::Ast) -> SuperProp {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        SuperProp::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_obj(&self, ast: &mut crate::Ast, obj: Super) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = obj.node_id().into();
    }
    #[inline]
    pub fn set_prop(&self, ast: &mut crate::Ast, prop: SuperProp) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = prop.node_id().into();
    }
}
impl SuperProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Ident(it) => it.span(ast),
            Self::Computed(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Ident(it) => it.set_span(ast, span),
            Self::Computed(it) => it.set_span(ast, span),
        }
    }
}
impl CondExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn test(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn cons(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn alt(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_test(&self, ast: &mut crate::Ast, test: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = test.node_id().into();
    }
    #[inline]
    pub fn set_cons(&self, ast: &mut crate::Ast, cons: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = cons.node_id().into();
    }
    #[inline]
    pub fn set_alt(&self, ast: &mut crate::Ast, alt: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = alt.node_id().into();
    }
}
impl CallExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn callee(&self, ast: &crate::Ast) -> Callee {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Callee::from_node_id(ret, ast)
    }
    #[inline]
    pub fn args(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_callee(&self, ast: &mut crate::Ast, callee: Callee) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = callee.node_id().into();
    }
    #[inline]
    pub fn set_args(&self, ast: &mut crate::Ast, args: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = args.into();
    }
}
impl NewExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn callee(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn args(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_callee(&self, ast: &mut crate::Ast, callee: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = callee.node_id().into();
    }
    #[inline]
    pub fn set_args(&self, ast: &mut crate::Ast, args: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = args.into();
    }
}
impl SeqExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exprs(&self, ast: &crate::Ast) -> TypedSubRange<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exprs(&self, ast: &mut crate::Ast, exprs: TypedSubRange<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = exprs.into();
    }
}
impl ArrowExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn params(&self, ast: &crate::Ast) -> TypedSubRange<Pat> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> BlockStmtOrExpr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmtOrExpr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn is_async(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn is_generator(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_params(&self, ast: &mut crate::Ast, params: TypedSubRange<Pat>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = params.into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: BlockStmtOrExpr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
    #[inline]
    pub fn set_is_async(&self, ast: &mut crate::Ast, is_async: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_async.into();
    }
    #[inline]
    pub fn set_is_generator(&self, ast: &mut crate::Ast, is_generator: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_generator.into();
    }
}
impl YieldExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn delegate(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].optional_node = arg.optional_node_id().into();
    }
    #[inline]
    pub fn set_delegate(&self, ast: &mut crate::Ast, delegate: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = delegate.into();
    }
}
impl MetaPropExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> MetaPropKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        MetaPropKind::from_extra_data(ret)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: MetaPropKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = kind.to_extra_data();
    }
}
impl AwaitExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl Tpl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn exprs(&self, ast: &crate::Ast) -> TypedSubRange<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn quasis(&self, ast: &crate::Ast) -> TypedSubRange<TplElement> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_exprs(&self, ast: &mut crate::Ast, exprs: TypedSubRange<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = exprs.into();
    }
    #[inline]
    pub fn set_quasis(&self, ast: &mut crate::Ast, quasis: TypedSubRange<TplElement>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = quasis.into();
    }
}
impl TaggedTpl {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn tag(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn tpl(&self, ast: &crate::Ast) -> Tpl {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Tpl::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_tag(&self, ast: &mut crate::Ast, tag: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = tag.node_id().into();
    }
    #[inline]
    pub fn set_tpl(&self, ast: &mut crate::Ast, tpl: Tpl) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = tpl.node_id().into();
    }
}
impl TplElement {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn tail(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn cooked(&self, ast: &crate::Ast) -> OptionalWtf8AtomId {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_wtf8_atom };
        ret.into()
    }
    #[inline]
    pub fn raw(&self, ast: &crate::Ast) -> AtomRef {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].atom };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_tail(&self, ast: &mut crate::Ast, tail: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = tail.into();
    }
    #[inline]
    pub fn set_cooked(&self, ast: &mut crate::Ast, cooked: OptionalWtf8AtomId) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_wtf8_atom = cooked.into();
    }
    #[inline]
    pub fn set_raw(&self, ast: &mut crate::Ast, raw: AtomRef) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].atom = raw.into();
    }
}
impl ParenExpr {
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
impl Callee {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Super(it) => it.span(ast),
            Self::Import(it) => it.span(ast),
            Self::Expr(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Super(it) => it.set_span(ast, span),
            Self::Import(it) => it.set_span(ast, span),
            Self::Expr(it) => it.set_span(ast, span),
        }
    }
}
impl Super {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl Import {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn phase(&self, ast: &crate::Ast) -> ImportPhase {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].other };
        ImportPhase::from_extra_data(ret)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_phase(&self, ast: &mut crate::Ast, phase: ImportPhase) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].other = phase.to_extra_data();
    }
}
impl ExprOrSpread {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::SpreadElement(it) => it.span(ast),
            Self::Expr(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::SpreadElement(it) => it.set_span(ast, span),
            Self::Expr(it) => it.set_span(ast, span),
        }
    }
}
impl BlockStmtOrExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::BlockStmt(it) => it.span(ast),
            Self::Expr(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::BlockStmt(it) => it.set_span(ast, span),
            Self::Expr(it) => it.set_span(ast, span),
        }
    }
}
impl AssignTarget {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Simple(it) => it.span(ast),
            Self::Pat(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Simple(it) => it.set_span(ast, span),
            Self::Pat(it) => it.set_span(ast, span),
        }
    }
}
impl AssignTargetPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Array(it) => it.span(ast),
            Self::Object(it) => it.span(ast),
            Self::Invalid(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Array(it) => it.set_span(ast, span),
            Self::Object(it) => it.set_span(ast, span),
            Self::Invalid(it) => it.set_span(ast, span),
        }
    }
}
impl SimpleAssignTarget {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Ident(it) => it.span(ast),
            Self::Member(it) => it.span(ast),
            Self::SuperProp(it) => it.span(ast),
            Self::Paren(it) => it.span(ast),
            Self::OptChain(it) => it.span(ast),
            Self::Invalid(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Ident(it) => it.set_span(ast, span),
            Self::Member(it) => it.set_span(ast, span),
            Self::SuperProp(it) => it.set_span(ast, span),
            Self::Paren(it) => it.set_span(ast, span),
            Self::OptChain(it) => it.set_span(ast, span),
            Self::Invalid(it) => it.set_span(ast, span),
        }
    }
}
impl OptChainExpr {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn optional(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn base(&self, ast: &crate::Ast) -> OptChainBase {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        OptChainBase::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].bool = optional.into();
    }
    #[inline]
    pub fn set_base(&self, ast: &mut crate::Ast, base: OptChainBase) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = base.node_id().into();
    }
}
impl OptChainBase {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Member(it) => it.span(ast),
            Self::Call(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Member(it) => it.set_span(ast, span),
            Self::Call(it) => it.set_span(ast, span),
        }
    }
}
impl OptCall {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn callee(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn args(&self, ast: &crate::Ast) -> TypedSubRange<ExprOrSpread> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_callee(&self, ast: &mut crate::Ast, callee: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = callee.node_id().into();
    }
    #[inline]
    pub fn set_args(&self, ast: &mut crate::Ast, args: TypedSubRange<ExprOrSpread>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = args.into();
    }
}
impl Invalid {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
}
impl Function {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn params(&self, ast: &crate::Ast) -> TypedSubRange<Param> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_generator(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn is_async(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_params(&self, ast: &mut crate::Ast, params: TypedSubRange<Param>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = params.into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_generator(&self, ast: &mut crate::Ast, is_generator: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_generator.into();
    }
    #[inline]
    pub fn set_is_async(&self, ast: &mut crate::Ast, is_async: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_async.into();
    }
}
impl Param {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn pat(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_pat(&self, ast: &mut crate::Ast, pat: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = pat.node_id().into();
    }
}
impl ParamOrTsParamProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Param(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Param(it) => it.set_span(ast, span),
        }
    }
}
impl Class {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> TypedSubRange<ClassMember> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn super_class(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_abstract(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: TypedSubRange<ClassMember>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = body.into();
    }
    #[inline]
    pub fn set_super_class(&self, ast: &mut crate::Ast, super_class: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = super_class.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_abstract(&self, ast: &mut crate::Ast, is_abstract: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_abstract.into();
    }
}
impl ClassMember {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Constructor(it) => it.span(ast),
            Self::Method(it) => it.span(ast),
            Self::PrivateMethod(it) => it.span(ast),
            Self::ClassProp(it) => it.span(ast),
            Self::PrivateProp(it) => it.span(ast),
            Self::Empty(it) => it.span(ast),
            Self::StaticBlock(it) => it.span(ast),
            Self::AutoAccessor(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Constructor(it) => it.set_span(ast, span),
            Self::Method(it) => it.set_span(ast, span),
            Self::PrivateMethod(it) => it.set_span(ast, span),
            Self::ClassProp(it) => it.set_span(ast, span),
            Self::PrivateProp(it) => it.set_span(ast, span),
            Self::Empty(it) => it.set_span(ast, span),
            Self::StaticBlock(it) => it.set_span(ast, span),
            Self::AutoAccessor(it) => it.set_span(ast, span),
        }
    }
}
impl ClassProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl PrivateProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PrivateName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PrivateName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PrivateName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl ClassMethod {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> MethodKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].other };
        MethodKind::from_extra_data(ret)
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: MethodKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].other = kind.to_extra_data();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl PrivateMethod {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PrivateName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PrivateName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn kind(&self, ast: &crate::Ast) -> MethodKind {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].other };
        MethodKind::from_extra_data(ret)
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PrivateName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
    #[inline]
    pub fn set_kind(&self, ast: &mut crate::Ast, kind: MethodKind) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].other = kind.to_extra_data();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl Constructor {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn params(&self, ast: &crate::Ast) -> TypedSubRange<ParamOrTsParamProp> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_params(&self, ast: &mut crate::Ast, params: TypedSubRange<ParamOrTsParamProp>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].sub_range = params.into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
}
impl StaticBlock {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> BlockStmt {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BlockStmt::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: BlockStmt) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = body.node_id().into();
    }
}
impl Key {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Private(it) => it.span(ast),
            Self::Public(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Private(it) => it.set_span(ast, span),
            Self::Public(it) => it.set_span(ast, span),
        }
    }
}
impl AutoAccessor {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> Key {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Key::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn is_static(&self, ast: &crate::Ast) -> bool {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].bool };
        ret.into()
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: Key) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
    }
    #[inline]
    pub fn set_is_static(&self, ast: &mut crate::Ast, is_static: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].bool = is_static.into();
    }
}
impl Prop {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Shorthand(it) => it.span(ast),
            Self::KeyValue(it) => it.span(ast),
            Self::Assign(it) => it.span(ast),
            Self::Getter(it) => it.span(ast),
            Self::Setter(it) => it.span(ast),
            Self::Method(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Shorthand(it) => it.set_span(ast, span),
            Self::KeyValue(it) => it.set_span(ast, span),
            Self::Assign(it) => it.set_span(ast, span),
            Self::Getter(it) => it.set_span(ast, span),
            Self::Setter(it) => it.set_span(ast, span),
            Self::Method(it) => it.set_span(ast, span),
        }
    }
}
impl KeyValueProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = value.node_id().into();
    }
}
impl AssignProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> Ident {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Ident::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: Ident) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = value.node_id().into();
    }
}
impl GetterProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
}
impl SetterProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn this_param(&self, ast: &crate::Ast) -> Option<Pat> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Pat::from_node_id(id, ast))
    }
    #[inline]
    pub fn param(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn body(&self, ast: &crate::Ast) -> Option<BlockStmt> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| BlockStmt::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_this_param(&self, ast: &mut crate::Ast, this_param: Option<Pat>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = this_param.optional_node_id().into();
    }
    #[inline]
    pub fn set_param(&self, ast: &mut crate::Ast, param: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 2usize;
        ast.extra_data[offset].node = param.node_id().into();
    }
    #[inline]
    pub fn set_body(&self, ast: &mut crate::Ast, body: Option<BlockStmt>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 3usize;
        ast.extra_data[offset].optional_node = body.optional_node_id().into();
    }
}
impl MethodProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn function(&self, ast: &crate::Ast) -> Function {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Function::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_function(&self, ast: &mut crate::Ast, function: Function) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = function.node_id().into();
    }
}
impl PropName {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Ident(it) => it.span(ast),
            Self::Str(it) => it.span(ast),
            Self::Num(it) => it.span(ast),
            Self::Computed(it) => it.span(ast),
            Self::BigInt(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Ident(it) => it.set_span(ast, span),
            Self::Str(it) => it.set_span(ast, span),
            Self::Num(it) => it.set_span(ast, span),
            Self::Computed(it) => it.set_span(ast, span),
            Self::BigInt(it) => it.set_span(ast, span),
        }
    }
}
impl ComputedPropName {
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
impl Pat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Ident(it) => it.span(ast),
            Self::Array(it) => it.span(ast),
            Self::Rest(it) => it.span(ast),
            Self::Object(it) => it.span(ast),
            Self::Assign(it) => it.span(ast),
            Self::Invalid(it) => it.span(ast),
            Self::Expr(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Ident(it) => it.set_span(ast, span),
            Self::Array(it) => it.set_span(ast, span),
            Self::Rest(it) => it.set_span(ast, span),
            Self::Object(it) => it.set_span(ast, span),
            Self::Assign(it) => it.set_span(ast, span),
            Self::Invalid(it) => it.set_span(ast, span),
            Self::Expr(it) => it.set_span(ast, span),
        }
    }
}
impl ArrayPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn elems(&self, ast: &crate::Ast) -> TypedSubRange<Option<Pat>> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
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
    pub fn set_elems(&self, ast: &mut crate::Ast, elems: TypedSubRange<Option<Pat>>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = elems.into();
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = optional.into();
    }
}
impl ObjectPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn props(&self, ast: &crate::Ast) -> TypedSubRange<ObjectPatProp> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].sub_range };
        unsafe { ret.cast_to_typed() }
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
    pub fn set_props(&self, ast: &mut crate::Ast, props: TypedSubRange<ObjectPatProp>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].sub_range = props.into();
    }
    #[inline]
    pub fn set_optional(&self, ast: &mut crate::Ast, optional: bool) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].bool = optional.into();
    }
}
impl AssignPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn left(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn right(&self, ast: &crate::Ast) -> Expr {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Expr::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_left(&self, ast: &mut crate::Ast, left: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = left.node_id().into();
    }
    #[inline]
    pub fn set_right(&self, ast: &mut crate::Ast, right: Expr) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = right.node_id().into();
    }
}
impl RestPat {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn dot_3_token(&self, ast: &crate::Ast) -> Span {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].span };
        ret.into()
    }
    #[inline]
    pub fn arg(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_dot3_token(&self, ast: &mut crate::Ast, dot3_token: Span) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].span = dot3_token.into();
    }
    #[inline]
    pub fn set_arg(&self, ast: &mut crate::Ast, arg: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = arg.node_id().into();
    }
}
impl ObjectPatProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::KeyValue(it) => it.span(ast),
            Self::Assign(it) => it.span(ast),
            Self::Rest(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::KeyValue(it) => it.set_span(ast, span),
            Self::Assign(it) => it.set_span(ast, span),
            Self::Rest(it) => it.set_span(ast, span),
        }
    }
}
impl KeyValuePatProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> PropName {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        PropName::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Pat {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].node };
        Pat::from_node_id(ret, ast)
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: PropName) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Pat) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].node = value.node_id().into();
    }
}
impl AssignPatProp {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn key(&self, ast: &crate::Ast) -> BindingIdent {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].node };
        BindingIdent::from_node_id(ret, ast)
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Option<Expr> {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        let ret = unsafe { ast.extra_data[offset].optional_node };
        ret.map(|id| Expr::from_node_id(id, ast))
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        ast.nodes[self.0].span = span;
    }
    #[inline]
    pub fn set_key(&self, ast: &mut crate::Ast, key: BindingIdent) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].node = key.node_id().into();
    }
    #[inline]
    pub fn set_value(&self, ast: &mut crate::Ast, value: Option<Expr>) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 1usize;
        ast.extra_data[offset].optional_node = value.optional_node_id().into();
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
impl Lit {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        match self {
            Self::Str(it) => it.span(ast),
            Self::Bool(it) => it.span(ast),
            Self::Null(it) => it.span(ast),
            Self::Num(it) => it.span(ast),
            Self::BigInt(it) => it.span(ast),
            Self::Regex(it) => it.span(ast),
        }
    }
    #[inline]
    pub fn set_span(&self, ast: &mut crate::Ast, span: crate::Span) {
        match self {
            Self::Str(it) => it.set_span(ast, span),
            Self::Bool(it) => it.set_span(ast, span),
            Self::Null(it) => it.set_span(ast, span),
            Self::Num(it) => it.set_span(ast, span),
            Self::BigInt(it) => it.set_span(ast, span),
            Self::Regex(it) => it.set_span(ast, span),
        }
    }
}
impl Str {
    #[inline]
    pub fn span(&self, ast: &crate::Ast) -> crate::Span {
        ast.nodes[self.0].span
    }
    #[inline]
    pub fn value(&self, ast: &crate::Ast) -> Wtf8AtomId {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        let ret = unsafe { ast.extra_data[offset].wtf8_atom };
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
    pub fn set_value(&self, ast: &mut crate::Ast, value: Wtf8AtomId) {
        let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + 0usize;
        ast.extra_data[offset].wtf8_atom = value.into();
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
