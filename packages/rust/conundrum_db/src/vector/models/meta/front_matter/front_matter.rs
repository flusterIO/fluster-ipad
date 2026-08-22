use conundrum::output::parsing_result::front_matter::FrontMatterResult;
use fake::Dummy;
use serde::{Deserialize, Serialize};

/// Still unsure about implementing front matter as a separate model like I did
/// previously. I want to, but I'm sooooooo over writing these models...
#[derive(Serialize, Deserialize, Clone, Debug, specta::Type, Dummy)]
#[serde(transparent)]
pub struct FrontMatter(FrontMatterResult);
