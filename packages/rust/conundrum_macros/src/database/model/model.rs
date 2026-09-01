use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::ops::Index;
use syn::Type;

use crate::database::model::unit_model::UnitModel;
use crate::database::model::{constants::DEFAULT_USE_TYPE_PATH, model_field::ModelField};

#[derive(Debug, Clone)]
pub struct Model {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub where_clause: Option<syn::WhereClause>,

    /// True if the model is held in the `conundrum` crate, false if held in
    /// any crate that imports conundrum.
    pub in_source_crate: bool,

    pub table: Option<syn::Path>,
    pub partial: Option<syn::Path>,
    pub fields: Vec<ModelField>,
    pub opts: ModelOptions,

    /// The underlying type when this model is a newtype.
    pub unit: Option<UnitModel>,
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

        let (fields, unit) = match data {
            syn::Data::Struct(data) => match &data.fields {
                syn::Fields::Named(_) => {
                    let fields = extract_model_fields(&data)?;
                    (fields, None)
                }

                syn::Fields::Unnamed(fields) => {
                    if fields.unnamed.len() != 1 {
                        return Err(syn::Error::new_spanned(fields, "database unit structs require exactly one field"));
                    }

                    let field = fields.unnamed.first().expect("checked that exactly one field exists");

                    let unit =
                        options.unit
                               .clone()
                               .ok_or_else(|| {
                                   syn::Error::new_spanned(field, "tuple database models require #[db(unit = ...)]")
                               })?;

                    let Type::Path(type_path) = &field.ty else {
                        return Err(syn::Error::new_spanned(field, "database unit type must be a type path"));
                    };

                    fn ends_with(unit: syn::Path, type_path: syn::Path) -> bool {
                        let r = format!("{}",
                                        quote! {
                                            #unit
                                        }).split("<")
                                          .map(String::from)
                                          .collect::<Vec<String>>();
                        if r.len() == 1
                           && r.index(0)
                              == format!("{}",
                                         quote! {
                                             #type_path
                                         }).trim()
                        {
                            true
                        } else {
                            false
                        }
                    };

                    if options.use_type_path {
                        (Vec::new(),
                         Some(UnitModel { path: type_path.path.clone(),
                                          ty: field.ty.clone() }))
                    } else {
                        // if type_path.path != unit && !ends_with(unit.clone(), type_path.path.clone())
                        // {     return Err(syn::Error::new_spanned(field,
                        //                                        format!("wrapped type `{}` does not
                        // match #[db(unit = {})]",
                        // quote::quote!(#type_path),
                        // quote::quote!(#unit),))); }
                        (Vec::new(),
                         Some(UnitModel { path: unit,
                                          ty: field.ty.clone() }))
                    }
                }

                syn::Fields::Unit => {
                    return Err(syn::Error::new_spanned(ident, "database models cannot be Rust unit structs"));
                }
            },

            _ => {
                return Err(syn::Error::new_spanned(ident, "DatabaseEntity can only be derived for structs"));
            }
        };

        let o = options.clone();

        Ok(Self { ident,
                  generics: generics.clone(),
                  in_source_crate: o.in_source_crate,
                  table: o.table,
                  partial: o.partial,
                  fields,
                  where_clause: generics.where_clause,
                  opts: options.clone(),
                  unit })
    }
}
#[derive(Debug, Clone)]
pub struct ModelOptions {
    pub table: Option<syn::Path>,
    pub in_source_crate: bool,
    pub partial: Option<syn::Path>,
    pub include_partial_generics: bool,
    pub include_generics: bool,
    pub use_type_path: bool,
    /// True f the partial type is Self.
    pub partial_self: bool,
    pub manual_partial_dummy: bool,
    pub unit: Option<syn::Path>,
}

impl Default for ModelOptions {
    fn default() -> Self {
        Self { table: None,
               in_source_crate: false,
               partial: None,
               include_partial_generics: false,
               include_generics: false,
               use_type_path: false,
               partial_self: false,
               manual_partial_dummy: false,
               unit: None }
    }
}

/// Extract fields from a normal struct.
///
/// This is kept as a public helper because the macro's model collection
/// layer may need to parse models before resolving #[db(unit = ...)].
pub fn extract_model_fields(data: &syn::DataStruct) -> Result<Vec<ModelField>, syn::Error> {
    Model::extract_named_fields(&data.fields)
}

impl ModelOptions {
    fn parse(attrs: &[syn::Attribute]) -> Result<Self, syn::Error> {
        let mut opts = ModelOptions::default();

        for attr in attrs {
            if !attr.path().is_ident("db") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("table") {
                        let value: syn::Path = meta.value()?.parse()?;
                        opts.table = Some(value);
                        return Ok(());
                    }
                    if meta.path.is_ident("include_generics") {
                        let b: bool = meta.value()?.parse().map(|x: syn::LitBool| x.value).unwrap_or(false);
                        opts.include_generics = b;
                        return Ok(());
                    }

                    if meta.path.is_ident("include_partial_generics") {
                        opts.include_partial_generics = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("partial_self") {
                        opts.partial_self = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("manual_partial_dummy") {
                        opts.manual_partial_dummy = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("use_type_path") {
                        let b: bool = meta.value()?
                                          .parse()
                                          .map(|x: syn::LitBool| x.value)
                                          .unwrap_or_else(|_| DEFAULT_USE_TYPE_PATH);
                        opts.use_type_path = b;
                        return Ok(());
                    }

                    if meta.path.is_ident("source_crate") {
                        let value: syn::LitBool = meta.value()?.parse()?;
                        opts.in_source_crate = value.value();
                        return Ok(());
                    }

                    if meta.path.is_ident("unit") {
                        let value: syn::Path = meta.value()?.parse()?;
                        opts.unit = Some(value);
                        return Ok(());
                    }

                    if meta.path.is_ident("partial") {
                        let value: syn::Path = meta.value()?.parse()?;
                        opts.partial = Some(value);
                        return Ok(());
                    }

                    let name =
                        meta.path.get_ident().map(|ident| ident.to_string()).unwrap_or_else(|| "unknown".to_string());

                    Err(meta.error(format!("unknown database model option: `{}`", name)))
                })?;
        }

        Ok(opts)
    }
}

impl Model {
    pub fn primary_field_type_with_nested_type_fallback(&self) -> TokenStream {
        self.primary_field()
            .map(|x| {
                let ty = x.ty.clone();
                quote! {
                  #ty
                }
            })
            .unwrap_or_else(|_| {
                let nested_type =
                    self.unit
                        .as_ref()
                        .cloned()
                        .expect("You must provide a 'unit' value if the struct does not have an id field.")
                        .ty;
                let crate_id = self.conundrum_crate_import();
                quote! {
                    <#nested_type as #crate_id::ecosystem::db::db_traits::db_entity::DBSchema>::IDType
                }
            })
    }

    fn extract_unit(fields: &syn::Fields, unit_path: Option<&syn::Path>) -> Result<Option<UnitModel>, syn::Error> {
        let Some(unit_path) = unit_path else {
            return Ok(None);
        };

        let syn::Fields::Unnamed(fields) = fields else {
            return Err(syn::Error::new_spanned(unit_path, "#[db(unit = ...)] requires a tuple/newtype struct"));
        };

        if fields.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(fields, "#[db(unit = ...)] requires exactly one field"));
        }

        let field = fields.unnamed.first().expect("checked that exactly one field exists");

        let ty = field.ty.clone();

        let Type::Path(type_path) = &ty else {
            return Err(syn::Error::new_spanned(field,
                                               "#[db(unit = ...)] requires the wrapped field to be a type path"));
        };

        if type_path.path != *unit_path {
            return Err(syn::Error::new_spanned(field,
                                               format!("#[db(unit = {})] does not match the wrapped type `{}`",
                                                       quote::quote!(#unit_path),
                                                       quote::quote!(#ty),)));
        }

        Ok(Some(UnitModel { path: unit_path.clone(),
                            ty }))
    }

    /// Extract fields from an ordinary named struct.
    fn extract_named_fields(fields: &syn::Fields) -> Result<Vec<ModelField>, syn::Error> {
        let syn::Fields::Named(fields) = fields else {
            return Err(syn::Error::new(Span::call_site(), "database models require named fields"));
        };

        fields.named.iter().map(ModelField::try_from).collect()
    }

    /// Resolve the DeriveInput corresponding to a #[db(unit = ...)] path.
    ///
    /// The caller must provide the parsed model definitions because a
    /// procedural macro cannot inspect the fields of an arbitrary compiled
    /// Rust type from a syn::Path alone.
    fn resolve_unit<'a>(unit_path: &syn::Path,
                        models: &'a [syn::DeriveInput])
                        -> Result<&'a syn::DeriveInput, syn::Error> {
        let Some(unit_ident) = unit_path.get_ident() else {
            return Err(syn::Error::new_spanned(unit_path,
                                               "#[db(unit = ...)] currently requires a simple type identifier"));
        };

        models.iter().find(|model| model.ident == *unit_ident).ok_or_else(|| {
                                                                  syn::Error::new_spanned(
                                                                                          unit_path,
                                                                                          format!(
                "could not find the definition of unit model `{}`",
                unit_ident
            ),
            )
                                                              })
    }

    /// Extract the fields from the underlying model of a newtype.
    ///
    /// For:
    ///
    ///     pub struct CdrmChunk(TextBasedChunk);
    ///
    /// this returns the ModelField list belonging to TextBasedChunk.
    fn extract_unit_fields(unit_path: &syn::Path, models: &[syn::DeriveInput]) -> Result<Vec<ModelField>, syn::Error> {
        let unit_input = Self::resolve_unit(unit_path, models)?;

        let syn::Data::Struct(unit_struct) = &unit_input.data else {
            return Err(syn::Error::new_spanned(&unit_input.ident,
                                               format!("unit model `{}` must be a struct", unit_input.ident)));
        };

        Self::extract_named_fields(&unit_struct.fields)
    }

    pub fn is_unit(&self) -> bool {
        self.unit.is_some()
    }

    /// Returns the underlying model type when this is a newtype.
    ///
    /// For CdrmChunk this returns TextBasedChunk.
    ///
    /// For a normal model this returns the model's own type.
    pub fn model_type(&self) -> syn::Path {
        self.unit.as_ref().map(|unit| unit.path.clone()).unwrap_or_else(|| syn::Path::from(self.ident.clone()))
    }

    pub fn partial_type_or_partial_name(&self) -> Result<syn::Ident, syn::Error> {
        if let Some(pt) = &self.partial {
            pt.get_ident()
              .cloned()
              .ok_or_else(|| syn::Error::new_spanned(pt, "partial type must be a simple type identifier"))
        } else {
            Ok(self.partial_name())
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

        self.fields.iter().find(|field| field.ident == "id").ok_or_else(|| {
                                                                syn::Error::new_spanned(
                    &self.ident,
                    "database model requires an `id` field or an explicit #[db(primary)] field",
                )
                                                            })
    }

    pub fn conundrum_crate_import(&self) -> syn::PathSegment {
        match self.in_source_crate {
            true => syn::PathSegment::from(syn::Ident::new("crate", Span::call_site())),
            false => syn::PathSegment::from(syn::Ident::new("conundrum", Span::call_site())),
        }
    }

    pub fn table_required(&self) -> Result<syn::Path, syn::Error> {
        self.table
            .as_ref()
            .cloned()
            .ok_or_else(|| syn::Error::new(Span::call_site(), "database model requires #[db(table = ...)]"))
    }

    pub fn as_db_entity_path(&self) -> TokenStream {
        let id_type = self.primary_field().ok();
        let crate_id = self.conundrum_crate_import();
        match id_type {
            Some(um) => {
                let id_type = um.ty.clone();
                quote! {
                    #crate_id::ecosystem::db::db_traits::db_entity
                }
            }
            None => {
                quote! {
                    #crate_id::ecosystem::db::db_traits::db_entity
                }
            }
        }
    }

    pub fn partial_name(&self) -> syn::Ident {
        let struct_name = &self.ident;

        if format!("{}",
                   quote! {
                       #struct_name
                   }).ends_with("Partial")
           || self.opts.partial_self
        {
            return struct_name.clone();
        }

        let partial_name = syn::Ident::new(&format!("{struct_name}Partial"), struct_name.span());
        partial_name
    }

    pub fn partial_dummy(&self) -> TokenStream {
        match self.opts.manual_partial_dummy {
            true => {
                quote! {}
            }
            false => {
                quote! {
                    , fake::Dummy
                }
            }
        }
    }

    pub fn is_partial(&self) -> bool {
        let struct_name = self.ident.clone();
        format!("{}",
                quote! {
                    #struct_name
                }).ends_with("Partial")
    }
}
