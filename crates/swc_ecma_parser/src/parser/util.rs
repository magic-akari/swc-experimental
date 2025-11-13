use rspack_experimental_swc_ecma_ast::*;
use swc_common::Span;

pub trait IsSimpleParameterList {
    fn is_simple_parameter_list(&self, ast: &Ast) -> bool;
}

impl IsSimpleParameterList for Vec<Param> {
    fn is_simple_parameter_list(&self, ast: &Ast) -> bool {
        self.iter()
            .all(|param| matches!(param.pat(ast), Pat::Ident(_)))
    }
}

impl IsSimpleParameterList for Vec<Pat> {
    fn is_simple_parameter_list(&self, _ast: &Ast) -> bool {
        self.iter().all(|pat| matches!(pat, Pat::Ident(_)))
    }
}

impl IsSimpleParameterList for Vec<ParamOrTsParamProp> {
    fn is_simple_parameter_list(&self, ast: &Ast) -> bool {
        self.iter().all(|param| match param {
            ParamOrTsParamProp::Param(param) => matches!(param.pat(ast), Pat::Ident(_)),
        })
    }
}

pub trait IsInvalidClassName {
    fn invalid_class_name(&self, ast: &Ast) -> Option<Span>;
}

impl IsInvalidClassName for Ident {
    fn invalid_class_name(&self, ast: &Ast) -> Option<Span> {
        match ast.get_atom(self.sym(ast)).as_str() {
            "string" | "null" | "number" | "object" | "any" | "unknown" | "boolean" | "bigint"
            | "symbol" | "void" | "never" | "intrinsic" => Some(self.span(ast)),
            _ => None,
        }
    }
}
impl IsInvalidClassName for Option<Ident> {
    fn invalid_class_name(&self, ast: &Ast) -> Option<Span> {
        self.as_ref().and_then(|i| i.invalid_class_name(ast))
    }
}

pub trait ExprExt {
    fn is_valid_simple_assignment_target(&self, ast: &Ast, strict: bool) -> bool;
}

impl ExprExt for Expr {
    /// "IsValidSimpleAssignmentTarget" from spec.
    fn is_valid_simple_assignment_target(&self, ast: &Ast, strict: bool) -> bool {
        match self {
            Expr::Ident(ident) => {
                if strict && ident.is_reserved_in_strict_bind(ast) {
                    return false;
                }
                true
            }

            Expr::This(..)
            | Expr::Lit(..)
            | Expr::Array(..)
            | Expr::Object(..)
            | Expr::Fn(..)
            | Expr::Class(..)
            | Expr::Tpl(..)
            | Expr::TaggedTpl(..) => false,
            Expr::Paren(paren) => paren
                .expr(ast)
                .is_valid_simple_assignment_target(ast, strict),
            Expr::Member(member) => {
                let obj = member.obj(ast);
                match obj {
                    Expr::Member(..) => obj.is_valid_simple_assignment_target(ast, strict),
                    Expr::OptChain(..) => false,
                    _ => true,
                }
            }

            Expr::SuperProp(..) => true,

            Expr::New(..) | Expr::Call(..) => false,
            // TODO: Spec only mentions `new.target`
            Expr::MetaProp(..) => false,

            Expr::Update(..) => false,

            Expr::Unary(..) | Expr::Await(..) => false,

            Expr::Bin(..) => false,

            Expr::Cond(..) => false,

            Expr::Yield(..) | Expr::Arrow(..) | Expr::Assign(..) => false,

            Expr::Seq(..) => false,

            Expr::OptChain(..) => false,

            // MemberExpression is valid assignment target
            Expr::PrivateName(..) => false,

            // jsx
            // Expr::JSXMember(..)
            // | Expr::JSXNamespacedName(..)
            // | Expr::JSXEmpty(..)
            // | Expr::JSXElement(..)
            // | Expr::JSXFragment(..) => false,

            // // typescript
            // Expr::TsNonNull(TsNonNullExpr { ref expr, .. })
            // | Expr::TsTypeAssertion(TsTypeAssertion { ref expr, .. })
            // | Expr::TsAs(TsAsExpr { ref expr, .. })
            // | Expr::TsInstantiation(TsInstantiation { ref expr, .. })
            // | Expr::TsSatisfies(TsSatisfiesExpr { ref expr, .. }) => {
            //     expr.is_valid_simple_assignment_target(strict)
            // }

            // Expr::TsConstAssertion(..) => false,
            Expr::Invalid(..) => false,
            #[cfg(swc_ast_unknown)]
            _ => unreachable!(),
        }
    }
}

pub trait FromStmt: GetNodeId {
    fn from_stmt(stmt: Stmt) -> Self;
}

impl FromStmt for ModuleItem {
    fn from_stmt(stmt: Stmt) -> Self {
        ModuleItem::Stmt(stmt)
    }
}

impl FromStmt for Stmt {
    fn from_stmt(stmt: Stmt) -> Self {
        stmt
    }
}
