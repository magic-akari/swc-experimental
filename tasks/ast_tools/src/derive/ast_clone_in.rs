use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstEnum, AstStruct, AstType, Schema},
};

pub fn ast_clone_in(schema: &Schema) -> RawOutput {
    let mut impls = TokenStream::new();
    for ty in schema.types.iter() {
        match ty {
            AstType::Struct(ast_struct) => {
                impls.extend(generate_clone_in_for_struct(ast_struct, schema))
            }
            AstType::Enum(ast_enum) => impls.extend(generate_clone_in_for_enum(ast_enum, schema)),
            _ => continue,
        };
    }

    let output = quote! {
            #![allow(unused)]

            use crate::{Ast, NodeKind, CloneIn};
            use crate::{node_id::*, ast::*};

            #impls
    };

    RustOutput {
        path: output_path(AST_CRATE_PATH, "ast_clone_in"),
        tokens: output,
    }
    .into()
}

fn generate_clone_in_for_struct(ast: &AstStruct, _schema: &Schema) -> TokenStream {
    let name = format_ident!("{}", ast.name);

    let mut fields = TokenStream::new();
    for field in ast.fields.iter() {
        let field_ident = format_ident!("{}", field.name);
        fields.extend(quote! {
            #field_ident: CloneIn::clone_in(&self.#field_ident(ast), ast),
        });
    }

    let clone_in = quote! {
        impl CloneIn for #name {
            type Cloned = #name;

            #[inline]
            fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
                // #name {
                //     #fields
                // }
                todo!()
            }
        }
    };

    quote! {
        #clone_in
    }
}

fn generate_clone_in_for_enum(ast: &AstEnum, _schema: &Schema) -> TokenStream {
    let name = format_ident!("{}", ast.name);

    let mut arms = TokenStream::new();
    for variant in ast.variants.iter() {
        let variant_ident = format_ident!("{}", variant.name);
        arms.extend(quote! {
            Self::#variant_ident(it) => Self::#variant_ident(it.clone_in(ast)),
        });
    }

    let node_id_getter = quote! {
        impl CloneIn for #name {
            type Cloned = #name;

            #[inline]
            fn clone_in(&self, ast: &mut Ast) -> Self::Cloned {
                match self {
                    #arms
                }
            }
        }
    };

    quote! {
        #node_id_getter
    }
}
