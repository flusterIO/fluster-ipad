use crate::database::model::{parser_functions::parse_partial, partial_options::PartialOptions};

#[derive(Clone, Debug)]
pub struct FieldOptions {
    pub skip: bool,
    pub rename: Option<String>,
    pub nullable: Option<bool>,
    pub with: Option<syn::Path>,
    pub large: bool,
    pub partial: PartialOptions,
    pub arrow_override: Option<syn::Path>,
    pub skip_arrow: bool,
    pub primary: bool,
}

impl FieldOptions {
    pub fn parse(attrs: &[syn::Attribute]) -> Result<Self, syn::Error> {
        let mut options = Self::default();

        for attr in attrs {
            if !attr.path().is_ident("db") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        options.skip = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("rename") {
                        let value: syn::LitStr = meta.value()?.parse()?;
                        options.rename = Some(value.value());
                        return Ok(());
                    }
                    if meta.path.is_ident("large") {
                        options.large = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("nullable") {
                        let value: syn::LitBool = meta.value()?.parse()?;
                        options.nullable = Some(value.value());
                        return Ok(());
                    }

                    if meta.path.is_ident("with") {
                        let value: syn::Path = meta.value()?.parse()?;
                        options.with = Some(value);
                        return Ok(());
                    }

                    if meta.path.is_ident("arrow") {
                        let value: syn::Path = meta.value()?.parse()?;
                        options.arrow_override = Some(value);
                        return Ok(());
                    }

                    if meta.path.is_ident("skip_arrow") {
                        options.skip_arrow = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("partial") {
                        parse_partial(&mut options.partial, meta)?;
                        return Ok(());
                    }

                    if meta.path.is_ident("primary") {
                        options.primary = true;
                        return Ok(());
                    }

                    Err(meta.error("unknown database attribute"))
                })?;
        }

        Ok(options)
    }
}

#[allow(clippy::derivable_impls)]
impl Default for FieldOptions {
    fn default() -> Self {
        Self { skip: false,
               rename: None,
               nullable: None,
               with: None,
               large: false,
               arrow_override: None,
               skip_arrow: false,
               partial: PartialOptions::default(),
               primary: false }
    }
}
