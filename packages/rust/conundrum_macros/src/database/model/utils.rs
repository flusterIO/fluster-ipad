use proc_macro2::TokenStream;
use quote::quote;

pub struct MacroUtils;

impl MacroUtils {
    pub fn comma_separated_list(items: Vec<TokenStream>) -> TokenStream {
        quote! {
            #(#items),*
        }
    }
}
