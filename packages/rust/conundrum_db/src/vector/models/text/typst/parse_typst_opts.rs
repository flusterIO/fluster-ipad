use fake::Dummy;
use serde::{Deserialize, Serialize};
use serde_with::SerializeDisplay;

#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
pub struct ParseTypstOpts {}
