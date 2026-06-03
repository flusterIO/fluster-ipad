/// ! I finally learned macros and now I can't find a usecase for them :(
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(ConundrumMacro, attributes(cdrm_property))]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    #[allow(clippy::collapsible_if)]
    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            let field_actions =
                fields.named.iter().map(|f| {
                                       let field_name = &f.ident;

                                       match f.attrs.iter().find(|a| a.path().is_ident("cdrm_property")) {
                                           Some(attr) => {
                                               let func_name = attr.parse_args::<syn::Expr>().unwrap();
                                               quote! {
                                                   if let Some(res) = (#func_name)(&self.#field_name) {
                                                       properties.push(res);
                                                   }
                                               }
                                           }
                                           None => {
                                               quote! {
                                                   if let Some(res) = &self.#field_name.as_ref().cloned().map(|n| n.as_cdrm_property(stringify!(#field_name))).flatten() {
                                                       properties.push(res.clone());
                                                   }
                                               }
                                           }
                                       }
                                   });

            let expanded = quote! {
                impl #name {
                    pub fn to_cdrm_property_stream(&self) -> String {
                        let mut properties: Vec<String> = Vec::new();
                        #(#field_actions)*
                        properties.join(" ")
                    }
                }
            };
            return expanded.into();
        }
    };
    quote!().into()
}
