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
//! application. Because of that, you might notice that some of the
//! documentation appears out of context on places like rustdoc, but it's
//! intended to be written for a more general-public, in-app experience.

uniffi::setup_scaffolding!();

pub mod embedded;
pub mod lang;
pub mod output;
pub mod parsers;
pub mod testing;
