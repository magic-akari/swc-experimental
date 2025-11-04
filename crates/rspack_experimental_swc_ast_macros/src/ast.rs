use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Ident, Item, ItemEnum, ItemStruct, Type};

pub fn expand_ast(item: Item) -> TokenStream {
    match item {
        Item::Enum(item_enum) => expand_enum(item_enum),
        Item::Struct(item_struct) => expand_struct(item_struct),
        _ => panic!("AST type must be enum or struct"),
    }
}

fn expand_struct(item: ItemStruct) -> TokenStream {
    let name = item.ident.clone();

    let decl = quote! {
        pub struct #name(NodeId);
    };

    let node_id_getter = quote! {
        pub fn node_id(&self) -> NodeId {
            self.0
        }
    };

    let mut field_getters = TokenStream::new();
    let mut field_setters = TokenStream::new();

    field_getters.extend(quote! {
        pub fn span(&self, ast: &Ast) -> Span {
            ast.nodes[self.0].span
        }
    });

    field_setters.extend(quote! {
        pub fn set_span(&self, ast: &mut Ast, span: Span) {
            ast.nodes[self.0].span = span;
        }
    });

    for (offset, field) in item.fields.into_iter().enumerate() {
        let field_name = field.ident.expect("ast field should have name");
        let field_ty = field.ty;
        let extra_data_name =
            get_extra_data_name_from_field(&field_ty).expect("Unsupported field type");

        let getter_name = field_name.clone();
        field_getters.extend(quote! {
            pub fn #getter_name(&self, ast: &Ast) -> #field_ty {
                let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + #offset;
                let ret = unsafe { ast.extra_data[offset].#extra_data_name };
                ret.into()
            }
        });

        let setter_name = format_ident!("set_{}", field_name);
        field_setters.extend(quote! {
            pub fn #setter_name(&self, ast: &mut Ast, #field_name: #field_ty) {
                let offset = unsafe { ast.nodes[self.0].data.extra_data_start } + #offset;
                ast.extra_data[offset].#extra_data_name = #field_name.into();
            }
        });
    }

    quote! {
        #decl

        impl #name {
            #node_id_getter
            #field_getters
            #field_setters
        }
    }
    .into()
}

fn expand_enum(item: ItemEnum) -> TokenStream {
    quote! { #item }
}

fn get_extra_data_name_from_field(ty: &Type) -> Option<Ident> {
    let Type::Path(ty) = ty else {
        return None;
    };

    let ty_name = match ty.path.get_ident() {
        Some(ident) => Some(ident.to_string()),
        None => ty.path.segments.first().map(|s| s.ident.to_string()),
    }?;
    let name = match ty_name.as_str() {
        "NodeId" => "node",
        "AtomRef" => "atom",
        "BigIntId" => "bigint",
        "OptionalNodeId" => "optional_node",
        "OptionalAtomRef" => "optional_atom",
        "f64" => "number",
        "TypedSubRange" => "sub_range",
        _ => return None,
    };

    Some(Ident::new(name, Span::call_site()))
}
