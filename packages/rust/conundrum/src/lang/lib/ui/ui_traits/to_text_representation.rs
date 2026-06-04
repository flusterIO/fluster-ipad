pub trait ToTextRepresentation {
    /// Useful for when an item like a color or an emphasis can be represented
    /// in the output in a multitude of ways, but still needs to be just
    /// described to the user with like a name or something.
    fn to_text_repr(&self) -> String;
}
