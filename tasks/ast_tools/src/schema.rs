use std::collections::HashSet;

use oxc_index::IndexVec;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

oxc_index::define_index_type! {
    pub struct TypeId = u32;
}

impl TypeId {
    pub const DUMMY: Self = Self::from_raw_unchecked(0);
}

pub struct Schema {
    pub types: IndexVec<TypeId, AstType>,
}

#[derive(Debug)]
pub enum AstType {
    Struct(AstStruct),
    Enum(AstEnum),
    Vec(AstVec),
    Node(AstNode),
    Primitive(AstPrimitive),
}

impl AstType {
    pub fn name(&self) -> &str {
        match self {
            AstType::Struct(ast) => &ast.name,
            AstType::Enum(ast) => &ast.name,
            AstType::Vec(ast) => &ast.name,
            AstType::Node(ast) => &ast.name,
            AstType::Primitive(ast) => &ast.name,
        }
    }

    pub fn wrapper_name(&self) -> &str {
        match self {
            AstType::Struct(ast) => &ast.name,
            AstType::Enum(ast) => &ast.name,
            AstType::Vec(ast) => &ast.wrapper_name,
            AstType::Node(ast) => &ast.wrapper_name,
            AstType::Primitive(ast) => &ast.name,
        }
    }

    pub fn repr_ident<'a>(&'a self, schema: &'a Schema) -> TokenStream {
        match self {
            AstType::Struct(ast) => Ident::new(&ast.name, Span::call_site()).into_token_stream(),
            AstType::Enum(ast) => Ident::new(&ast.name, Span::call_site()).into_token_stream(),
            AstType::Vec(ast) => {
                let name = Ident::new(&ast.wrapper_name, Span::call_site());
                let inner_name =
                    Ident::new(schema.types[ast.inner_type_id].name(), Span::call_site());
                quote!( #name<#inner_name> )
            }
            AstType::Node(ast) => {
                Ident::new(schema.types[ast.inner_type_id].name(), Span::call_site())
                    .into_token_stream()
            }
            AstType::Primitive(ast) => Ident::new(&ast.name, Span::call_site()).into_token_stream(),
        }
    }
}

#[derive(Debug)]
pub struct AstStruct {
    pub type_id: TypeId,
    pub name: String,
    pub fields: Vec<AstStructField>,
    pub attrs: AstAttrs,
}

impl AstStruct {
    pub fn full_ident(&self, _schema: &Schema) -> TokenStream {
        format_ident!("{}", self.name).to_token_stream()
    }
}

#[derive(Debug)]
pub struct AstStructField {
    pub type_id: TypeId,
    pub name: String,
}

#[derive(Debug)]
pub struct AstEnum {
    pub type_id: TypeId,
    pub name: String,
    pub variants: Vec<AstEnumVariant>,
    pub attrs: AstAttrs,
}

#[derive(Debug)]
pub struct AstEnumVariant {
    pub type_id: Option<TypeId>,
    pub name: String,
}

#[derive(Debug)]
pub struct AstVec {
    pub type_id: TypeId,
    pub name: String,
    pub wrapper_name: String,
    pub inner_type_id: TypeId,
}

#[derive(Debug)]
pub struct AstNode {
    pub type_id: TypeId,
    pub name: String,
    pub wrapper_name: String,
    pub inner_type_id: TypeId,
    pub is_optional: bool,
}

#[derive(Debug)]
pub struct AstPrimitive {
    pub type_id: TypeId,
    pub name: &'static str,
}

#[derive(Debug)]
pub struct AstAttrs {
    pub ast_attrs: HashSet<String>,
    pub generate_derives: HashSet<String>,
}
