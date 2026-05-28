pub trait IconRepresentable {
    /// Returns a boolean indicating if a specific variant is an icon.
    fn is_icon(&self) -> bool {
        todo!()
    }
    /// This _must_ return a lucide icon or a `char`. The reason it doesn't
    /// return an `Icon` directly is just to allow for non-exhaustive
    /// matches in Enums where some variants will return an icon and others
    /// wont.
    fn as_icon(&self) -> char {
        todo!()
    }
}
