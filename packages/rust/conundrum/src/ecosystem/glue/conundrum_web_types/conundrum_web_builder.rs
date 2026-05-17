use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Plans
/// - [ ] Next
/// - [ ] Vite
///
/// ### Planned server providers doing basically the same thing
/// This probably seems like a lot for someone so far behind, but thanks to the
/// amazing FFI libraries in Rust this will actually be really straight forward.
///
/// - [ ] Node
/// - [ ] Rust
/// - [ ] Go
/// - [ ] Python
#[derive(Serialize, Default, Deserialize, strum_macros::Display, Clone, Eq, PartialEq, JsonSchema)]
pub enum ConundrumWebProjectBuilder {
    /// When using the **Next.js** provider, your output should point to a file.
    /// If it points to a directory, a single `cdrm_output.json` file will
    /// be generated in that directory.
    ///
    /// Unfortunately, there currently isn't support for multiple files. I plan
    /// to get around to it, but my to-do list is _long_.
    #[serde(rename = "next")]
    #[default]
    Next,
}
