use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, ItemEnum, ItemStruct};

pub fn expand_ast(item: Item) -> TokenStream {
    match item {
        Item::Enum(item_enum) => expand_enum(item_enum),
        Item::Struct(item_struct) => expand_struct(item_struct),
        _ => panic!("AST type must be enum or struct"),
    }
}

fn expand_struct(item: ItemStruct) -> TokenStream {
    let name = item.ident.clone();
    quote! {
        pub struct #name(pub(crate) crate::NodeId);
    }
}

fn expand_enum(item: ItemEnum) -> TokenStream {
    let fn_cast = quote! {};
    quote! {
        #item
        #fn_cast
    }
}
