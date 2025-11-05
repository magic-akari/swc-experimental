use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    AST_CRATE_PATH,
    output::{RawOutput, RustOutput, output_path},
    schema::{AstStruct, AstType, Schema},
};

pub fn ast_builder(schema: &Schema) -> RawOutput {
    let mut build_functions = TokenStream::new();
    for ty in schema.types.iter() {
        match ty {
            AstType::Struct(ast_struct) => {
                build_functions.extend(generate_build_function_for_struct(ast_struct, schema))
            }
            AstType::Enum(_ast_enum) => {
                continue;
                // for variant in ast_enum.variants.iter() {
                //     let Some(ty_id) = variant.type_id else {
                //         continue;
                //     };

                //     let ty_name = schema.types[ty_id].name();
                //     let fn_name = Ident::new(
                //         &format!("{}{}", ast_enum.name, variant.name).to_case(Case::Snake),
                //         Span::call_site(),
                //     );
                //     // let ty_name = variant.type_id

                //     let tokens = quote! {
                //         // pub fn #fn_name(&mut self) {}
                //     };

                //     build_functions.extend(tokens);
                // }
            }
            _ => continue,
        };
    }

    let output = quote! {
            use swc_common::Span;

            use crate::{
                Ast, AstNode, ExtraData, NodeData, NodeKind,
                node_id::*,
                ast::*,
            };

            impl Ast {
                #build_functions
            }
    };

    RustOutput {
        path: output_path(AST_CRATE_PATH, "ast_builder"),
        tokens: output,
    }
    .into()
}

fn generate_build_function_for_struct(ast: &AstStruct, schema: &Schema) -> TokenStream {
    let fn_name = format_ident!("{}", ast.name.to_case(Case::Snake));
    let ret_ty = ast.full_ident(schema);
    let fn_params = generate_fn_params(ast, schema);

    let mut add_extra_data = TokenStream::new();
    let node_kind = ast.full_ident(schema);

    for (index, field) in ast.fields.iter().enumerate() {
        let extra_data_id = format_ident!("_f{}", index);
        let field_name = format_ident!("{}", field.name);

        let field_ty = schema.types[field.type_id].wrapper_name();
        let extra_data_field = format_ident!("{}", map_field_type_to_extra_field(field_ty));
        add_extra_data.extend(quote! {
            let #extra_data_id = self.add_extra(ExtraData { #extra_data_field: #field_name.into() });
        });
    }

    let node_data = match ast.fields.len() {
        0 => quote!(empty: ()),
        _ => quote!(extra_data_start: _f0),
    };

    let tokens = quote! {
        pub fn #fn_name(&mut self, span: Span, #fn_params) -> TypedNodeId<#ret_ty> {
            #add_extra_data

            unsafe {
                self.add_node(AstNode {
                    span,
                    kind: NodeKind::#node_kind,
                    data: NodeData {
                        #node_data
                    },
                })
                .cast_to_typed()
            }
        }
    };

    tokens
}

fn generate_fn_params(ast: &AstStruct, schema: &Schema) -> TokenStream {
    let mut fields = Vec::default();
    for field in ast.fields.iter() {
        let field_name = format_ident!("{}", field.name);
        let field_ty = schema.types[field.type_id].full_ident(schema);
        fields.push(quote!(#field_name: #field_ty));
    }

    quote!( #(#fields),* )
}

fn map_field_type_to_extra_field(field_type: &str) -> &str {
    match field_type {
        "AtomRef" => "atom",
        "OptionalAtomRef" => "optional_atom",
        "BigIntId" => "bigint",
        "bool" => "bool",
        "f64" => "number",

        "TypedSubRange" => "sub_range",
        "TypedNodeId" => "node",
        "TypedOptionalNodeId" => "optional_node",
        _ => panic!("Unsupport field type {field_type}"),
    }
}
