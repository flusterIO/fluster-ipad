//! # Conundrum Server
//!
//! As part of the [Conundrum ecosystem](https://flusterapp.com) of academic tools, the Conundrum server connects the
//! dots, providing your server scale LLM with all of the tools it needs to help
//! you reach your academic goals, while giving you local scale fallbacks when
//! working offline or just trying to reduce your token expenditure.
pub mod mcp;
mod rest;
mod rig;
mod routes;
mod rpc;
pub mod run_server;
