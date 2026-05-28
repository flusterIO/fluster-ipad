/// Useful for when the `to_string` property is occupied, but the data still
/// needs to be represented as the shape of the actual data.
pub trait EnumVariantLabel {
    fn to_variant_label(&self) -> String;
}
