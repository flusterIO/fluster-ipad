pub struct PartialOptions {
    pub include: bool,

    pub required: bool,

    pub patch: bool,
}

pub struct FieldOptions {
    pub skip: bool,

    pub rename: Option<String>,

    pub nullable: Option<bool>,

    pub with: Option<syn::Path>,

    pub partial: PartialOptions,
}

pub struct ModelField {
    pub ident: syn::Ident,

    pub key: String,

    pub ty: syn::Type,

    pub inner_ty: syn::Type,

    pub nullable: bool,

    pub options: FieldOptions,
}

pub struct Model {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub fields: Vec<ModelField>,
}
