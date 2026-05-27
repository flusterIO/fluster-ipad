use std::str::FromStr;

use log::{Level, Metadata, Record};
use once_cell::sync::Lazy;

use crate::ecosystem::environment_variables::cdrm_env_variable::CdrmEnvVariable;

/// ## Conundrum Logging & Error Handling
///
/// A simple logger that's mostly used for the Conundrum cli output.
/// Most notifications passed back to the user are actually passed back through
/// the `MdxParsingResult` struct as 'warnings' and are intended to be handled
/// by the developer before being displayed to the user in a more non-developer
/// friendly format. There's a place for console output, and if that fits your
/// target audience than do your thing, but a a framework aiming to be usable by
/// people that never intend to become full-time developers, I think we can do
/// better in most cases with something like  toast or a modal.
///
/// Since all of the errors are type safe and generated across the language
/// barrier for all the languages that Conundrum intends to support (pretty much
/// any language with decent support for Rust FFI) we as developers should be
/// able to use th this information to pull off some pretty cool, high level
/// features. That's one of the benefits of being a markdown-first language...
/// it's ***simple***. There aren't nearly as many moving pieces as html/js/css
/// and that will never change. The component list might grow, but the
/// complexity shouldn't. Because of this simplicity I'm able to produce pretty
/// reasonable error messages without having even implemented any real error
/// handling yet. Once something like `miette` is in place with proper `Spans`
/// being returned, you as a developer should access to the exact location and
/// cause of an error in _most_ cases. That's only possible because of the
/// simplicity of markdown, allowing the language to represent most of the
/// entire language as unique types, integrating the parser directly with each
/// component.
///
/// In short, the error messages right now aren't great, but we're in a position
/// to have error handling in Conundrum be on par with the best LSP's available.
pub struct Logger {
    level: Level,
}

impl Default for Logger {
    fn default() -> Self {
        let level = CdrmEnvVariable::LogLevel.read()
                                             .map(|res| Level::from_str(res.as_str()).unwrap_or_else(|_| Level::Info))
                                             .unwrap_or_else(|_| Level::Info);
        Logger { level }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub static LOGGER: Lazy<Logger> = Lazy::new(Logger::default);
