use syn::{parse_macro_input, DeriveInput, Data, DataStruct};
use quote::quote;
use proc_macro::TokenStream;

#[proc_macro_derive(MsgBaseTrait)]
pub fn msg_base_macro_derive(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    // 结构体名称
    let struct_name = &input.ident;

    let expanded = match input.data {
        Data::Struct(DataStruct { ref fields, .. }) => {
            let implemented_base = quote! {
                impl frp_core::msg::MsgBase for #struct_name {
                    fn get_body(&self) -> String {
                        serde_json::to_string(&self).unwrap()
                    }

                    fn get_head_byte(&self) -> u8 {
                        frp_core::msg::common::LOGIN as u8
                    }
                }
            };

            implemented_base
        }
        _ => panic!("只对结构体有效, 请确认是否使用在结构体上")
    };
    expanded.into()
}