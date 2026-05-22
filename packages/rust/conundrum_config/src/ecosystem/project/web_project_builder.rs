use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// ## Plans
///
/// ### Planned frontend providers
/// - [ ] Next
/// - [ ] Vite
///
/// ### Planned backend providers
/// This will be the core of what makes Conundrum work, AI integrated back-ends
/// that can be dropped into any runtime that supports the following languages.
///
/// To _parse_ Conundrum we need to be able to run Rust (don't worry, it runs
/// almost everywhere. That's a big part of why I chose it.) but to _serve_
/// compiled conundrum, we do not.
///
/// We can rely on the application compiling conundrum at runtime, and just
/// serve conundrum as either raw conundrum content or as compiled content,
/// still offering a significant subset of the framework's features with like
/// first-year-web-developer level complexity.
///
/// Of course docker containers are planed for the future, making this almost drag-and-drop.
///
/// - [ ] Node
/// - [ ] Rust
/// - [ ] Go
/// - [ ] Python
#[derive(Serialize, Default, Deserialize, strum_macros::Display, Clone, Eq, PartialEq, JsonSchema)]
pub enum ProjectFrontendBuilder {
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
