use core::panic;

use syn::{DeriveInput, Error, Field, GenericArgument, PathArguments, Type};

fn is_option(ty: &Type) -> bool {
    let Type::Path(type_path) = ty else {
        return false;
    };

    let Some(segment) = type_path.path.segments.last() else {
        return false;
    };

    segment.ident == "Option"
}

fn extract_inner_type(ty: &Type) -> Type {
    let Type::Path(type_path) = ty else {
        return ty.clone();
    };

    let Some(segment) = type_path.path.segments.last() else {
        return ty.clone();
    };

    if segment.ident != "Option" {
        return ty.clone();
    }

    let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return ty.clone();
    };

    let Some(GenericArgument::Type(inner)) = arguments.args.first() else {
        return ty.clone();
    };

    inner.clone()
}

fn parse_partial(options: &mut PartialOptions, meta: syn::meta::ParseNestedMeta<'_>) -> Result<(), syn::Error> {
    meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                options.skip = true;
                return Ok(());
            }

            if meta.path.is_ident("required") {
                options.required = true;
                return Ok(());
            }

            if meta.path.is_ident("patch") {
                options.patch = true;
                return Ok(());
            }

            Err(meta.error("unknown partial option"))
        })
}

#[derive(Clone)]
pub struct PartialOptions {
    pub skip: bool,
    pub required: bool,
    pub patch: bool,
}

impl Default for PartialOptions {
    fn default() -> Self {
        Self { skip: false,
               required: false,
               patch: false }
    }
}

#[derive(Clone)]
pub struct FieldOptions {
    pub skip: bool,
    pub rename: Option<String>,
    pub nullable: Option<bool>,
    pub with: Option<syn::Path>,
    pub partial: PartialOptions,
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

impl Default for FieldOptions {
    fn default() -> Self {
        Self { skip: false,
               rename: None,
               nullable: None,
               primary: false,
               with: None,
               partial: PartialOptions::default() }
    }
}

#[derive(Clone)]
pub struct ModelField {
    pub ident: syn::Ident,

    pub key: String,

    pub ty: syn::Type,

    pub inner_ty: syn::Type,

    pub nullable: bool,

    pub options: FieldOptions,
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
}

impl TryFrom<&Field> for ModelField {
    type Error = syn::Error;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        let ident =
            field.ident.clone().ok_or_else(|| Error::new_spanned(field, "database models require named fields"))?;

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

pub struct Model {
    pub ident: syn::Ident,
    pub generics: syn::Generics,

    pub table: syn::Path,
    pub partial: Option<syn::Path>,

    pub fields: Vec<ModelField>,
}

impl Model {
    pub fn partial_type_or_self(&self) -> Result<syn::Ident, syn::Error> {
        if let Some(pt) = &self.partial {
            if let Some(ident) = pt.get_ident() {
                Ok(ident.clone())
            } else {
                panic!("Failed to get ident for partial type.");
            }
        } else {
            Ok(self.ident.clone())
        }
    }

    pub fn primary_field(&self) -> Result<&ModelField, syn::Error> {
        let explicit = self.fields.iter().filter(|field| field.options.primary).collect::<Vec<_>>();

        if explicit.len() > 1 {
            return Err(syn::Error::new_spanned(&self.ident, "database model can only have one primary key"));
        }

        if let Some(field) = explicit.first() {
            return Ok(field);
        }

        self.fields
            .iter()
            .find(|field| field.ident == "id")
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    &self.ident,
                    "database model requires an `id` field or an explicit #[database(primary)] field",
                )
            })
    }
}

pub struct ModelOptions {
    pub table: syn::Path,
    pub partial: Option<syn::Path>,
}

impl ModelOptions {
    fn parse(attrs: &[syn::Attribute]) -> Result<Self, syn::Error> {
        let mut table = None;

        let mut partial = None;

        for attr in attrs {
            if !attr.path().is_ident("db") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("table") {
                        let value: syn::Path = meta.value()?.parse()?;
                        table = Some(value);
                        return Ok(());
                    }
                    if meta.path.is_ident("partial") {
                        let value: syn::Path = meta.value()?.parse()?;
                        partial = Some(value);
                        return Ok(());
                    }

                    Err(meta.error("unknown database model option"))
                })?;
        }

        let table = table.ok_or_else(|| {
                             syn::Error::new(proc_macro2::Span::call_site(),
                                             "database model requires #[db(table = ...)]")
                         })?;

        Ok(Self { table,
                  partial })
    }
}

impl TryFrom<syn::DeriveInput> for Model {
    type Error = syn::Error;

    fn try_from(input: syn::DeriveInput) -> Result<Self, Self::Error> {
        let syn::DeriveInput { ident,
                               generics,
                               attrs,
                               data,
                               .. } = input;

        let options = ModelOptions::parse(&attrs)?;

        let fields = match data {
            syn::Data::Struct(data) => data.fields.iter().map(ModelField::try_from).collect::<Result<Vec<_>, _>>()?,

            _ => {
                return Err(syn::Error::new_spanned(ident, "DatabaseModel can only be derived for structs"));
            }
        };

        Ok(Self { ident,
                  generics,
                  table: options.table,
                  partial: options.partial.as_ref().cloned(),
                  fields })
    }
}
