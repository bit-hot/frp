use syn::{DeriveInput, GenericParam, parse_quote, Generics};
use quote::quote;
use proc_macro2::{Ident, Span};

pub fn impl_msg_struct(derive_input: DeriveInput) -> proc_macro::TokenStream {
    // 结构体名称
    let struct_name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    let upper_const_struct = Ident::new(get_upper_struct(struct_name.to_string()).as_str(), Span::call_site());

    let expanded = quote! {
        impl #impl_generics frp_trait::msg::MsgBase for #struct_name #ty_generics #where_clause {
            fn get_body(&self) -> String {
                serde_json::to_string(&self).unwrap()
            }

            fn get_head_byte(&self) -> u8 {
                frp_trait::msg::#upper_const_struct as u8
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn get_upper_struct(struct_name: String) -> String {
    let mut name = String::new();
    for (i, c) in struct_name.chars().enumerate() {
        if i == 0 ||  c.is_lowercase() {
            name.push(c.to_ascii_uppercase());
        } else {
            name.push('_');
            name.push(c.to_ascii_uppercase());
        }
    }
    name
}