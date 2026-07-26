#![recursion_limit = "1024"]
//! ## Package Layout
//!
//! - entites: All of the database logic for a given model.
//! - models: The structure that is actually stored in the database but rejoined
//! to a shape that will make sense for most applications, and
//!   occasionally, some utility
//! methods... usually around type conversion to turn it into something that's
//! actually useful.
pub mod vector;
