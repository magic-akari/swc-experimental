use rspack_experimental_swc_ast_macros::ast;

use crate::ast::{ClassExpr, Decl, Expr, FnExpr, Ident, ObjectLit, Str};

#[ast]
pub enum ModuleDecl {
    Import(ImportDecl),
    ExportDecl(ExportDecl),
    ExportNamed(NamedExport),
    ExportDefaultDecl(ExportDefaultDecl),
    ExportDefaultExpr(ExportDefaultExpr),
    ExportAll(ExportAll),
    // TsImportEquals(TsImportEqualsDecl),
    // TsExportAssignment(TsExportAssignment),
    // TsNamespaceExport(TsNamespaceExportDecl),
}

#[ast]
pub struct ImportDecl {
    specifiers: TypedSubRange<ImportSpecifier>,
    src: TypedNode<Str>,
    type_only: bool,
    with: TypedOptionalNode<ObjectLit>,
    // phase: ImportPhase,
}

#[ast]
pub enum ImportSpecifier {
    Named(ImportNamedSpecifier),
    Default(ImportDefaultSpecifier),
    Namespace(ImportStarAsSpecifier),
}

#[ast]
pub struct ImportNamedSpecifier {
    local: TypedNode<Ident>,
    imported: TypedOptionalNode<ModuleExportName>,
    is_type_only: bool,
}

#[ast]
pub struct ImportDefaultSpecifier {
    local: TypedNode<Ident>,
}

#[ast]
pub struct ImportStarAsSpecifier {
    local: TypedNode<Ident>,
}

#[ast]
pub struct ExportDecl {
    decl: TypedNode<Decl>,
}

#[ast]
pub struct NamedExport {
    specifiers: TypedSubRange<ExportSpecifier>,
    src: TypedOptionalNode<Str>,
    type_only: bool,
    with: TypedOptionalNode<ObjectLit>,
}

#[ast]
pub enum ExportSpecifier {
    Namespace(ExportNamespaceSpecifier),
    Default(ExportDefaultSpecifier),
    Named(ExportNamedSpecifier),
}

#[ast]
pub struct ExportNamespaceSpecifier {
    name: TypedNode<ModuleExportName>,
}

#[ast]
pub enum ModuleExportName {
    Ident(Ident),
    Str(Str),
}

#[ast]
pub struct ExportDefaultSpecifier {
    exported: TypedNode<Ident>,
}

#[ast]
pub struct ExportNamedSpecifier {
    orig: TypedNode<ModuleExportName>,
    exported: TypedOptionalNode<ModuleExportName>,
    is_type_only: bool,
}

#[ast]
pub struct ExportDefaultDecl {
    decl: TypedNode<DefaultDecl>,
}

#[ast]
pub enum DefaultDecl {
    Class(ClassExpr),
    Fn(FnExpr),
    // TsInterfaceDecl(TsInterfaceDecl),
}

#[ast]
pub struct ExportDefaultExpr {
    expr: TypedNode<Expr>,
}

#[ast]
pub struct ExportAll {
    src: TypedNode<Str>,
    type_only: bool,
    with: TypedOptionalNode<ObjectLit>,
}
