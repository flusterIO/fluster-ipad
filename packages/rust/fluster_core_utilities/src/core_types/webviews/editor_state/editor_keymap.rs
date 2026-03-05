use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uniffi::Enum;

// TODO: Override the Swift enum with this now that all editor state is being lifted to Rust, at
// least for typing & codegen.
#[typeshare]
#[derive(Enum, strum_macros::Display, Serialize, Deserialize)]
pub enum CodeEditorKeymap {
    #[serde(rename = "vim")]
    #[strum(to_string = "vim")]
    Vim,
    #[serde(rename = "base")]
    #[strum(to_string = "base")]
    Base,
    #[serde(rename = "emacs")]
    #[strum(to_string = "emacs")]
    Emacs,
}

