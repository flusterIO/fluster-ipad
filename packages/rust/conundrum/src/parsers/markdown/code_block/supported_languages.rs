use serde::{Deserialize, Serialize};
use strum::EnumIter;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use winnow::error::ErrMode;

use crate::lang::{
    lib::ui::ui_traits::jsx_prop_representable::JsxPropRepresentable,
    runtime::state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
    },
};

/// All keys must be cast to lowercase and all `_` replaced with `-`.
#[typeshare::typeshare]
#[derive(Serialize,
           Deserialize,
           strum_macros::Display,
           strum_macros::EnumString,
           EnumIter,
           uniffi::Enum,
           Clone,
           Debug)]
pub enum SupportedCodeBlockSyntax {
    #[serde(rename = "Plain Text")]
    #[strum(to_string = "Plain Text", serialize = "text", serialize = "txt")]
    PlainText,
    #[serde(rename = "ASP")]
    #[strum(to_string = "ASP", serialize = "asp")]
    ASP,
    #[serde(rename = "HTML (ASP)")]
    #[strum(to_string = "HTML (ASP)", serialize = "html-asp")]
    #[allow(non_camel_case_types)]
    HTML_ASP,
    #[serde(rename = "ActionScript")]
    #[strum(to_string = "ActionScript", serialize = "actionscript", serialize = "action-script")]
    ActionScript,
    #[serde(rename = "AppleScript")]
    #[strum(to_string = "AppleScript", serialize = "applescript")]
    AppleScript,
    #[serde(rename = "Batch File")]
    #[strum(to_string = "Batch File", serialize = "batchfile", serialize = "batch-file")]
    BatchFile,
    #[serde(rename = "NAnt Build File")]
    #[strum(to_string = "NAnt Build File", serialize = "nantbuild", serialize = "nantbuildfile")]
    NAntBuildFile,
    #[serde(rename = "C#")]
    #[strum(to_string = "C#", serialize = "c#", serialize = "csharp")]
    CSharp,
    #[serde(rename = "C++")]
    #[strum(to_string = "C++", serialize = "cpp", serialize = "c++")]
    Cpp,
    #[serde(rename = "C")]
    #[strum(to_string = "C", serialize = "c")]
    C,
    #[serde(rename = "CSS")]
    #[strum(to_string = "CSS", serialize = "css")]
    CSS,
    #[serde(rename = "Clojure")]
    #[strum(to_string = "Clojure", serialize = "clojure")]
    Clojure,
    #[serde(rename = "D")]
    #[strum(to_string = "D", serialize = "d")]
    D,
    #[serde(rename = "Diff")]
    #[strum(to_string = "Diff", serialize = "diff")]
    Diff,
    #[serde(rename = "Erlang")]
    #[strum(to_string = "Erlang", serialize = "erlang")]
    Erlang,
    #[serde(rename = "HTML (Erlang)")]
    #[strum(to_string = "HTML (Erlang)", serialize = "html-erlang")]
    #[allow(non_camel_case_types)]
    HTML_Erlang,
    #[serde(rename = "Go")]
    #[strum(to_string = "Go", serialize = "go")]
    Go,
    #[serde(rename = "Graphviz (DOT)")]
    #[strum(to_string = "Graphviz (DOT)", serialize = "graphiv", serialize = "graphviz-dot", serialize = "graphvizdot")]
    GraphvizDOT,
    #[serde(rename = "Groovy")]
    #[strum(to_string = "Groovy", serialize = "groovy")]
    Groovy,
    #[serde(rename = "HTML")]
    #[strum(to_string = "HTML", serialize = "html")]
    HTML,
    #[serde(rename = "Haskell")]
    #[strum(to_string = "Haskell", serialize = "haskell")]
    Haskell,
    #[serde(rename = "Literate Haskell")]
    #[strum(to_string = "Literate Haskell", serialize = "literate-haskell", serialize = "literatehaskell")]
    LiterateHaskell,
    #[serde(rename = "Java Server Page (JSP)")]
    #[strum(to_string = "Java Server Page (JSP)", serialize = "jsp", serialize = "java-server-page")]
    JavaServerPage,
    #[serde(rename = "Java")]
    #[strum(to_string = "Java", serialize = "java")]
    Java,
    #[serde(rename = "JavaDoc")]
    #[strum(to_string = "JavaDoc", serialize = "javadoc")]
    JavaDoc,
    #[serde(rename = "Java Properties")]
    #[strum(to_string = "Java Properties", serialize = "java-properties", serialize = "java-props")]
    JavaProperties,
    #[serde(rename = "JSON")]
    #[strum(to_string = "JSON", serialize = "json")]
    JSON,
    #[serde(rename = "JavaScript")]
    #[strum(to_string = "JavaScript", serialize = "js", serialize = "javascript")]
    JavaScript,
    #[serde(rename = "Regular Expressions (Javascript)")]
    #[strum(to_string = "Regular Expressions (Javascript)", serialize = "regex-js", serialize = "regex-javascript")]
    RegexJs,
    #[serde(rename = "BibTeX")]
    #[strum(to_string = "BibTeX", serialize = "bibtex")]
    BibTeX,
    #[serde(rename = "LaTeX Log")]
    #[strum(to_string = "LaTeX Log", serialize = "latex-log", serialize = "latexlog")]
    LaTeXLog,
    #[serde(rename = "LaTeX")]
    #[strum(to_string = "LaTeX", serialize = "latex")]
    LaTeX,
    #[serde(rename = "TeX")]
    #[strum(to_string = "TeX", serialize = "tex")]
    TeX,
    #[serde(rename = "Lisp")]
    #[strum(to_string = "Lisp", serialize = "lisp")]
    Lisp,
    #[serde(rename = "Lua")]
    #[strum(to_string = "Lua", serialize = "lua")]
    Lua,
    #[serde(rename = "Make Output")]
    #[strum(to_string = "Make Output", serialize = "make-output", serialize = "makeoutput")]
    MakeOutput,
    #[serde(rename = "Makefile")]
    #[strum(to_string = "Makefile", serialize = "makefile")]
    Makefile,
    #[serde(rename = "Markdown")]
    #[strum(to_string = "Markdown", serialize = "markdown", serialize = "md")]
    Markdown,
    #[serde(rename = "MultiMarkdown")]
    #[strum(to_string = "MultiMarkdown", serialize = "multi-markdown", serialize = "multi-md")]
    MultiMarkdown,
    #[serde(rename = "MATLAB")]
    #[strum(to_string = "MATLAB", serialize = "matlab")]
    MATLAB,
    #[serde(rename = "OCaml")]
    #[strum(to_string = "OCaml", serialize = "ocaml")]
    OCaml,
    #[serde(rename = "OCamllex")]
    #[strum(to_string = "OCamllex", serialize = "ocamllex")]
    OCamllex,
    #[serde(rename = "OCamlyacc")]
    #[strum(to_string = "OCamlyacc", serialize = "ocamlyacc")]
    OCamlyacc,
    #[serde(rename = "camlp4")]
    #[strum(to_string = "camlp4")]
    Camlp4,
    #[serde(rename = "Objective-C++")]
    #[strum(to_string = "Objective-C++", serialize = "objective-cpp", serialize = "objective-c++")]
    ObjectiveCpp,
    #[serde(rename = "Objective-C")]
    #[strum(to_string = "Objective-C", serialize = "objective-c")]
    ObjectiveC,
    #[serde(rename = "PHP Source")]
    #[strum(to_string = "PHP Source", serialize = "php-source")]
    PHPSource,
    #[serde(rename = "PHP")]
    #[strum(to_string = "PHP", serialize = "php")]
    PHP,
    #[serde(rename = "Pascal")]
    #[strum(to_string = "Pascal", serialize = "pascal")]
    Pascal,
    #[serde(rename = "Perl")]
    #[strum(to_string = "Perl", serialize = "perl")]
    Perl,
    #[serde(rename = "Python")]
    #[strum(to_string = "Python", serialize = "python", serialize = "py")]
    Python,
    #[serde(rename = "Regular Expressions (Python)")]
    #[strum(to_string = "Regular Expressions (Python)", serialize = "regex-py", serialize = "regex-python")]
    RegexPython,
    #[serde(rename = "R Console")]
    #[strum(to_string = "R Console", serialize = "r-console")]
    RConsole,
    #[serde(rename = "R")]
    #[strum(to_string = "R", serialize = "r")]
    R,
    #[serde(rename = "Rd (R Documentation)")]
    #[strum(to_string = "Rd (R Documentation)", serialize = "rdoc", serialize = "r-doc", serialize = "r-documentation")]
    Rdoc,
    #[serde(rename = "HTML (Rails)")]
    #[strum(to_string = "HTML (Rails)", serialize = "html-rails", serialize = "htmlrails")]
    #[allow(non_camel_case_types)]
    HTML_Rails,
    #[serde(rename = "JavaScript (Rails)")]
    #[strum(to_string = "JavaScript (Rails)",
            serialize = "jsrails",
            serialize = "js-rails",
            serialize = "javascript-rails",
            serialize = "javascriptrails")]
    #[allow(non_camel_case_types)]
    JavaScript_Rails,
    #[serde(rename = "Ruby Haml")]
    #[strum(to_string = "Ruby Haml", serialize = "ruby-haml", serialize = "rubyhaml")]
    RubyHaml,
    #[serde(rename = "Ruby on Rails")]
    #[strum(to_string = "Ruby on Rails", serialize = "rubyonrails", serialize = "ruby-on-rails", serialize = "rails")]
    RubyOnRails,
    #[serde(rename = "SQL (Rails)")]
    #[strum(to_string = "SQL (Rails)", serialize = "sql-rails", serialize = "sqlrails")]
    #[allow(non_camel_case_types)]
    SQL_Rails,
    #[serde(rename = "Regular Expression")]
    #[strum(to_string = "Regular Expression", serialize = "regular-expression", serialize = "regex")]
    Regex,
    #[serde(rename = "reStructuredText")]
    #[strum(to_string = "reStructuredText", serialize = "restructuredtext", serialize = "restructured-text")]
    ReStructuredText,
    #[serde(rename = "Ruby")]
    #[strum(to_string = "Ruby", serialize = "ruby")]
    Ruby,
    #[serde(rename = "Cargo Build Results")]
    #[strum(to_string = "Cargo Build Results",
            serialize = "cargo-build-results",
            serialize = "cargo-build-output",
            serialize = "cargo-build",
            serialize = "cargobuildresults",
            serialize = "cargobuildoutput",
            serialize = "cargobuild")]
    CargoBuildResults,
    #[serde(rename = "Rust")]
    #[strum(to_string = "Rust", serialize = "rust", serialize = "rs")]
    Rust,
    #[serde(rename = "SQL")]
    #[strum(to_string = "SQL", serialize = "sql")]
    SQL,
    #[serde(rename = "Scala")]
    #[strum(to_string = "Scala", serialize = "scalar")]
    Scala,
    #[serde(rename = "Bourne Again Shell (bash)")]
    #[strum(to_string = "Bourne Again Shell (bash)", serialize = "bash")]
    Bash,
    #[serde(rename = "Shell-Unix-Generic")]
    #[strum(to_string = "Shell-Unix-Generic", serialize = "unix-shell", serialize = "unixshell")]
    GenericUnixShell,
    #[serde(rename = "commands-builtin-shell-bash")]
    #[strum(to_string = "commands-builtin-shell-bash")]
    CommandsBuiltinShellBash,
    #[serde(rename = "HTML (Tcl)")]
    #[strum(to_string = "HTML (Tcl)", serialize = "html-tcl", serialize = "htmltcl")]
    #[allow(non_camel_case_types)]
    HTML_TCL,
    #[serde(rename = "Tcl")]
    #[strum(to_string = "Tcl", serialize = "tcl")]
    Tcl,
    #[serde(rename = "Textile")]
    #[strum(to_string = "Textile", serialize = "textile")]
    Textile,
    #[serde(rename = "XML")]
    #[strum(to_string = "XML", serialize = "xml")]
    XML,
    #[serde(rename = "YAML")]
    #[strum(to_string = "YAML", serialize = "yaml")]
    YAML,
    // Conundrum specific blocks
    #[serde(rename = "conundrum-ai")]
    #[strum(to_string = "conundrum-ai", serialize = "fluster-ai")]
    ConundrumAi,
    #[serde(rename = "dictionary")]
    #[strum(to_string = "dictionary")]
    Dictionary,
}

impl SupportedCodeBlockSyntax {
    pub fn format_string_for_key(val: &str) -> String {
        val.trim().to_lowercase().replace("_", "-")
    }

    pub fn load_syntax(&self, ss: &SyntaxSet) -> ConundrumModalResult<SyntaxReference> {
        ss.find_syntax_by_name(self.to_string().as_str()).cloned().ok_or_else(|| {
            ErrMode::Cut(
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Parser Error", format!("The `{}` language grammar could not be loaded. If this continues please file an issue on Github.", self).as_str()))
            )
        })
    }

    /// Since markdown rendering is completely left up to the platform, and with
    /// that, the languages they support and the keys that they use, this
    /// function attempts to sacrifice some highlighter accuracy for some
    /// more generic syntaxes that are more likely to be
    /// supported elsewhere.
    pub fn markdown_representation(&self) -> String {
        match self {
            SupportedCodeBlockSyntax::PlainText => "text".to_string(),
            SupportedCodeBlockSyntax::ASP => "asp".to_string(),
            SupportedCodeBlockSyntax::HTML_ASP => "html".to_string(),
            SupportedCodeBlockSyntax::ActionScript => "actionscript".to_string(),
            SupportedCodeBlockSyntax::AppleScript => "applescript".to_string(),
            SupportedCodeBlockSyntax::BatchFile => "batchfile".to_string(),
            SupportedCodeBlockSyntax::NAntBuildFile => "nantbuildfile".to_string(),
            SupportedCodeBlockSyntax::CSharp => "c#".to_string(),
            SupportedCodeBlockSyntax::Cpp => "c++".to_string(),
            SupportedCodeBlockSyntax::C => "c".to_string(),
            SupportedCodeBlockSyntax::CSS => "css".to_string(),
            SupportedCodeBlockSyntax::Clojure => "clojure".to_string(),
            SupportedCodeBlockSyntax::D => "d".to_string(),
            SupportedCodeBlockSyntax::Diff => "diff".to_string(),
            SupportedCodeBlockSyntax::Erlang => "erlang".to_string(),
            SupportedCodeBlockSyntax::HTML_Erlang => "html".to_string(),
            SupportedCodeBlockSyntax::Go => "go".to_string(),
            SupportedCodeBlockSyntax::GraphvizDOT => "graphviz".to_string(),
            SupportedCodeBlockSyntax::Groovy => "groovy".to_string(),
            SupportedCodeBlockSyntax::HTML => "html".to_string(),
            SupportedCodeBlockSyntax::Haskell => "haskell".to_string(),
            SupportedCodeBlockSyntax::LiterateHaskell => "haskell".to_string(),
            SupportedCodeBlockSyntax::JavaServerPage => "java".to_string(),
            SupportedCodeBlockSyntax::Java => "java".to_string(),
            SupportedCodeBlockSyntax::JavaDoc => "javadoc".to_string(),
            SupportedCodeBlockSyntax::JavaProperties => "java-properties".to_string(),
            SupportedCodeBlockSyntax::JSON => "json".to_string(),
            SupportedCodeBlockSyntax::JavaScript => "javascript".to_string(),
            SupportedCodeBlockSyntax::RegexJs => "regexp".to_string(),
            SupportedCodeBlockSyntax::BibTeX => "bibtex".to_string(),
            SupportedCodeBlockSyntax::LaTeXLog => "latexlog".to_string(),
            SupportedCodeBlockSyntax::LaTeX => "latex".to_string(),
            SupportedCodeBlockSyntax::TeX => "tex".to_string(),
            SupportedCodeBlockSyntax::Lisp => "lisp".to_string(),
            SupportedCodeBlockSyntax::Lua => "lua".to_string(),
            SupportedCodeBlockSyntax::MakeOutput => "shell".to_string(),
            SupportedCodeBlockSyntax::Makefile => "make".to_string(),
            SupportedCodeBlockSyntax::Markdown => "markdown".to_string(),
            SupportedCodeBlockSyntax::MultiMarkdown => "markdown".to_string(),
            SupportedCodeBlockSyntax::MATLAB => "matlab".to_string(),
            SupportedCodeBlockSyntax::OCaml => "ocaml".to_string(),
            SupportedCodeBlockSyntax::OCamllex => "ocamllex".to_string(),
            SupportedCodeBlockSyntax::OCamlyacc => "ocamlyacc".to_string(),
            SupportedCodeBlockSyntax::Camlp4 => "camlp4".to_string(),
            SupportedCodeBlockSyntax::ObjectiveCpp => "objective-cpp".to_string(),
            SupportedCodeBlockSyntax::ObjectiveC => "objective-c".to_string(),
            SupportedCodeBlockSyntax::PHPSource => "php-source".to_string(),
            SupportedCodeBlockSyntax::PHP => "php".to_string(),
            SupportedCodeBlockSyntax::Pascal => "pascal".to_string(),
            SupportedCodeBlockSyntax::Perl => "perl".to_string(),
            SupportedCodeBlockSyntax::Python => "python".to_string(),
            SupportedCodeBlockSyntax::RegexPython => "regexp".to_string(),
            SupportedCodeBlockSyntax::RConsole => "r-console".to_string(),
            SupportedCodeBlockSyntax::R => "r".to_string(),
            SupportedCodeBlockSyntax::Rdoc => "rdoc".to_string(),
            SupportedCodeBlockSyntax::HTML_Rails => "html".to_string(),
            SupportedCodeBlockSyntax::JavaScript_Rails => "javascript".to_string(),
            SupportedCodeBlockSyntax::RubyHaml => "html".to_string(),
            SupportedCodeBlockSyntax::RubyOnRails => "rubyonrails".to_string(),
            SupportedCodeBlockSyntax::SQL_Rails => "sql".to_string(),
            SupportedCodeBlockSyntax::Regex => "regexp".to_string(),
            SupportedCodeBlockSyntax::ReStructuredText => "restructuredtext".to_string(),
            SupportedCodeBlockSyntax::Ruby => "ruby".to_string(),
            SupportedCodeBlockSyntax::CargoBuildResults => "shell".to_string(),
            SupportedCodeBlockSyntax::Rust => "rust".to_string(),
            SupportedCodeBlockSyntax::SQL => "sql".to_string(),
            SupportedCodeBlockSyntax::Scala => "scala".to_string(),
            SupportedCodeBlockSyntax::Bash => "bash".to_string(),
            SupportedCodeBlockSyntax::GenericUnixShell => "shell".to_string(),
            SupportedCodeBlockSyntax::CommandsBuiltinShellBash => "shell".to_string(),
            SupportedCodeBlockSyntax::HTML_TCL => "html".to_string(),
            SupportedCodeBlockSyntax::Tcl => "tcl".to_string(),
            SupportedCodeBlockSyntax::Textile => "textile".to_string(),
            SupportedCodeBlockSyntax::XML => "xml".to_string(),
            SupportedCodeBlockSyntax::YAML => "yaml".to_string(),
            SupportedCodeBlockSyntax::ConundrumAi => "text".to_string(),
            SupportedCodeBlockSyntax::Dictionary => "text".to_string(),
        }
    }
}

impl JsxPropRepresentable for SupportedCodeBlockSyntax {
    fn to_jsx_prop(&self, key: &str) -> String {
        format!("{}=\"{}\"", key, self)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn gets_basic_languages() {
        SupportedCodeBlockSyntax::from_str("javascript").expect("Parses alias successfully.");
        // assert_eq!(result, 4);
    }
}
