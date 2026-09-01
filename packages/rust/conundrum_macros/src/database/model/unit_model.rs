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
