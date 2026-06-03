pub trait CDRMPropertyRepresentable {
    /// Returns the _entire_ property, both key and value as it appears in a
    /// Conundrum component.
    fn to_cdrm_property(&self) -> String;
}

pub trait CDRMPropertyStreamRepresentable {
    /// Returns a string representing a list of properties,  with both key and
    /// value as it appears in a Conundrum component.
    fn to_cdrm_property_stream(&self) -> String;
}
