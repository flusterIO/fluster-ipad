use crate::lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState};

pub trait DataAttributeRepresentable {
    /// Returns the ***complete*** data-attribute, with both the property and
    /// key.
    fn as_html_data_attribute(&self) -> String {
        todo!()
    }
}

pub trait DataAttributeRepresentableStateful {
    /// Returns the ***complete*** data-attribute, with both the property and
    /// key.
    fn to_html_data_attribute_with_state(&self, state: ArcState) -> ConundrumModalResult<String> {
        todo!()
    }
}
