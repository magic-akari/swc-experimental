use crate::{ast::*, node_id::*};
use crate::{Ast, NodeKind};
impl GetNodeId for Program {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Module(it) => it.node_id(),
            Self::Script(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Program> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Program {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Module => Self::Module(Module::from_node_id(id, ast)),
            NodeKind::Script => Self::Script(Script::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for Module {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Module> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Module {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Script {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Script> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Script {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ModuleItem {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::ModuleDecl(it) => it.node_id(),
            Self::Stmt(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ModuleItem> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ModuleItem {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::ModuleDecl => Self::ModuleDecl(ModuleDecl::from_node_id(id, ast)),
            NodeKind::Stmt => Self::Stmt(Stmt::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ModuleDecl {
    #[inline]
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
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ModuleDecl {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Import => Self::Import(ImportDecl::from_node_id(id, ast)),
            NodeKind::ExportDecl => Self::ExportDecl(ExportDecl::from_node_id(id, ast)),
            NodeKind::ExportNamed => Self::ExportNamed(NamedExport::from_node_id(id, ast)),
            NodeKind::ExportDefaultDecl => {
                Self::ExportDefaultDecl(ExportDefaultDecl::from_node_id(id, ast))
            }
            NodeKind::ExportDefaultExpr => {
                Self::ExportDefaultExpr(ExportDefaultExpr::from_node_id(id, ast))
            }
            NodeKind::ExportAll => Self::ExportAll(ExportAll::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ImportDecl {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportDecl> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ImportDecl {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ImportSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Named(it) => it.node_id(),
            Self::Default(it) => it.node_id(),
            Self::Namespace(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ImportSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ImportSpecifier {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Named => Self::Named(ImportNamedSpecifier::from_node_id(id, ast)),
            NodeKind::Default => Self::Default(ImportDefaultSpecifier::from_node_id(id, ast)),
            NodeKind::Namespace => Self::Namespace(ImportStarAsSpecifier::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ImportNamedSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportNamedSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ImportNamedSpecifier {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ImportDefaultSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportDefaultSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ImportDefaultSpecifier {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ImportStarAsSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ImportStarAsSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ImportStarAsSpecifier {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ExportDecl {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDecl> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportDecl {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for NamedExport {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<NamedExport> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl NamedExport {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ExportSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Namespace(it) => it.node_id(),
            Self::Default(it) => it.node_id(),
            Self::Named(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ExportSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportSpecifier {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Namespace => Self::Namespace(ExportNamespaceSpecifier::from_node_id(id, ast)),
            NodeKind::Default => Self::Default(ExportDefaultSpecifier::from_node_id(id, ast)),
            NodeKind::Named => Self::Named(ExportNamedSpecifier::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ExportNamespaceSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportNamespaceSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportNamespaceSpecifier {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ModuleExportName {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Ident(it) => it.node_id(),
            Self::Str(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<ModuleExportName> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ModuleExportName {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Ident => Self::Ident(Ident::from_node_id(id, ast)),
            NodeKind::Str => Self::Str(Str::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ExportDefaultSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDefaultSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportDefaultSpecifier {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ExportNamedSpecifier {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportNamedSpecifier> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportNamedSpecifier {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ExportDefaultDecl {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDefaultDecl> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportDefaultDecl {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for DefaultDecl {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Class(it) => it.node_id(),
            Self::Fn(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<DefaultDecl> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl DefaultDecl {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Class => Self::Class(ClassExpr::from_node_id(id, ast)),
            NodeKind::Fn => Self::Fn(FnExpr::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ExportDefaultExpr {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportDefaultExpr> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportDefaultExpr {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ExportAll {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ExportAll> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ExportAll {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Stmt {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Empty(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Stmt> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Stmt {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Empty => Self::Empty(EmptyStmt::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for EmptyStmt {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<EmptyStmt> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl EmptyStmt {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Decl {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Class(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Decl> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Decl {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Class => Self::Class(ClassDecl::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ClassDecl {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ClassDecl> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ClassDecl {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Expr {
    #[inline]
    fn node_id(&self) -> NodeId {
        match self {
            Self::Lit(it) => it.node_id(),
        }
    }
}
impl GetOptionalNodeId for Option<Expr> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Expr {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Lit => Self::Lit(Lit::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for ObjectLit {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ObjectLit> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ObjectLit {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for ClassExpr {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<ClassExpr> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl ClassExpr {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for FnExpr {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<FnExpr> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl FnExpr {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Ident {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Ident> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Ident {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for IdentName {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<IdentName> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl IdentName {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for PrivateName {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<PrivateName> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl PrivateName {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for BindingIdent {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<BindingIdent> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl BindingIdent {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Lit {
    #[inline]
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
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Lit {
    #[inline]
    pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
        match &ast.nodes[id].kind {
            NodeKind::Str => Self::Str(Str::from_node_id(id, ast)),
            NodeKind::Bool => Self::Bool(Bool::from_node_id(id, ast)),
            NodeKind::Null => Self::Null(Null::from_node_id(id, ast)),
            NodeKind::Num => Self::Num(Num::from_node_id(id, ast)),
            NodeKind::BigInt => Self::BigInt(BigInt::from_node_id(id, ast)),
            NodeKind::Regex => Self::Regex(Regex::from_node_id(id, ast)),
            NodeKind::JSXText => Self::JSXText(JSXText::from_node_id(id, ast)),
            _ => unreachable!(),
        }
    }
}
impl GetNodeId for Str {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Str> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Str {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Bool {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Bool> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Bool {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Null {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Null> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Null {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Num {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Num> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Num {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for BigInt {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<BigInt> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl BigInt {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for Regex {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<Regex> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl Regex {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
impl GetNodeId for JSXText {
    #[inline]
    fn node_id(&self) -> NodeId {
        self.0
    }
}
impl GetOptionalNodeId for Option<JSXText> {
    #[inline]
    fn optional_node_id(&self) -> OptionalNodeId {
        match self {
            Some(it) => it.node_id().into(),
            None => OptionalNodeId::none(),
        }
    }
}
impl JSXText {
    #[inline]
    pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
        Self(node_id)
    }
}
