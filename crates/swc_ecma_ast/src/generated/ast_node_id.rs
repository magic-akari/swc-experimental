use crate::{ast::*, node_id::*};
impl GetNodeId for Program {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Module(it) => it.node_id(),
            Self::Script(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Program> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Module {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Module> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Script {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Script> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ModuleItem {
    fn node_id(&self) -> NodeId {
        match self {
            Self::ModuleDecl(it) => it.node_id(),
            Self::Stmt(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ModuleItem> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ModuleDecl {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Import(it) => it.node_id(),
            Self::ExportDecl(it) => it.node_id(),
            Self::ExportNamed(it) => it.node_id(),
            Self::ExportDefaultDecl(it) => it.node_id(),
            Self::ExportDefaultExpr(it) => it.node_id(),
            Self::ExportAll(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ModuleDecl> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ImportDecl {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportDecl> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ImportSpecifier {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Named(it) => it.node_id(),
            Self::Default(it) => it.node_id(),
            Self::Namespace(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ImportSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ImportNamedSpecifier {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportNamedSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ImportDefaultSpecifier {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportDefaultSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ImportStarAsSpecifier {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportStarAsSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportDecl {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDecl> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for NamedExport {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<NamedExport> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportSpecifier {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Namespace(it) => it.node_id(),
            Self::Default(it) => it.node_id(),
            Self::Named(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ExportSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportNamespaceSpecifier {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportNamespaceSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ModuleExportName {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Ident(it) => it.node_id(),
            Self::Str(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ModuleExportName> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportDefaultSpecifier {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDefaultSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportNamedSpecifier {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportNamedSpecifier> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportDefaultDecl {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDefaultDecl> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for DefaultDecl {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Class(it) => it.node_id(),
            Self::Fn(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<DefaultDecl> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportDefaultExpr {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDefaultExpr> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ExportAll {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportAll> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Stmt {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Empty(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Stmt> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for EmptyStmt {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<EmptyStmt> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Decl {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Class(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Decl> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ClassDecl {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ClassDecl> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Expr {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Lit(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Expr> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ObjectLit {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ObjectLit> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for ClassExpr {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ClassExpr> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for FnExpr {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<FnExpr> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Ident {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Ident> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for IdentName {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<IdentName> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for PrivateName {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<PrivateName> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for BindingIdent {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<BindingIdent> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Lit {
    fn node_id(&self) -> NodeId {
        match self {
            Self::Str(it) => it.node_id(),
            Self::Bool(it) => it.node_id(),
            Self::Null(it) => it.node_id(),
            Self::Num(it) => it.node_id(),
            Self::BigInt(it) => it.node_id(),
            Self::Regex(it) => it.node_id(),
            Self::JSXText(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Lit> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Str {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Str> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Bool {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Bool> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Null {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Null> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Number {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Number> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for BigInt {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<BigInt> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for Regex {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Regex> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl GetNodeId for JSXText {
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<JSXText> {
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
