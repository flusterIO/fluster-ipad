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
    #[serde(rename = "Plain Text", alias = "text", alias = "txt")]
    #[strum(to_string = "Plain Text")]
    PlainText,
    #[serde(rename = "ASP", alias = "asp")]
    #[strum(to_string = "ASP")]
    ASP,
    #[serde(rename = "HTML (ASP)", alias = "html-asp")]
    #[strum(to_string = "HTML (ASP)")]
    #[allow(non_camel_case_types)]
    HTML_ASP,
    #[serde(rename = "ActionScript", alias = "actionscript", alias = "action-script")]
    #[strum(to_string = "ActionScript")]
    ActionScript,
    #[serde(rename = "AppleScript", alias = "applescript")]
    #[strum(to_string = "AppleScript")]
    AppleScript,
    #[serde(rename = "Batch File", alias = "batchfile", alias = "batch-file")]
    #[strum(to_string = "Batch File")]
    BatchFile,
    #[serde(rename = "NAnt Build File", alias = "nantbuild", alias = "nantbuildfile")]
    #[strum(to_string = "NAnt Build File")]
    NAntBuildFile,
    #[serde(rename = "C#", alias = "c#", alias = "csharp")]
    #[strum(to_string = "C#")]
    CSharp,
    #[serde(rename = "C++", alias = "cpp", alias = "c++")]
    #[strum(to_string = "C++")]
    Cpp,
    #[serde(rename = "C", alias = "c")]
    #[strum(to_string = "C")]
    C,
    #[serde(rename = "CSS", alias = "css")]
    #[strum(to_string = "CSS")]
    CSS,
    #[serde(rename = "Clojure", alias = "clojure")]
    #[strum(to_string = "Clojure")]
    Clojure,
    #[serde(rename = "D", alias = "d")]
    #[strum(to_string = "D")]
    D,
    #[serde(rename = "Diff", alias = "diff")]
    #[strum(to_string = "Diff")]
    Diff,
    #[serde(rename = "Erlang", alias = "erlang")]
    #[strum(to_string = "Erlang")]
    Erlang,
    #[serde(rename = "HTML (Erlang)", alias = "html-erlang")]
    #[strum(to_string = "HTML (Erlang)")]
    #[allow(non_camel_case_types)]
    HTML_Erlang,
    #[serde(rename = "Go", alias = "go")]
    #[strum(to_string = "Go")]
    Go,
    #[serde(rename = "Graphviz (DOT)", alias = "graphiv", alias = "graphviz-dot", alias = "graphvizdot")]
    #[strum(to_string = "Graphviz (DOT)")]
    GraphvizDOT,
    #[serde(rename = "Groovy", alias = "groovy")]
    #[strum(to_string = "Groovy")]
    Groovy,
    #[serde(rename = "HTML", alias = "html")]
    #[strum(to_string = "HTML")]
    HTML,
    #[serde(rename = "Haskell", alias = "haskell")]
    #[strum(to_string = "Haskell")]
    Haskell,
    #[serde(rename = "Literate Haskell", alias = "literate-haskell", alias = "literatehaskell")]
    #[strum(to_string = "Literate Haskell")]
    LiterateHaskell,
    #[serde(rename = "Java Server Page (JSP)", alias = "jsp", alias = "java-server-page")]
    #[strum(to_string = "Java Server Page (JSP)")]
    JavaServerPage,
    #[serde(rename = "Java", alias = "java")]
    #[strum(to_string = "Java")]
    Java,
    #[serde(rename = "JavaDoc", alias = "javadoc")]
    #[strum(to_string = "JavaDoc")]
    JavaDoc,
    #[serde(rename = "Java Properties", alias = "java-properties", alias = "java-props")]
    #[strum(to_string = "Java Properties")]
    JavaProperties,
    #[serde(rename = "JSON", alias = "json")]
    #[strum(to_string = "JSON")]
    JSON,
    #[serde(rename = "JavaScript", alias = "js", alias = "javascript")]
    #[strum(to_string = "JavaScript")]
    JavaScript,
    #[serde(rename = "Regular Expressions (Javascript)", alias = "regex-js", alias = "regex-javascript")]
    #[strum(to_string = "Regular Expressions (Javascript)")]
    RegularExpressionsJavascript,
    #[serde(rename = "BibTeX", alias = "bibtex")]
    #[strum(to_string = "BibTeX")]
    BibTeX,
    #[serde(rename = "LaTeX Log", alias = "latex-log", alias = "latexlog")]
    #[strum(to_string = "LaTeX Log")]
    LaTeXLog,
    #[serde(rename = "LaTeX", alias = "latex")]
    #[strum(to_string = "LaTeX")]
    LaTeX,
    #[serde(rename = "TeX", alias = "tex")]
    #[strum(to_string = "TeX")]
    TeX,
    #[serde(rename = "Lisp", alias = "lisp")]
    #[strum(to_string = "Lisp")]
    Lisp,
    #[serde(rename = "Lua", alias = "lua")]
    #[strum(to_string = "Lua")]
    Lua,
    #[serde(rename = "Make Output", alias = "make-output", alias = "makeoutput")]
    #[strum(to_string = "Make Output")]
    MakeOutput,
    #[serde(rename = "Makefile", alias = "makefile")]
    #[strum(to_string = "Makefile")]
    Makefile,
    #[serde(rename = "Markdown", alias = "markdown", alias = "md")]
    #[strum(to_string = "Markdown")]
    Markdown,
    #[serde(rename = "MultiMarkdown", alias = "multi-markdown", alias = "multi-md")]
    #[strum(to_string = "MultiMarkdown")]
    MultiMarkdown,
    #[serde(rename = "MATLAB", alias = "matlab")]
    #[strum(to_string = "MATLAB")]
    MATLAB,
    #[serde(rename = "OCaml", alias = "ocaml")]
    #[strum(to_string = "OCaml")]
    OCaml,
    #[serde(rename = "OCamllex", alias = "ocamllex")]
    #[strum(to_string = "OCamllex")]
    OCamllex,
    #[serde(rename = "OCamlyacc", alias = "ocamlyacc")]
    #[strum(to_string = "OCamlyacc")]
    OCamlyacc,
    #[serde(rename = "camlp4")]
    #[strum(to_string = "camlp4")]
    Camlp4,
    #[serde(rename = "Objective-C++", alias = "objective-cpp", alias = "objective-c++")]
    #[strum(to_string = "Objective-C++")]
    ObjectiveCpp,
    #[serde(rename = "Objective-C", alias = "objective-c")]
    #[strum(to_string = "Objective-C")]
    ObjectiveC,
    #[serde(rename = "PHP Source", alias = "php-source")]
    #[strum(to_string = "PHP Source")]
    PHPSource,
    #[serde(rename = "PHP", alias = "php")]
    #[strum(to_string = "PHP")]
    PHP,
    #[serde(rename = "Pascal", alias = "pascal")]
    #[strum(to_string = "Pascal")]
    Pascal,
    #[serde(rename = "Perl", alias = "perl")]
    #[strum(to_string = "Perl")]
    Perl,
    #[serde(rename = "Python", alias = "python", alias = "py")]
    #[strum(to_string = "Python")]
    Python,
    #[serde(rename = "Regular Expressions (Python)", alias = "regex-py", alias = "regex-python")]
    #[strum(to_string = "Regular Expressions (Python)")]
    RegularExpressionsPython,
    #[serde(rename = "R Console", alias = "r-console")]
    #[strum(to_string = "R Console")]
    RConsole,
    #[serde(rename = "R", alias = "r")]
    #[strum(to_string = "R")]
    R,
    #[serde(rename = "Rd (R Documentation)", alias = "rdoc", alias = "r-doc", alias = "r-documentation")]
    #[strum(to_string = "Rd (R Documentation)")]
    Rdoc,
    #[serde(rename = "HTML (Rails)", alias = "html-rails", alias = "htmlrails")]
    #[strum(to_string = "HTML (Rails)")]
    #[allow(non_camel_case_types)]
    HTML_Rails,
    #[serde(rename = "JavaScript (Rails)",
            alias = "jsrails",
            alias = "js-rails",
            alias = "javascript-rails",
            alias = "javascriptrails")]
    #[strum(to_string = "JavaScript (Rails)")]
    #[allow(non_camel_case_types)]
    JavaScript_Rails,
    #[serde(rename = "Ruby Haml", alias = "ruby-haml", alias = "rubyhaml")]
    #[strum(to_string = "Ruby Haml")]
    RubyHaml,
    #[serde(rename = "Ruby on Rails", alias = "rubyonrails", alias = "ruby-on-rails", alias = "rails")]
    #[strum(to_string = "Ruby on Rails")]
    RubyOnRails,
    #[serde(rename = "SQL (Rails)", alias = "sql-rails", alias = "sqlrails")]
    #[strum(to_string = "SQL (Rails)")]
    #[allow(non_camel_case_types)]
    SQL_Rails,
    #[serde(rename = "Regular Expression", alias = "regular-expression", alias = "regex")]
    #[strum(to_string = "Regular Expression")]
    RegularExpression,
    #[serde(rename = "reStructuredText", alias = "restructuredtext", alias = "restructured-text")]
    #[strum(to_string = "reStructuredText")]
    ReStructuredText,
    #[serde(rename = "Ruby", alias = "ruby")]
    #[strum(to_string = "Ruby")]
    Ruby,
    #[serde(rename = "Cargo Build Results",
            alias = "cargo-build-results",
            alias = "cargo-build-output",
            alias = "cargo-build",
            alias = "cargobuildresults",
            alias = "cargobuildoutput",
            alias = "cargobuild")]
    #[strum(to_string = "Cargo Build Results")]
    CargoBuildResults,
    #[serde(rename = "Rust", alias = "rust", alias = "rs")]
    #[strum(to_string = "Rust")]
    Rust,
    #[serde(rename = "SQL", alias = "sql")]
    #[strum(to_string = "SQL")]
    SQL,
    #[serde(rename = "Scala", alias = "scalar")]
    #[strum(to_string = "Scala")]
    Scala,
    #[serde(rename = "Bourne Again Shell (bash)", alias = "bash")]
    #[strum(to_string = "Bourne Again Shell (bash)")]
    Bash,
    #[serde(rename = "Shell-Unix-Generic", alias = "unix-shell", alias = "unixshell")]
    #[strum(to_string = "Shell-Unix-Generic")]
    GenericUnixShell,
    #[serde(rename = "commands-builtin-shell-bash")]
    #[strum(to_string = "commands-builtin-shell-bash")]
    CommandsBuiltinShellBash,
    #[serde(rename = "HTML (Tcl)", alias = "html-tcl", alias = "htmltcl")]
    #[strum(to_string = "HTML (Tcl)")]
    #[allow(non_camel_case_types)]
    HTML_TCL,
    #[serde(rename = "Tcl", alias = "tcl")]
    #[strum(to_string = "Tcl")]
    Tcl,
    #[serde(rename = "Textile", alias = "textile")]
    #[strum(to_string = "Textile")]
    Textile,
    #[serde(rename = "XML", alias = "xml")]
    #[strum(to_string = "XML")]
    XML,
    #[serde(rename = "YAML", alias = "yaml")]
    #[strum(to_string = "YAML")]
    YAML,
    // Conundrum specific blocks
    #[serde(rename = "conundrum-ai", alias = "fluster-ai")]
    #[strum(to_string = "conundrum-ai")]
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
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Parser Error", format!("The `{}` language grammar could not be loaded. If this continues please file an issue on Github.", self.to_string()).as_str()))
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
            SupportedCodeBlockSyntax::RegularExpressionsJavascript => "regexp".to_string(),
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
            SupportedCodeBlockSyntax::RegularExpressionsPython => "regexp".to_string(),
            SupportedCodeBlockSyntax::RConsole => "r-console".to_string(),
            SupportedCodeBlockSyntax::R => "r".to_string(),
            SupportedCodeBlockSyntax::Rdoc => "rdoc".to_string(),
            SupportedCodeBlockSyntax::HTML_Rails => "html".to_string(),
            SupportedCodeBlockSyntax::JavaScript_Rails => "javascript".to_string(),
            SupportedCodeBlockSyntax::RubyHaml => "html".to_string(),
            SupportedCodeBlockSyntax::RubyOnRails => "rubyonrails".to_string(),
            SupportedCodeBlockSyntax::SQL_Rails => "sql".to_string(),
            SupportedCodeBlockSyntax::RegularExpression => "regexp".to_string(),
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
