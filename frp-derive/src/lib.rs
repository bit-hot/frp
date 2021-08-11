use syn::{parse_macro_input, DeriveInput};
use proc_macro2::TokenStream;

mod msg;

#[proc_macro_derive(FrpMsg)]
pub fn msg_base_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    msg::impl_msg_struct(input)
}
