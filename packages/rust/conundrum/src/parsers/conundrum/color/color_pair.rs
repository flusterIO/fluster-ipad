#[derive(Clone)]
pub struct ColorPair<T>
    where T: Clone {
    pub background: T,
    pub foreground: T,
}
