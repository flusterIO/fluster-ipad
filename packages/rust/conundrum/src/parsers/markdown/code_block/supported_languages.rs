use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// All keys must be cast to lowercase and all `_` replaced with `-`.
#[derive(Serialize, Deserialize, strum_macros::Display, EnumIter)]
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
}
