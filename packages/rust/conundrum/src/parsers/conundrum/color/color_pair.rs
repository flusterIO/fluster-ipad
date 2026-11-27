use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Dummy, specta::Type)]
pub struct ColorPair<T>
    where T: Clone {
    pub background: T,
    pub foreground: T,
}
