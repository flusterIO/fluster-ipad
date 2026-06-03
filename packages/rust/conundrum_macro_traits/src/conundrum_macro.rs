pub trait ConundrumMacroProperty {
    /// Returns the property **and** key as it appears in a conundrum component.
    ///
    /// #### Example
    ///
    /// ```rs
    /// fn to_cdrm_property(&self, property_key: String) -> Result<String, E> {
    ///    String::from("{}={}", property_key, self.value)
    /// }
    /// ```
    fn as_cdrm_property(&self, property_key: &str) -> Option<String>
        where Self: Sized;
}

pub trait ConundrumPropertyMap<E> {
    fn to_cdrm_property_stream(&self) -> Result<String, E>;
}
