use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::database::model::{
    field_options::FieldOptions,
    parser_functions::{extract_inner_type, is_option},
    utils::MacroUtils,
};

#[derive(Clone, Debug)]
pub struct ModelField {
    pub ident: syn::Ident,

    pub key: String,

    pub ty: syn::Type,

    pub inner_ty: syn::Type,

    pub nullable: bool,

    pub options: FieldOptions,
}

impl PartialEq<ModelField> for ModelField {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident
        && self.key == other.key
        && self.ty == other.ty
        && self.inner_ty == other.inner_ty
        && self.nullable == other.nullable
    }
}

impl TryFrom<&Field> for ModelField {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        let ident = field.ident
                         .clone()
                         .ok_or_else(|| syn::Error::new_spanned(field, "database models require named fields"))?;

        let ty = field.ty.clone();
        let inner_ty = extract_inner_type(&ty);
        let options = FieldOptions::parse(&field.attrs)?;

        let nullable = options.nullable.unwrap_or_else(|| is_option(&ty));

        let key = options.rename.clone().unwrap_or_else(|| ident.to_string());

        Ok(Self { ident,
                  key,
                  ty,
                  inner_ty,
                  nullable,
                  options })
    }
}
impl ModelField {
    pub fn is_skipped(&self) -> bool {
        self.options.skip
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    pub fn partial_included(&self) -> bool {
        !self.options.partial.skip
    }

    pub fn partial_required(&self) -> bool {
        self.options.partial.required
    }

    pub fn partial_patch(&self) -> bool {
        self.options.partial.patch
    }

    pub fn database_key(&self) -> &str {
        &self.key
    }

    pub fn field_arrow_type(&self) -> TokenStream {
        if let Some(res) = self.options.arrow_override.clone() {
            quote! {
                #res
            }
        } else {
            let t = self.ty.clone();
            quote! {
                #t
            }
        }
    }

    fn partial_field_macros(&self, is_primary_field: bool) -> Vec<TokenStream> {
        let mut data = vec![];
        if is_primary_field {
            data.push(quote! {
                          primary
                      });
        }

        if self.options.partial.skip_arrow {
            data.push(quote! {
                          skip_arrow
                      });
        }
        data
    }

    pub fn serde_macro(&self) -> TokenStream {
        if let Some(default_value) = &self.options.partial.default {
            quote! {
                #[serde(default = #default_value)]
            }
        } else {
            quote! {}
        }
    }

    pub fn partial_field_macros_tokens(&self, is_primary_field: bool) -> TokenStream {
        let s = self.partial_field_macros(is_primary_field);
        match s.len() {
            0 => {
                quote! {}
            }
            _ => {
                let t = MacroUtils::comma_separated_list(s);
                quote! {
                    #[db(#t)]
                }
            }
        }
    }
}
