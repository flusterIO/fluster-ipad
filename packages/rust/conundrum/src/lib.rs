//! # Conundrum
//!
//! Well, it's me again... the creator of [Fluster](https://flusterapp.com). After my initial
//! attempt to open-source the entire project left me just as homeless I decided
//! to make Fluster proprietary, but I want everyone to have access to the
//! transpiler. Compiler? I'm not even sure...
//!
//! This documentation is intended both for developers and for the general
//! user. Currently the app is undergoing a migration where all of the logic
//! pertaining to the language itself is
//! lifted up to this Conundrum crate, including all of the components and
//! likewise the component documentation. Instead of my half-axxed markdown
//! tables, we can now rely on the various Cargo crates for generatng markdown
//! documentation and just _collect_ the generated documentation for the
//! language's built-in documentation feature. Because of that, you might notice
//! that some of the documentation appears out of context on places like
//! rustdoc, but it's intended to be written for a more general-public, in-app
//! experience.
//!
//! ## General Users
//!
//! As mentioned above, much (but not all) of the embedded documentation is
//! generated directly from the code, and since so is the online documentation,
//! there's a _ton_ of overlap with the online content.
//!
//! Still, take a look at the [component's
//! documentation](crate::lang::lib::ui::components), as they will have links to
//! all of the recurring properties like the Sizable struct that you'll use over
//! and over again while writing Conundrum.
//!
//! ## Future Plans
//!
//! The future of Conundrum is coming together **quickly**, with a memory layer
//! right around the corner. At first this will just look like basic math and
//! the ability to assign variables directly to your note content (think
//! inserting the output of a Slider component into your text content or
//! something similar), but within a year I hope to have a language that is
//! fully capable of acting as a first-step, exploratory language that is
//! perfect for the note taking environment.
//!
//! Longer term plans are a little more ambitious. The goal is to create a
//! high-level API in this parent language, Conundrum, that exposes it's memory
//! layer to the nested code blocks through the Conundrum compiler, a sandboxed
//! build environment, and the amazing work people have done around FFI and
//! other cross-language technologies. It will take some time, but we should
//! have an environment where a user can generate a complex array of numbers in
//! C or Rust in 1 code block, and then use that array of numbers in Python in
//! the next code-block to generate a plot, without ever saving anything to
//! disk.

uniffi::setup_scaffolding!();

pub mod embedded;
pub mod lang;
pub mod output;
pub mod parsers;
pub mod testing;
