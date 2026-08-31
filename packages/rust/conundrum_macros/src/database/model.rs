use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Field, GenericArgument, PathArguments, Type};

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

#[derive(Clone, Debug)]
pub struct PartialOptions {
    pub skip: bool,
    pub required: bool,
    pub patch: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for PartialOptions {
    fn default() -> Self {
        Self { skip: false,
               required: false,
               patch: false }
    }
}

#[derive(Clone, Debug)]
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

#[allow(clippy::derivable_impls)]
impl Default for FieldOptions {
    fn default() -> Self {
        Self { skip: false,
               rename: None,
               nullable: None,
               with: None,
               partial: PartialOptions::default(),
               primary: false }
    }
}

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

#[derive(Clone, Debug)]
pub struct UnitModel {
    /// The type explicitly specified by:
    ///
    /// #[db(unit = TextBasedChunk)]
    pub path: syn::Path,

    /// The actual type contained by the tuple/newtype field.
    pub ty: syn::Type,
}

impl UnitModel {
    pub fn ident(&self) -> Option<&syn::Ident> {
        self.path.get_ident()
    }

    pub fn path(&self) -> &syn::Path {
        &self.path
    }

    pub fn ty(&self) -> &syn::Type {
        &self.ty
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub ident: syn::Ident,
    pub generics: syn::Generics,

    /// True if the model is held in the `conundrum` crate, false if held in
    /// any crate that imports conundrum.
    pub in_source_crate: bool,

    pub table: Option<syn::Path>,
    pub partial: Option<syn::Path>,
    pub fields: Vec<ModelField>,

    /// The underlying type when this model is a newtype.
    pub unit: Option<UnitModel>,
}

impl Model {
    pub fn primary_field_with_nested_type_fallback(&self) -> TokenStream {
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

        models
            .iter()
            .find(|model| model.ident == *unit_ident)
            .ok_or_else(|| {
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

    /// Construct a Model.
    ///
    /// `models` must contain the DeriveInput definitions for all models that
    /// can be referenced by #[db(unit = ...)].
    ///
    /// For an ordinary model:
    ///
    ///     Model::from_input(input, &models)
    ///
    /// extracts its own fields.
    ///
    /// For:
    ///
    ///     #[db(unit = TextBasedChunk)]
    ///     struct CdrmChunk(TextBasedChunk);
    ///
    /// extracts TextBasedChunk's fields and stores them directly in
    /// `Model.fields`.
    pub fn from_input(input: syn::DeriveInput, models: &[syn::DeriveInput]) -> Result<Self, syn::Error> {
        let options = ModelOptions::parse(&input.attrs)?;

        let syn::Data::Struct(data) = &input.data else {
            return Err(syn::Error::new_spanned(&input.ident, "DatabaseEntity can only be derived for structs"));
        };

        let unit = Self::extract_unit(&data.fields, options.unit.as_ref())?;

        let fields = match &unit {
            Some(unit) => Self::extract_unit_fields(&unit.path, models)?,

            None => Self::extract_named_fields(&data.fields)?,
        };

        Ok(Self { ident: input.ident,
                  generics: input.generics,
                  in_source_crate: options.in_source_crate,
                  table: options.table,
                  partial: options.partial,
                  fields,
                  unit })
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

    pub fn partial_type_or_self(&self) -> Result<syn::Ident, syn::Error> {
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

        self.fields
            .iter()
            .find(|field| field.ident == "id")
            .ok_or_else(|| {
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

        let partial_name = syn::Ident::new(&format!("{struct_name}Partial"), struct_name.span());
        partial_name
    }
}

pub struct ModelOptions {
    pub table: Option<syn::Path>,
    pub in_source_crate: bool,
    pub partial: Option<syn::Path>,
    pub unit: Option<syn::Path>,
}

impl ModelOptions {
    fn parse(attrs: &[syn::Attribute]) -> Result<Self, syn::Error> {
        let mut table = None;
        let mut partial = None;
        let mut in_source_crate = false;
        let mut unit = None;

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

                    if meta.path.is_ident("source_crate") {
                        let value: syn::LitBool = meta.value()?.parse()?;
                        in_source_crate = value.value();
                        return Ok(());
                    }

                    if meta.path.is_ident("unit") {
                        let value: syn::Path = meta.value()?.parse()?;
                        unit = Some(value);
                        return Ok(());
                    }

                    if meta.path.is_ident("partial") {
                        let value: syn::Path = meta.value()?.parse()?;
                        partial = Some(value);
                        return Ok(());
                    }

                    let name =
                        meta.path.get_ident().map(|ident| ident.to_string()).unwrap_or_else(|| "unknown".to_string());

                    Err(meta.error(format!("unknown database model option: `{}`", name)))
                })?;
        }

        // let table =
        //     table;

        Ok(Self { table,
                  in_source_crate,
                  partial,
                  unit })
    }
}

/// Extract fields from a normal struct.
///
/// This is kept as a public helper because the macro's model collection
/// layer may need to parse models before resolving #[db(unit = ...)].
pub fn extract_model_fields(data: &syn::DataStruct) -> Result<Vec<ModelField>, syn::Error> {
    Model::extract_named_fields(&data.fields)
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

                    if type_path.path != unit {
                        return Err(syn::Error::new_spanned(field,
                                                           format!("wrapped type `{}` does not match #[db(unit = {})]",
                                                                   quote::quote!(#type_path),
                                                                   quote::quote!(#unit),)));
                    }

                    (Vec::new(),
                     Some(UnitModel { path: unit,
                                      ty: field.ty.clone() }))
                }

                syn::Fields::Unit => {
                    return Err(syn::Error::new_spanned(ident, "database models cannot be Rust unit structs"));
                }
            },

            _ => {
                return Err(syn::Error::new_spanned(ident, "DatabaseEntity can only be derived for structs"));
            }
        };

        Ok(Self { ident,
                  generics,
                  in_source_crate: options.in_source_crate,
                  table: options.table,
                  partial: options.partial,
                  fields,
                  unit })
    }
}
