use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstEnum, AstStruct, AstType, Schema},
};

pub fn ast_node_id(schema: &Schema) -> RawOutput {
    let mut impls = TokenStream::new();
    for ty in schema.types.iter() {
        match ty {
            AstType::Struct(ast_struct) => {
                impls.extend(generate_node_id_for_struct(ast_struct, schema))
            }
            AstType::Enum(ast_enum) => impls.extend(generate_node_id_for_enum(ast_enum, schema)),
            _ => continue,
        };
    }

    let output = quote! {
            use crate::{Ast, NodeKind};
            use crate::{node_id::*, ast::*};

            #impls
    };

    RustOutput {
        path: output_path(AST_CRATE_PATH, "ast_node_id"),
        tokens: output,
    }
    .into()
}

fn generate_node_id_for_struct(ast: &AstStruct, _schema: &Schema) -> TokenStream {
    let name = format_ident!("{}", ast.name);

    let node_id_getter = quote! {
        impl GetNodeId for #name {
            #[inline]
            fn node_id(&self) -> NodeId {
                self.0
            }
        }
    };

    let optional_id_getter = quote! {
        impl GetOptionalNodeId for Option<#name> {
            #[inline]
            fn optional_node_id(&self) -> OptionalNodeId {
                match self {
                    Some(it) => it.node_id().into(),
                    None => OptionalNodeId::none(),
                }
            }
        }
    };

    let from_node_id = quote! {
        impl #name {
            #[inline]
            pub(crate) fn from_node_id(node_id: NodeId, _ast: &Ast) -> Self {
                Self(node_id)
            }
        }
    };

    quote! {
        #node_id_getter
        #optional_id_getter
        #from_node_id
    }
}

fn generate_node_id_for_enum(ast: &AstEnum, schema: &Schema) -> TokenStream {
    let name = format_ident!("{}", ast.name);

    let mut node_id_getter_arms = TokenStream::new();
    for variant in ast.variants.iter() {
        let variant_ident = format_ident!("{}", variant.name);
        node_id_getter_arms.extend(quote! {
            Self::#variant_ident(it) => it.node_id(),
        });
    }

    let node_id_getter = quote! {
        impl GetNodeId for #name {
            #[inline]
            fn node_id(&self) -> NodeId {
                match self {
                    #node_id_getter_arms
                }
            }
        }
    };

    let optional_id_getter = quote! {
        impl GetOptionalNodeId for Option<#name> {
            #[inline]
            fn optional_node_id(&self) -> OptionalNodeId {
                match self {
                    Some(it) => it.node_id().into(),
                    None => OptionalNodeId::none(),
                }
            }
        }
    };

    let mut from_node_id_arms = TokenStream::new();
    for variant in ast.variants.iter() {
        let Some(variant_type_id) = variant.type_id else {
            continue;
        };

        let variant_ident = format_ident!("{}", variant.name);
        let variant_type = &schema.types[variant_type_id];
        let variant_type_name = format_ident!("{}", variant_type.name());
        from_node_id_arms.extend(quote! {
            NodeKind::#variant_ident => Self::#variant_ident(#variant_type_name::from_node_id(id, ast)),
        });
    }
    let from_node_id = quote! {
        impl #name {
            #[inline]
            pub(crate) fn from_node_id(id: NodeId, ast: &Ast) -> Self {
                match &ast.nodes[id].kind {
                    #from_node_id_arms
                    _ => unreachable!(),
                }
            }
        }
    };

    quote! {
        #node_id_getter
        #optional_id_getter
        #from_node_id
    }
}
