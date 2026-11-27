use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Dummy, specta::Type)]
pub struct Duration(f32);
