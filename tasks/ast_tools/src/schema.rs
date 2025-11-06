use std::collections::HashSet;

use oxc_index::IndexVec;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

oxc_index::define_index_type! {
    pub struct TypeId = u32;
}

oxc_index::define_index_type! {
    pub struct FileId = u32;
}

impl TypeId {
    pub const DUMMY: Self = Self::from_raw_unchecked(0);
}

pub struct Schema {
    pub files: IndexVec<FileId, AstFile>,
    pub types: IndexVec<TypeId, AstType>,
}

pub struct AstFile {
    pub krate: String,
    pub use_path: TokenStream,
}

impl AstFile {
    pub fn new(file_path: &str) -> Self {
        let mut parts = file_path.split('/');
        let krate = match parts.next().unwrap() {
            "crates" => parts.next().unwrap().to_string(),
            _ => panic!("Expect path beginning with `crates/`"),
        };

        // Skip `src`
        parts.next();

        let mut use_path = quote!();
        for part in parts {
            if let Some(rs_index) = part.find(".rs") {
                let ident = Ident::new(&part[..rs_index], Span::call_site());
                use_path.extend(quote!(#ident::*));
            } else {
                let ident = Ident::new(part, Span::call_site());
                use_path.extend(quote!(#ident::));
            }
        }

        Self { krate, use_path }
    }

    pub fn use_path_in_crate(&self) -> TokenStream {
        let path = &self.use_path;
        TokenStream::from(quote!(use crate::#path;))
    }
}

#[derive(Debug)]
pub enum AstType {
    Struct(AstStruct),
    Enum(AstEnum),
    SubRange(AstTypedSubRange),
    TypedId(AstTypedId),
    Primitive(AstPrimitive),
}

impl AstType {
    pub fn name(&self) -> &str {
        match self {
            AstType::Struct(ast) => &ast.name,
            AstType::Enum(ast) => &ast.name,
            AstType::SubRange(ast) => &ast.name,
            AstType::TypedId(ast) => &ast.name,
            AstType::Primitive(ast) => &ast.name,
        }
    }

    pub fn wrapper_name(&self) -> &str {
        match self {
            AstType::Struct(ast) => &ast.name,
            AstType::Enum(ast) => &ast.name,
            AstType::SubRange(ast) => &ast.wrapper_name,
            AstType::TypedId(ast) => &ast.wrapper_name,
            AstType::Primitive(ast) => &ast.name,
        }
    }

    pub fn repr_ident<'a>(&'a self, schema: &'a Schema) -> TokenStream {
        match self {
            AstType::Struct(ast) => Ident::new(&ast.name, Span::call_site()).into_token_stream(),
            AstType::Enum(ast) => Ident::new(&ast.name, Span::call_site()).into_token_stream(),
            AstType::SubRange(ast) => {
                let name = Ident::new(&ast.wrapper_name, Span::call_site());
                let inner_name =
                    Ident::new(schema.types[ast.inner_type_id].name(), Span::call_site());
                quote!( #name<#inner_name> )
            }
            AstType::TypedId(ast) => {
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
    pub file_id: FileId,
    pub name: String,
    pub has_lifetime: bool,
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
    pub file_id: FileId,
    pub name: String,
    pub has_lifetime: bool,
    pub variants: Vec<AstEnumVariant>,
    pub attrs: AstAttrs,
}

impl AstEnum {
    pub fn full_ident(&self, _schema: &Schema) -> TokenStream {
        format_ident!("{}", self.name).to_token_stream()
    }
}

impl AstEnum {
    pub fn ty_with_lifetime(&self) -> TokenStream {
        let ident = format_ident!("{}", self.name);
        if self.has_lifetime {
            quote!(#ident<'a>)
        } else {
            quote!(#ident)
        }
    }
}

#[derive(Debug)]
pub struct AstEnumVariant {
    pub type_id: Option<TypeId>,
    pub name: String,
}

#[derive(Debug)]
pub struct AstTypedSubRange {
    pub type_id: TypeId,
    pub name: String,
    pub wrapper_name: String,
    pub inner_type_id: TypeId,
}

#[derive(Debug)]
pub struct AstTypedId {
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
