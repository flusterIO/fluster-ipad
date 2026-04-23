// WARN: THiS FILE IS AUTO GENERATED. EDIT THINGS HERE AND YOUR WORK WILL BE
// LOST.
use devicons;
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
///
/// I gave up and used AI 1/2 way through for these... I don't have internet and
/// I haven't written 70% of these languages... so blame a local qwen
/// model for if these are wrong, until I can get some stable time on WIFI. I
/// didn't really feel like basically emoji's was the best use of my precious
/// homeless-wifi time.:
///
/// This defaults to python for code blocks, unless otherwise specified in the
/// applicat
#[typeshare::typeshare]
#[allow(non_camel_case_types)]
#[derive(Serialize,
           Deserialize,
           strum_macros::Display,
           strum_macros::EnumString,
           EnumIter,
           uniffi::Enum,
           Default,
           Clone,
           Debug,
           Eq,
           PartialEq)]
pub enum SupportedCodeBlockSyntax {
    #[serde(rename = "Plain Text")]
    #[strum(to_string = "Plain Text", serialize = "txt")]
    Plain_Text,
    #[serde(rename = "ASP")]
    #[strum(to_string = "ASP", serialize = "asa")]
    ASP,
    #[serde(rename = "HTML (ASP)")]
    #[strum(to_string = "HTML (ASP)", serialize = "asp")]
    HTML_ASP,
    #[serde(rename = "ActionScript")]
    #[strum(to_string = "ActionScript", serialize = "as")]
    ActionScript,
    #[serde(rename = "AppleScript")]
    #[strum(to_string = "AppleScript", serialize = "applescript", serialize = "script editor")]
    AppleScript,
    #[serde(rename = "Batch File")]
    #[strum(to_string = "Batch File", serialize = "bat", serialize = "cmd")]
    Batch_File,
    #[serde(rename = "NAnt Build File")]
    #[strum(to_string = "NAnt Build File", serialize = "build")]
    NAnt_Build_File,
    #[serde(rename = "C#")]
    #[strum(to_string = "C#", serialize = "cs", serialize = "csx")]
    CSharp,
    #[serde(rename = "C++")]
    #[strum(to_string = "C++",
            serialize = "cpp",
            serialize = "cc",
            serialize = "cp",
            serialize = "cxx",
            serialize = "c++",
            serialize = "C",
            serialize = "h",
            serialize = "hh",
            serialize = "hpp",
            serialize = "hxx",
            serialize = "h++",
            serialize = "inl",
            serialize = "ipp")]
    Cpp,
    #[serde(rename = "C")]
    #[strum(to_string = "C", serialize = "c", serialize = "h")]
    C,
    #[serde(rename = "CSS")]
    #[strum(to_string = "CSS", serialize = "css", serialize = "css.erb", serialize = "css.liquid")]
    CSS,
    #[serde(rename = "Clojure")]
    #[strum(to_string = "Clojure", serialize = "clj", serialize = "cljc", serialize = "cljs", serialize = "edn")]
    Clojure,
    #[serde(rename = "D")]
    #[strum(to_string = "D", serialize = "d", serialize = "di")]
    D,
    #[serde(rename = "DMD Output")]
    #[strum(to_string = "DMD Output")]
    DMD_Output,
    #[serde(rename = "Diff")]
    #[strum(to_string = "Diff", serialize = "diff", serialize = "patch")]
    Diff,
    #[serde(rename = "Erlang")]
    #[strum(to_string = "Erlang",
            serialize = "erl",
            serialize = "hrl",
            serialize = "Emakefile",
            serialize = "emakefile",
            serialize = "escript")]
    Erlang,
    #[serde(rename = "HTML (Erlang)")]
    #[strum(to_string = "HTML (Erlang)", serialize = "yaws")]
    HTML_Erlang,
    #[serde(rename = "Git Attributes")]
    #[strum(to_string = "Git Attributes",
            serialize = "attributes",
            serialize = "gitattributes",
            serialize = ".gitattributes")]
    Git_Attributes,
    #[serde(rename = "Git Commit")]
    #[strum(to_string = "Git Commit", serialize = "COMMIT_EDITMSG", serialize = "MERGE_MSG", serialize = "TAG_EDITMSG")]
    Git_Commit,
    #[serde(rename = "Git Common")]
    #[strum(to_string = "Git Common")]
    Git_Common,
    #[serde(rename = "Git Config")]
    #[strum(to_string = "Git Config", serialize = "gitconfig", serialize = ".gitconfig", serialize = ".gitmodules")]
    Git_Config,
    #[serde(rename = "Git Ignore")]
    #[strum(to_string = "Git Ignore", serialize = "exclude", serialize = "gitignore", serialize = ".gitignore")]
    Git_Ignore,
    #[serde(rename = "Git Link")]
    #[strum(to_string = "Git Link", serialize = ".git")]
    Git_Link,
    #[serde(rename = "Git Log")]
    #[strum(to_string = "Git Log", serialize = "gitlog")]
    Git_Log,
    #[serde(rename = "Git Mailmap")]
    #[strum(to_string = "Git Mailmap", serialize = ".mailmap", serialize = "mailmap")]
    Git_Mailmap,
    #[serde(rename = "Git Rebase Todo")]
    #[strum(to_string = "Git Rebase Todo", serialize = "git-rebase-todo")]
    Git_Rebase_Todo,
    #[serde(rename = "Go")]
    #[strum(to_string = "Go", serialize = "go")]
    Go,
    #[serde(rename = "Graphviz (DOT)")]
    #[strum(to_string = "Graphviz (DOT)", serialize = "dot", serialize = "DOT", serialize = "gv")]
    Graphviz_DOT,
    #[serde(rename = "Groovy")]
    #[strum(to_string = "Groovy",
            serialize = "groovy",
            serialize = "gvy",
            serialize = "gradle",
            serialize = "Jenkinsfile")]
    Groovy,
    #[serde(rename = "HTML")]
    #[strum(to_string = "HTML", serialize = "html", serialize = "htm", serialize = "shtml", serialize = "xhtml")]
    HTML,
    #[serde(rename = "Haskell")]
    #[strum(to_string = "Haskell", serialize = "hs")]
    Haskell,
    #[serde(rename = "Literate Haskell")]
    #[strum(to_string = "Literate Haskell", serialize = "lhs")]
    Literate_Haskell,
    #[serde(rename = "JSON")]
    #[strum(to_string = "JSON",
            serialize = "json",
            serialize = "sublime-settings",
            serialize = "sublime-menu",
            serialize = "sublime-keymap",
            serialize = "sublime-mousemap",
            serialize = "sublime-theme",
            serialize = "sublime-build",
            serialize = "sublime-project",
            serialize = "sublime-completions",
            serialize = "sublime-commands",
            serialize = "sublime-macro",
            serialize = "sublime-color-scheme",
            serialize = "ipynb",
            serialize = "Pipfile.lock")]
    JSON,
    #[serde(rename = "Java Server Page (JSP)")]
    #[strum(to_string = "Java Server Page (JSP)", serialize = "jsp")]
    Java_Server_Page_JSP,
    #[serde(rename = "Java")]
    #[strum(to_string = "Java", serialize = "java", serialize = "bsh")]
    Java,
    #[serde(rename = "Javadoc")]
    #[strum(to_string = "Javadoc")]
    Javadoc,
    #[serde(rename = "Java Properties")]
    #[strum(to_string = "Java Properties", serialize = "properties")]
    Java_Properties,
    #[serde(rename = "JavaScript")]
    #[strum(to_string = "JavaScript", serialize = "js", serialize = "htc")]
    JavaScript,
    #[serde(rename = "Regular Expressions (Javascript)")]
    #[strum(to_string = "Regular Expressions (Javascript)")]
    Regular_Expressions_Javascript,
    #[serde(rename = "BibTeX")]
    #[strum(to_string = "BibTeX", serialize = "bib")]
    BibTeX,
    #[serde(rename = "LaTeX Log")]
    #[strum(to_string = "LaTeX Log")]
    LaTeX_Log,
    #[serde(rename = "LaTeX")]
    #[strum(to_string = "LaTeX", serialize = "tex", serialize = "ltx")]
    LaTeX,
    #[serde(rename = "TeX")]
    #[strum(to_string = "TeX", serialize = "sty", serialize = "cls")]
    TeX,
    #[serde(rename = "Lisp")]
    #[strum(to_string = "Lisp",
            serialize = "lisp",
            serialize = "cl",
            serialize = "clisp",
            serialize = "l",
            serialize = "mud",
            serialize = "el",
            serialize = "scm",
            serialize = "ss",
            serialize = "lsp",
            serialize = "fasl")]
    Lisp,
    #[serde(rename = "Lua")]
    #[strum(to_string = "Lua", serialize = "lua")]
    Lua,
    #[serde(rename = "Make Output")]
    #[strum(to_string = "Make Output")]
    Make_Output,
    #[serde(rename = "Makefile")]
    #[strum(to_string = "Makefile",
            serialize = "make",
            serialize = "GNUmakefile",
            serialize = "makefile",
            serialize = "Makefile",
            serialize = "makefile.am",
            serialize = "Makefile.am",
            serialize = "makefile.in",
            serialize = "Makefile.in",
            serialize = "OCamlMakefile",
            serialize = "mak",
            serialize = "mk")]
    Makefile,
    #[serde(rename = "Markdown")]
    #[strum(to_string = "Markdown",
            serialize = "md",
            serialize = "mdown",
            serialize = "markdown",
            serialize = "markdn")]
    Markdown,
    #[serde(rename = "MultiMarkdown")]
    #[strum(to_string = "MultiMarkdown")]
    MultiMarkdown,
    #[serde(rename = "MATLAB")]
    #[strum(to_string = "MATLAB", serialize = "matlab")]
    MATLAB,
    #[serde(rename = "OCaml")]
    #[strum(to_string = "OCaml", serialize = "ml", serialize = "mli")]
    OCaml,
    #[serde(rename = "OCamllex")]
    #[strum(to_string = "OCamllex", serialize = "mll")]
    OCamllex,
    #[serde(rename = "OCamlyacc")]
    #[strum(to_string = "OCamlyacc", serialize = "mly")]
    OCamlyacc,
    #[serde(rename = "camlp4")]
    #[strum(to_string = "camlp4")]
    Camlp4,
    #[serde(rename = "Objective-C++")]
    #[strum(to_string = "Objective-C++", serialize = "mm", serialize = "M", serialize = "h")]
    Objective_Cpp,
    #[serde(rename = "Objective-C")]
    #[strum(to_string = "Objective-C", serialize = "m", serialize = "h")]
    Objective_C,
    #[serde(rename = "PHP Source")]
    #[strum(to_string = "PHP Source")]
    PHP_Source,
    #[serde(rename = "PHP")]
    #[strum(to_string = "PHP",
            serialize = "php",
            serialize = "php3",
            serialize = "php4",
            serialize = "php5",
            serialize = "php7",
            serialize = "phps",
            serialize = "phpt",
            serialize = "phtml")]
    PHP,
    #[serde(rename = "Regular Expressions (PHP)")]
    #[strum(to_string = "Regular Expressions (PHP)")]
    Regular_Expressions_PHP,
    #[serde(rename = "Pascal")]
    #[strum(to_string = "Pascal", serialize = "pas", serialize = "p", serialize = "dpr")]
    Pascal,
    #[serde(rename = "Perl")]
    #[strum(to_string = "Perl",
            serialize = "pl",
            serialize = "pc",
            serialize = "pm",
            serialize = "pmc",
            serialize = "pod",
            serialize = "t")]
    Perl,
    #[serde(rename = "Python")]
    #[strum(to_string = "Python",
            serialize = "py",
            serialize = "py3",
            serialize = "pyw",
            serialize = "pyi",
            serialize = "pyx",
            serialize = "pyx.in",
            serialize = "pxd",
            serialize = "pxd.in",
            serialize = "pxi",
            serialize = "pxi.in",
            serialize = "rpy",
            serialize = "cpy",
            serialize = "SConstruct",
            serialize = "Sconstruct",
            serialize = "sconstruct",
            serialize = "SConscript",
            serialize = "gyp",
            serialize = "gypi",
            serialize = "Snakefile",
            serialize = "vpy",
            serialize = "wscript",
            serialize = "bazel",
            serialize = "bzl")]
    #[default]
    Python,
    #[serde(rename = "Regular Expressions (Python)")]
    #[strum(to_string = "Regular Expressions (Python)")]
    Regular_Expressions_Python,
    #[serde(rename = "R Console")]
    #[strum(to_string = "R Console")]
    R_Console,
    #[serde(rename = "R")]
    #[strum(to_string = "R", serialize = "R", serialize = "r", serialize = "Rprofile")]
    R,
    #[serde(rename = "Rd (R Documentation)")]
    #[strum(to_string = "Rd (R Documentation)", serialize = "rd")]
    Rd_R_Documentation,
    #[serde(rename = "HTML (Rails)")]
    #[strum(to_string = "HTML (Rails)",
            serialize = "rails",
            serialize = "rhtml",
            serialize = "erb",
            serialize = "html.erb")]
    HTML_Rails,
    #[serde(rename = "JavaScript (Rails)")]
    #[strum(to_string = "JavaScript (Rails)", serialize = "js.erb")]
    JavaScript_Rails,
    #[serde(rename = "Ruby Haml")]
    #[strum(to_string = "Ruby Haml", serialize = "haml", serialize = "sass")]
    Ruby_Haml,
    #[serde(rename = "Ruby on Rails")]
    #[strum(to_string = "Ruby on Rails", serialize = "rxml", serialize = "builder")]
    Ruby_on_Rails,
    #[serde(rename = "SQL (Rails)")]
    #[strum(to_string = "SQL (Rails)", serialize = "erbsql", serialize = "sql.erb")]
    SQL_Rails,
    #[serde(rename = "Regular Expression")]
    #[strum(to_string = "Regular Expression", serialize = "re")]
    Regular_Expression,
    #[serde(rename = "reStructuredText")]
    #[strum(to_string = "reStructuredText", serialize = "rst", serialize = "rest")]
    ReStructuredText,
    #[serde(rename = "Ruby")]
    #[strum(to_string = "Ruby",
            serialize = "rb",
            serialize = "Appfile",
            serialize = "Appraisals",
            serialize = "Berksfile",
            serialize = "Brewfile",
            serialize = "capfile",
            serialize = "cgi",
            serialize = "Cheffile",
            serialize = "config.ru",
            serialize = "Deliverfile",
            serialize = "Fastfile",
            serialize = "fcgi",
            serialize = "Gemfile",
            serialize = "gemspec",
            serialize = "Guardfile",
            serialize = "irbrc",
            serialize = "jbuilder",
            serialize = "Podfile",
            serialize = "podspec",
            serialize = "prawn",
            serialize = "rabl",
            serialize = "rake",
            serialize = "Rakefile",
            serialize = "Rantfile",
            serialize = "rbx",
            serialize = "rjs",
            serialize = "ruby.rail",
            serialize = "Scanfile",
            serialize = "simplecov",
            serialize = "Snapfile",
            serialize = "thor",
            serialize = "Thorfile",
            serialize = "Vagrantfile")]
    Ruby,
    #[serde(rename = "Cargo Build Results")]
    #[strum(to_string = "Cargo Build Results")]
    Cargo_Build_Results,
    #[serde(rename = "Rust")]
    #[strum(to_string = "Rust", serialize = "rs")]
    Rust,
    #[serde(rename = "SQL")]
    #[strum(to_string = "SQL", serialize = "sql", serialize = "ddl", serialize = "dml")]
    SQL,
    #[serde(rename = "Scala")]
    #[strum(to_string = "Scala", serialize = "scala", serialize = "sbt", serialize = "sc")]
    Scala,
    #[serde(rename = "Bourne Again Shell (bash)")]
    #[strum(to_string = "Bourne Again Shell (bash)",
            serialize = "sh",
            serialize = "bash",
            serialize = "zsh",
            serialize = "ash",
            serialize = ".bash_aliases",
            serialize = ".bash_completions",
            serialize = ".bash_functions",
            serialize = ".bash_login",
            serialize = ".bash_logout",
            serialize = ".bash_profile",
            serialize = ".bash_variables",
            serialize = ".bashrc",
            serialize = ".profile",
            serialize = ".textmate_init",
            serialize = ".zlogin",
            serialize = ".zlogout",
            serialize = ".zprofile",
            serialize = ".zshenv",
            serialize = ".zshrc",
            serialize = "PKGBUILD",
            serialize = "ebuild",
            serialize = "eclass")]
    Bourne_Again_Shell_bash,
    #[serde(rename = "Shell-Unix-Generic")]
    #[strum(to_string = "Shell-Unix-Generic")]
    Shell_Unix_Generic,
    #[serde(rename = "commands-builtin-shell-bash")]
    #[strum(to_string = "commands-builtin-shell-bash")]
    Commands_builtin_shell_bash,
    #[serde(rename = "HTML (Tcl)")]
    #[strum(to_string = "HTML (Tcl)", serialize = "adp")]
    HTML_Tcl,
    #[serde(rename = "Tcl")]
    #[strum(to_string = "Tcl", serialize = "tcl")]
    Tcl,
    #[serde(rename = "Textile")]
    #[strum(to_string = "Textile", serialize = "textile")]
    Textile,
    #[serde(rename = "XML")]
    #[strum(to_string = "XML",
            serialize = "xml",
            serialize = "xsd",
            serialize = "xslt",
            serialize = "tld",
            serialize = "dtml",
            serialize = "rng",
            serialize = "rss",
            serialize = "opml",
            serialize = "svg",
            serialize = "xaml")]
    XML,
    #[serde(rename = "YAML")]
    #[strum(to_string = "YAML", serialize = "yaml", serialize = "yml", serialize = "sublime-syntax")]
    YAML,
    #[serde(rename = "AWK")]
    #[strum(to_string = "AWK", serialize = "awk")]
    AWK,
    #[serde(rename = "Ada")]
    #[strum(to_string = "Ada", serialize = "adb", serialize = "ads", serialize = "gpr")]
    Ada,
    #[serde(rename = "Apache Conf")]
    #[strum(to_string = "Apache Conf",
            serialize = "envvars",
            serialize = "htaccess",
            serialize = "HTACCESS",
            serialize = "htgroups",
            serialize = "HTGROUPS",
            serialize = "htpasswd",
            serialize = "HTPASSWD",
            serialize = ".htaccess",
            serialize = ".HTACCESS",
            serialize = ".htgroups",
            serialize = ".HTGROUPS",
            serialize = ".htpasswd",
            serialize = ".HTPASSWD")]
    Apache_Conf,
    #[serde(rename = "AsciiDoc (Asciidoctor)")]
    #[strum(to_string = "AsciiDoc (Asciidoctor)", serialize = "adoc", serialize = "ad", serialize = "asciidoc")]
    AsciiDoc_Asciidoctor,
    #[serde(rename = "ARM Assembly")]
    #[strum(to_string = "ARM Assembly", serialize = "s", serialize = "S")]
    ARM_Assembly,
    #[serde(rename = "Assembly (x86_64)")]
    #[strum(to_string = "Assembly (x86_64)",
            serialize = "yasm",
            serialize = "nasm",
            serialize = "asm",
            serialize = "inc",
            serialize = "mac")]
    Assembly_x86_64,
    #[serde(rename = "CMake C Header")]
    #[strum(to_string = "CMake C Header", serialize = "h.in")]
    CMake_C_Header,
    #[serde(rename = "CMake C++ Header")]
    #[strum(to_string = "CMake C++ Header",
            serialize = "hh.in",
            serialize = "hpp.in",
            serialize = "hxx.in",
            serialize = "h++.in")]
    CMake_Cpp_Header,
    #[serde(rename = "CMake")]
    #[strum(to_string = "CMake", serialize = "CMakeLists.txt", serialize = "cmake")]
    CMake,
    #[serde(rename = "CMakeCache")]
    #[strum(to_string = "CMakeCache", serialize = "CMakeCache.txt")]
    CMakeCache,
    #[serde(rename = "CMakeCommands")]
    #[strum(to_string = "CMakeCommands")]
    CMakeCommands,
    #[serde(rename = "Comma Separated Values")]
    #[strum(to_string = "Comma Separated Values", serialize = "csv", serialize = "tsv")]
    Comma_Separated_Values,
    #[serde(rename = "Cabal")]
    #[strum(to_string = "Cabal", serialize = "cabal")]
    Cabal,
    #[serde(rename = "CoffeeScript")]
    #[strum(to_string = "CoffeeScript",
            serialize = "coffee",
            serialize = "Cakefile",
            serialize = "coffee.erb",
            serialize = "cson")]
    CoffeeScript,
    #[serde(rename = "CpuInfo")]
    #[strum(to_string = "CpuInfo", serialize = "cpuinfo")]
    CpuInfo,
    #[serde(rename = "Crontab")]
    #[strum(to_string = "Crontab", serialize = "tab", serialize = "crontab", serialize = "cron.d")]
    Crontab,
    #[serde(rename = "Crystal")]
    #[strum(to_string = "Crystal", serialize = "cr")]
    Crystal,
    #[serde(rename = "Dart")]
    #[strum(to_string = "Dart", serialize = "dart")]
    Dart,
    #[serde(rename = "Dockerfile")]
    #[strum(to_string = "Dockerfile", serialize = "Dockerfile", serialize = "dockerfile")]
    Dockerfile,
    #[serde(rename = "DotENV")]
    #[strum(to_string = "DotENV",
            serialize = ".env",
            serialize = ".env.dist",
            serialize = ".env.local",
            serialize = ".env.sample",
            serialize = ".env.example",
            serialize = ".env.template",
            serialize = ".env.test",
            serialize = ".env.test.local",
            serialize = ".env.testing",
            serialize = ".env.dev",
            serialize = ".env.development",
            serialize = ".env.development.local",
            serialize = ".env.prod",
            serialize = ".env.production",
            serialize = ".env.production.local",
            serialize = ".env.dusk.local",
            serialize = ".env.staging",
            serialize = ".env.default",
            serialize = ".env.defaults",
            serialize = ".envrc",
            serialize = ".flaskenv",
            serialize = "env",
            serialize = "env.example",
            serialize = "env.sample",
            serialize = "env.template")]
    DotENV,
    #[serde(rename = "Elixir")]
    #[strum(to_string = "Elixir", serialize = "ex", serialize = "exs")]
    Elixir,
    #[serde(rename = "HTML (EEx)")]
    #[strum(to_string = "HTML (EEx)", serialize = "html.eex", serialize = "html.leex")]
    HTML_EEx,
    #[serde(rename = "Regular Expressions (Elixir)")]
    #[strum(to_string = "Regular Expressions (Elixir)", serialize = "ex.re")]
    Regular_Expressions_Elixir,
    #[serde(rename = "Elm Compile Messages")]
    #[strum(to_string = "Elm Compile Messages")]
    Elm_Compile_Messages,
    #[serde(rename = "Elm Documentation")]
    #[strum(to_string = "Elm Documentation")]
    Elm_Documentation,
    #[serde(rename = "Elm")]
    #[strum(to_string = "Elm", serialize = "elm")]
    Elm,
    #[serde(rename = "Email")]
    #[strum(to_string = "Email", serialize = "eml", serialize = "msg", serialize = "mbx", serialize = "mboxz")]
    Email,
    #[serde(rename = "F#")]
    #[strum(to_string = "F#", serialize = "fs", serialize = "fsi", serialize = "fsx")]
    FSharp,
    #[serde(rename = "Fish")]
    #[strum(to_string = "Fish", serialize = "fish")]
    Fish,
    #[serde(rename = "Fortran (Fixed Form)")]
    #[strum(to_string = "Fortran (Fixed Form)",
            serialize = "f",
            serialize = "F",
            serialize = "f77",
            serialize = "F77",
            serialize = "for",
            serialize = "FOR",
            serialize = "fpp",
            serialize = "FPP")]
    Fortran_Fixed_Form,
    #[serde(rename = "Fortran (Modern)")]
    #[strum(to_string = "Fortran (Modern)",
            serialize = "f90",
            serialize = "F90",
            serialize = "f95",
            serialize = "F95",
            serialize = "f03",
            serialize = "F03",
            serialize = "f08",
            serialize = "F08")]
    Fortran_Modern,
    #[serde(rename = "Fortran Namelist")]
    #[strum(to_string = "Fortran Namelist", serialize = "namelist")]
    Fortran_Namelist,
    #[serde(rename = "GFortran Build Results")]
    #[strum(to_string = "GFortran Build Results")]
    GFortran_Build_Results,
    #[serde(rename = "OpenMP (Fortran)")]
    #[strum(to_string = "OpenMP (Fortran)")]
    OpenMP_Fortran,
    #[serde(rename = "fstab")]
    #[strum(to_string = "fstab", serialize = "fstab", serialize = "crypttab", serialize = "mtab")]
    Fstab,
    #[serde(rename = "GLSL")]
    #[strum(to_string = "GLSL",
            serialize = "vs",
            serialize = "fs",
            serialize = "gs",
            serialize = "vsh",
            serialize = "fsh",
            serialize = "gsh",
            serialize = "vshader",
            serialize = "fshader",
            serialize = "gshader",
            serialize = "vert",
            serialize = "frag",
            serialize = "geom",
            serialize = "tesc",
            serialize = "tese",
            serialize = "comp",
            serialize = "glsl",
            serialize = "mesh",
            serialize = "task",
            serialize = "rgen",
            serialize = "rint",
            serialize = "rahit",
            serialize = "rchit",
            serialize = "rmiss",
            serialize = "rcall")]
    GLSL,
    #[serde(rename = "GraphQL")]
    #[strum(to_string = "GraphQL", serialize = "graphql", serialize = "graphqls", serialize = "gql")]
    GraphQL,
    #[serde(rename = "Groff/troff")]
    #[strum(to_string = "Groff/troff",
            serialize = "groff",
            serialize = "troff",
            serialize = "1",
            serialize = "2",
            serialize = "3",
            serialize = "4",
            serialize = "5",
            serialize = "6",
            serialize = "7",
            serialize = "8",
            serialize = "9")]
    Groff_troff,
    #[serde(rename = "group")]
    #[strum(to_string = "group", serialize = "group")]
    Group,
    #[serde(rename = "HTML (Twig)")]
    #[strum(to_string = "HTML (Twig)", serialize = "twig", serialize = "html.twig")]
    HTML_Twig,
    #[serde(rename = "hosts")]
    #[strum(to_string = "hosts", serialize = "hosts")]
    Hosts,
    #[serde(rename = "INI")]
    #[strum(to_string = "INI",
            serialize = "ini",
            serialize = "inf",
            serialize = "INF",
            serialize = "reg",
            serialize = "REG",
            serialize = "lng",
            serialize = "cfg",
            serialize = "CFG",
            serialize = "desktop",
            serialize = "url",
            serialize = "URL",
            serialize = ".editorconfig",
            serialize = ".coveragerc",
            serialize = ".pylintrc",
            serialize = ".gitlint",
            serialize = ".hgrc",
            serialize = "hgrc")]
    INI,
    #[serde(rename = "JavaScript (Babel)")]
    #[strum(to_string = "JavaScript (Babel)",
            serialize = "js",
            serialize = "mjs",
            serialize = "jsx",
            serialize = "babel",
            serialize = "es6",
            serialize = "cjs")]
    JavaScript_Babel,
    #[serde(rename = "HTML (Jinja2)")]
    #[strum(to_string = "HTML (Jinja2)",
            serialize = "htm.j2",
            serialize = "html.j2",
            serialize = "xhtml.j2",
            serialize = "xml.j2")]
    HTML_Jinja2,
    #[serde(rename = "Jinja2")]
    #[strum(to_string = "Jinja2", serialize = "j2", serialize = "jinja2", serialize = "jinja")]
    Jinja2,
    #[serde(rename = "jsonnet")]
    #[strum(to_string = "jsonnet", serialize = "jsonnet", serialize = "libsonnet", serialize = "libjsonnet")]
    Jsonnet,
    #[serde(rename = "Julia")]
    #[strum(to_string = "Julia", serialize = "jl")]
    Julia,
    #[serde(rename = "Kotlin")]
    #[strum(to_string = "Kotlin", serialize = "kt", serialize = "kts")]
    Kotlin,
    #[serde(rename = "Less")]
    #[strum(to_string = "Less", serialize = "less", serialize = "css.less")]
    Less,
    #[serde(rename = "LLVM")]
    #[strum(to_string = "LLVM", serialize = "ll")]
    LLVM,
    #[serde(rename = "Lean")]
    #[strum(to_string = "Lean", serialize = "lean")]
    Lean,
    #[serde(rename = "LiveScript")]
    #[strum(to_string = "LiveScript", serialize = "ls", serialize = "Slakefile", serialize = "ls.erb")]
    LiveScript,
    #[serde(rename = "Manpage")]
    #[strum(to_string = "Manpage", serialize = "man")]
    Manpage,
    #[serde(rename = "MediawikerPanel")]
    #[strum(to_string = "MediawikerPanel")]
    MediawikerPanel,
    #[serde(rename = "MediaWiki")]
    #[strum(to_string = "MediaWiki", serialize = "mediawiki", serialize = "wikipedia", serialize = "wiki")]
    MediaWiki,
    #[serde(rename = "MemInfo")]
    #[strum(to_string = "MemInfo", serialize = "meminfo")]
    MemInfo,
    #[serde(rename = "nginx")]
    #[strum(to_string = "nginx",
            serialize = "conf.erb",
            serialize = "conf",
            serialize = "nginx.conf",
            serialize = "mime.types",
            serialize = "fastcgi_params",
            serialize = "scgi_params",
            serialize = "uwsgi_params")]
    Nginx,
    #[serde(rename = "Nim")]
    #[strum(to_string = "Nim", serialize = "nim", serialize = "nims", serialize = "nimble")]
    Nim,
    #[serde(rename = "Ninja")]
    #[strum(to_string = "Ninja", serialize = "ninja")]
    Ninja,
    #[serde(rename = "Nix")]
    #[strum(to_string = "Nix", serialize = "nix")]
    Nix,
    #[serde(rename = "orgmode")]
    #[strum(to_string = "orgmode", serialize = "org")]
    Orgmode,
    #[serde(rename = "passwd")]
    #[strum(to_string = "passwd", serialize = "passwd")]
    Passwd,
    #[serde(rename = "PowerShell")]
    #[strum(to_string = "PowerShell", serialize = "ps1", serialize = "psm1", serialize = "psd1")]
    PowerShell,
    #[serde(rename = "Protocol Buffer")]
    #[strum(to_string = "Protocol Buffer", serialize = "proto", serialize = "protodevel")]
    Protocol_Buffer,
    #[serde(rename = "Protocol Buffer (TEXT)")]
    #[strum(to_string = "Protocol Buffer (TEXT)",
            serialize = "pb.txt",
            serialize = "proto.text",
            serialize = "textpb",
            serialize = "pbtxt",
            serialize = "prototxt")]
    Protocol_Buffer_TEXT,
    #[serde(rename = "Puppet")]
    #[strum(to_string = "Puppet", serialize = "pp", serialize = "epp")]
    Puppet,
    #[serde(rename = "PureScript")]
    #[strum(to_string = "PureScript", serialize = "purs")]
    PureScript,
    #[serde(rename = "QML")]
    #[strum(to_string = "QML", serialize = "qml", serialize = "qmlproject")]
    QML,
    #[serde(rename = "Racket")]
    #[strum(to_string = "Racket", serialize = "rkt")]
    Racket,
    #[serde(rename = "Rego")]
    #[strum(to_string = "Rego", serialize = "rego")]
    Rego,
    #[serde(rename = "Requirements.txt")]
    #[strum(to_string = "Requirements.txt",
            serialize = "requirements.txt",
            serialize = "requirements.in",
            serialize = "pip")]
    RequirementsDottxt,
    #[serde(rename = "resolv")]
    #[strum(to_string = "resolv", serialize = "resolv.conf")]
    Resolv,
    #[serde(rename = "Robot Framework")]
    #[strum(to_string = "Robot Framework", serialize = "robot", serialize = "resource")]
    Robot_Framework,
    #[serde(rename = "SCSS")]
    #[strum(to_string = "SCSS", serialize = "scss")]
    SCSS,
    #[serde(rename = "Sass")]
    #[strum(to_string = "Sass", serialize = "sass")]
    Sass,
    #[serde(rename = "Salt State (SLS)")]
    #[strum(to_string = "Salt State (SLS)", serialize = "sls")]
    Salt_State_SLS,
    #[serde(rename = "SML")]
    #[strum(to_string = "SML", serialize = "sml", serialize = "cm", serialize = "sig")]
    SML,
    #[serde(rename = "Ruby Slim")]
    #[strum(to_string = "Ruby Slim", serialize = "slim", serialize = "skim")]
    Ruby_Slim,
    #[serde(rename = "Strace")]
    #[strum(to_string = "Strace", serialize = "strace")]
    Strace,
    #[serde(rename = "Stylus")]
    #[strum(to_string = "Stylus", serialize = "styl", serialize = "stylus")]
    Stylus,
    #[serde(rename = "Solidity")]
    #[strum(to_string = "Solidity", serialize = "sol")]
    Solidity,
    #[serde(rename = "Vyper")]
    #[strum(to_string = "Vyper", serialize = "vy")]
    Vyper,
    #[serde(rename = "JQ")]
    #[strum(to_string = "JQ", serialize = "jq")]
    JQ,
    #[serde(rename = "Svelte")]
    #[strum(to_string = "Svelte", serialize = "svlt", serialize = "svelte")]
    Svelte,
    #[serde(rename = "Swift")]
    #[strum(to_string = "Swift", serialize = "swift")]
    Swift,
    #[serde(rename = "SystemVerilog")]
    #[strum(to_string = "SystemVerilog", serialize = "sv", serialize = "v", serialize = "svh", serialize = "vh")]
    SystemVerilog,
    #[serde(rename = "Navigational Bar SV")]
    #[strum(to_string = "Navigational Bar SV")]
    Navigational_Bar_SV,
    #[serde(rename = "TOML")]
    #[strum(to_string = "TOML",
            serialize = "toml",
            serialize = "tml",
            serialize = "Cargo.lock",
            serialize = "Gopkg.lock",
            serialize = "Pipfile",
            serialize = "pdm.lock",
            serialize = "poetry.lock")]
    TOML,
    #[serde(rename = "JSON (Terraform)")]
    #[strum(to_string = "JSON (Terraform)", serialize = "tfstate")]
    JSON_Terraform,
    #[serde(rename = "Terraform")]
    #[strum(to_string = "Terraform", serialize = "tf", serialize = "tfvars", serialize = "hcl")]
    Terraform,
    #[serde(rename = "Todo.txt")]
    #[strum(to_string = "Todo.txt", serialize = "todo.txt", serialize = "done.txt")]
    TodoDottxt,
    #[serde(rename = "TypeScript")]
    #[strum(to_string = "TypeScript", serialize = "ts", serialize = "mts", serialize = "cts")]
    TypeScript,
    #[serde(rename = "TypeScriptReact")]
    #[strum(to_string = "TypeScriptReact", serialize = "tsx")]
    TypeScriptReact,
    #[serde(rename = "Verilog")]
    #[strum(to_string = "Verilog", serialize = "v", serialize = "V")]
    Verilog,
    #[serde(rename = "VimHelp")]
    #[strum(to_string = "VimHelp", serialize = "vimhelp")]
    VimHelp,
    #[serde(rename = "VimL")]
    #[strum(to_string = "VimL",
            serialize = "vim",
            serialize = "vimrc",
            serialize = "gvimrc",
            serialize = ".vimrc",
            serialize = ".gvimrc",
            serialize = "_vimrc",
            serialize = "_gvimrc")]
    VimL,
    #[serde(rename = "Vue Component")]
    #[strum(to_string = "Vue Component", serialize = "vue")]
    Vue_Component,
    #[serde(rename = "Zig")]
    #[strum(to_string = "Zig", serialize = "zig")]
    Zig,
    #[serde(rename = "Command Help")]
    #[strum(to_string = "Command Help", serialize = "cmd-help", serialize = "help")]
    Command_Help,
    #[serde(rename = "gnuplot")]
    #[strum(to_string = "gnuplot",
            serialize = "gp",
            serialize = "gpl",
            serialize = "gnuplot",
            serialize = "gnu",
            serialize = "plot",
            serialize = "plt")]
    Gnuplot,
    #[serde(rename = "HTTP Request and Response")]
    #[strum(to_string = "HTTP Request and Response", serialize = "http")]
    HTTP_Request_and_Response,
    #[serde(rename = "log")]
    #[strum(to_string = "log", serialize = "log")]
    Log,
    #[serde(rename = "Highlight non-printables")]
    #[strum(to_string = "Highlight non-printables", serialize = "show-nonprintable")]
    Highlight_non_printables,
    #[serde(rename = "Authorized Keys")]
    #[strum(to_string = "Authorized Keys",
            serialize = "authorized_keys",
            serialize = "pub",
            serialize = "authorized_keys2")]
    Authorized_Keys,
    #[serde(rename = "Known Hosts")]
    #[strum(to_string = "Known Hosts", serialize = "known_hosts", serialize = "known_hosts.old")]
    Known_Hosts,
    #[serde(rename = "Private Key")]
    #[strum(to_string = "Private Key")]
    Private_Key,
    #[serde(rename = "SSH Common")]
    #[strum(to_string = "SSH Common")]
    SSH_Common,
    #[serde(rename = "SSH Config")]
    #[strum(to_string = "SSH Config", serialize = "ssh_config")]
    SSH_Config,
    #[serde(rename = "SSH Crypto")]
    #[strum(to_string = "SSH Crypto")]
    SSH_Crypto,
    #[serde(rename = "SSHD Config")]
    #[strum(to_string = "SSHD Config", serialize = "sshd_config")]
    SSHD_Config,
    #[serde(rename = "syslog")]
    #[strum(to_string = "syslog", serialize = "syslog")]
    Syslog,
    #[serde(rename = "varlink")]
    #[strum(to_string = "varlink", serialize = "varlink")]
    Varlink,

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

    pub fn devicon(&self, is_dark_mode: &bool) -> devicons::FileIcon {
        match self {
            Self::Plain_Text => devicons::FileIcon::from("txt"),
            Self::ASP => devicons::FileIcon::from("asa"),
            Self::HTML_ASP => devicons::FileIcon::from("asp"),
            Self::ActionScript => devicons::FileIcon::from("as"),
            Self::AppleScript => devicons::FileIcon::from("applescript"),
            Self::Batch_File => devicons::FileIcon::from("bat"),
            Self::NAnt_Build_File => devicons::FileIcon::from("build"),
            Self::CSharp => devicons::FileIcon::from("cs"),
            Self::Cpp => devicons::FileIcon::from("cpp"),
            Self::C => devicons::FileIcon::from("c"),
            Self::CSS => devicons::FileIcon::from("css"),
            Self::Clojure => devicons::FileIcon::from("clj"),
            Self::D => devicons::FileIcon::from("d"),
            Self::Diff => devicons::FileIcon::from("diff"),
            Self::Erlang => devicons::FileIcon::from("erl"),
            Self::HTML_Erlang => devicons::FileIcon::from("yaws"),
            Self::Git_Attributes => devicons::FileIcon::from("attributes"),
            Self::Git_Commit => devicons::FileIcon::from("COMMIT_EDITMSG"),
            Self::Git_Config => devicons::FileIcon::from("gitconfig"),
            Self::Git_Ignore => devicons::FileIcon::from("exclude"),
            Self::Git_Link => devicons::FileIcon::from(".git"),
            Self::Git_Log => devicons::FileIcon::from("gitlog"),
            Self::Git_Mailmap => devicons::FileIcon::from(".mailmap"),
            Self::Git_Rebase_Todo => devicons::FileIcon::from("git-rebase-todo"),
            Self::Go => devicons::FileIcon::from("go"),
            Self::Graphviz_DOT => devicons::FileIcon::from("dot"),
            Self::Groovy => devicons::FileIcon::from("groovy"),
            Self::HTML => devicons::FileIcon::from("html"),
            Self::Haskell => devicons::FileIcon::from("hs"),
            Self::Literate_Haskell => devicons::FileIcon::from("lhs"),
            Self::JSON => devicons::FileIcon::from("json"),
            Self::Java_Server_Page_JSP => devicons::FileIcon::from("jsp"),
            Self::Java => devicons::FileIcon::from("java"),
            Self::Java_Properties => devicons::FileIcon::from("properties"),
            Self::JavaScript => devicons::FileIcon::from("js"),
            Self::BibTeX => devicons::FileIcon::from("bib"),
            Self::LaTeX => devicons::FileIcon::from("tex"),
            Self::TeX => devicons::FileIcon::from("sty"),
            Self::Lisp => devicons::FileIcon::from("lisp"),
            Self::Lua => devicons::FileIcon::from("lua"),
            Self::Makefile => devicons::FileIcon::from("make"),
            Self::Markdown => devicons::FileIcon::from("md"),
            Self::MATLAB => devicons::FileIcon::from("matlab"),
            Self::OCaml => devicons::FileIcon::from("ml"),
            Self::OCamllex => devicons::FileIcon::from("mll"),
            Self::OCamlyacc => devicons::FileIcon::from("mly"),
            Self::Objective_Cpp => devicons::FileIcon::from("mm"),
            Self::Objective_C => devicons::FileIcon::from("m"),
            Self::PHP => devicons::FileIcon::from("php"),
            Self::Pascal => devicons::FileIcon::from("pas"),
            Self::Perl => devicons::FileIcon::from("pl"),
            Self::Python => devicons::FileIcon::from("py"),
            Self::R => devicons::FileIcon::from("R"),
            Self::Rd_R_Documentation => devicons::FileIcon::from("rd"),
            Self::HTML_Rails => devicons::FileIcon::from("rails"),
            Self::JavaScript_Rails => devicons::FileIcon::from("js.erb"),
            Self::Ruby_Haml => devicons::FileIcon::from("haml"),
            Self::Ruby_on_Rails => devicons::FileIcon::from("rxml"),
            Self::SQL_Rails => devicons::FileIcon::from("erbsql"),
            Self::Regular_Expression => devicons::FileIcon::from("re"),
            Self::ReStructuredText => devicons::FileIcon::from("rst"),
            Self::Ruby => devicons::FileIcon::from("rb"),
            Self::Rust => devicons::FileIcon::from("rs"),
            Self::SQL => devicons::FileIcon::from("sql"),
            Self::Scala => devicons::FileIcon::from("scala"),
            Self::Bourne_Again_Shell_bash => devicons::FileIcon::from("sh"),
            Self::HTML_Tcl => devicons::FileIcon::from("adp"),
            Self::Tcl => devicons::FileIcon::from("tcl"),
            Self::Textile => devicons::FileIcon::from("textile"),
            Self::XML => devicons::FileIcon::from("xml"),
            Self::YAML => devicons::FileIcon::from("yaml"),
            Self::AWK => devicons::FileIcon::from("awk"),
            Self::Ada => devicons::FileIcon::from("adb"),
            Self::Apache_Conf => devicons::FileIcon::from("envvars"),
            Self::AsciiDoc_Asciidoctor => devicons::FileIcon::from("adoc"),
            Self::ARM_Assembly => devicons::FileIcon::from("s"),
            Self::Assembly_x86_64 => devicons::FileIcon::from("yasm"),
            Self::CMake_C_Header => devicons::FileIcon::from("h.in"),
            Self::CMake_Cpp_Header => devicons::FileIcon::from("hh.in"),
            Self::CMake => devicons::FileIcon::from("CMakeLists.txt"),
            Self::CMakeCache => devicons::FileIcon::from("CMakeCache.txt"),
            Self::Comma_Separated_Values => devicons::FileIcon::from("csv"),
            Self::Cabal => devicons::FileIcon::from("cabal"),
            Self::CoffeeScript => devicons::FileIcon::from("coffee"),
            Self::CpuInfo => devicons::FileIcon::from("cpuinfo"),
            Self::Crontab => devicons::FileIcon::from("tab"),
            Self::Crystal => devicons::FileIcon::from("cr"),
            Self::Dart => devicons::FileIcon::from("dart"),
            Self::Dockerfile => devicons::FileIcon::from("Dockerfile"),
            Self::DotENV => devicons::FileIcon::from(".env"),
            Self::Elixir => devicons::FileIcon::from("ex"),
            Self::HTML_EEx => devicons::FileIcon::from("html.eex"),
            Self::Regular_Expressions_Elixir => devicons::FileIcon::from("ex.re"),
            Self::Elm => devicons::FileIcon::from("elm"),
            Self::Email => devicons::FileIcon::from("eml"),
            Self::FSharp => devicons::FileIcon::from("fs"),
            Self::Fish => devicons::FileIcon::from("fish"),
            Self::Fortran_Fixed_Form => devicons::FileIcon::from("f"),
            Self::Fortran_Modern => devicons::FileIcon::from("f90"),
            Self::Fortran_Namelist => devicons::FileIcon::from("namelist"),
            Self::Fstab => devicons::FileIcon::from("fstab"),
            Self::GLSL => devicons::FileIcon::from("vs"),
            Self::GraphQL => devicons::FileIcon::from("graphql"),
            Self::Groff_troff => devicons::FileIcon::from("groff"),
            Self::Group => devicons::FileIcon::from("group"),
            Self::HTML_Twig => devicons::FileIcon::from("twig"),
            Self::Hosts => devicons::FileIcon::from("hosts"),
            Self::INI => devicons::FileIcon::from("ini"),
            Self::JavaScript_Babel => devicons::FileIcon::from("js"),
            Self::HTML_Jinja2 => devicons::FileIcon::from("htm.j2"),
            Self::Jinja2 => devicons::FileIcon::from("j2"),
            Self::Jsonnet => devicons::FileIcon::from("jsonnet"),
            Self::Julia => devicons::FileIcon::from("jl"),
            Self::Kotlin => devicons::FileIcon::from("kt"),
            Self::Less => devicons::FileIcon::from("less"),
            Self::LLVM => devicons::FileIcon::from("ll"),
            Self::Lean => devicons::FileIcon::from("lean"),
            Self::LiveScript => devicons::FileIcon::from("ls"),
            Self::Manpage => devicons::FileIcon::from("man"),
            Self::MediaWiki => devicons::FileIcon::from("mediawiki"),
            Self::MemInfo => devicons::FileIcon::from("meminfo"),
            Self::Nginx => devicons::FileIcon::from("conf.erb"),
            Self::Nim => devicons::FileIcon::from("nim"),
            Self::Ninja => devicons::FileIcon::from("ninja"),
            Self::Nix => devicons::FileIcon::from("nix"),
            Self::Orgmode => devicons::FileIcon::from("org"),
            Self::Passwd => devicons::FileIcon::from("passwd"),
            Self::PowerShell => devicons::FileIcon::from("ps1"),
            Self::Protocol_Buffer => devicons::FileIcon::from("proto"),
            Self::Protocol_Buffer_TEXT => devicons::FileIcon::from("pb.txt"),
            Self::Puppet => devicons::FileIcon::from("pp"),
            Self::PureScript => devicons::FileIcon::from("purs"),
            Self::QML => devicons::FileIcon::from("qml"),
            Self::Racket => devicons::FileIcon::from("rkt"),
            Self::Rego => devicons::FileIcon::from("rego"),
            Self::RequirementsDottxt => devicons::FileIcon::from("requirements.txt"),
            Self::Resolv => devicons::FileIcon::from("resolv.conf"),
            Self::Robot_Framework => devicons::FileIcon::from("robot"),
            Self::SCSS => devicons::FileIcon::from("scss"),
            Self::Sass => devicons::FileIcon::from("sass"),
            Self::Salt_State_SLS => devicons::FileIcon::from("sls"),
            Self::SML => devicons::FileIcon::from("sml"),
            Self::Ruby_Slim => devicons::FileIcon::from("slim"),
            Self::Strace => devicons::FileIcon::from("strace"),
            Self::Stylus => devicons::FileIcon::from("styl"),
            Self::Solidity => devicons::FileIcon::from("sol"),
            Self::Vyper => devicons::FileIcon::from("vy"),
            Self::JQ => devicons::FileIcon::from("jq"),
            Self::Svelte => devicons::FileIcon::from("svlt"),
            Self::Swift => devicons::FileIcon::from("swift"),
            Self::SystemVerilog => devicons::FileIcon::from("sv"),
            Self::TOML => devicons::FileIcon::from("toml"),
            Self::JSON_Terraform => devicons::FileIcon::from("tfstate"),
            Self::Terraform => devicons::FileIcon::from("tf"),
            Self::TodoDottxt => devicons::FileIcon::from("todo.txt"),
            Self::TypeScript => devicons::FileIcon::from("ts"),
            Self::TypeScriptReact => devicons::FileIcon::from("tsx"),
            Self::Verilog => devicons::FileIcon::from("v"),
            Self::VimHelp => devicons::FileIcon::from("vimhelp"),
            Self::VimL => devicons::FileIcon::from("vim"),
            Self::Vue_Component => devicons::FileIcon::from("vue"),
            Self::Zig => devicons::FileIcon::from("zig"),
            Self::Command_Help => devicons::FileIcon::from("cmd-help"),
            Self::Gnuplot => devicons::FileIcon::from("gp"),
            Self::HTTP_Request_and_Response => devicons::FileIcon::from("http"),
            Self::Log => devicons::FileIcon::from("log"),
            Self::Highlight_non_printables => devicons::FileIcon::from("show-nonprintable"),
            Self::Authorized_Keys => devicons::FileIcon::from("authorized_keys"),
            Self::Known_Hosts => devicons::FileIcon::from("known_hosts"),
            Self::SSH_Config => devicons::FileIcon::from("ssh_config"),
            Self::SSHD_Config => devicons::FileIcon::from("sshd_config"),
            Self::Syslog => devicons::FileIcon::from("syslog"),
            Self::Varlink => devicons::FileIcon::from("varlink"),
            // TODO: Actually create a Conundrum logo and add that guy here...
            Self::ConundrumAi => devicons::FileIcon::from("txt"),
            Self::Dictionary => devicons::FileIcon::from("txt"),
            _ => devicons::FileIcon::from("txt"),
        }
    }

    /// Since markdown rendering is completely left up to the platform, and with
    /// that, the languages they support and the keys that they use, this
    /// function attempts to sacrifice some highlighter accuracy for some
    /// more generic syntaxes that are more likely to be
    /// supported elsewhere.
    pub fn markdown_representation(&self) -> String {
        match self {
            SupportedCodeBlockSyntax::Plain_Text => "txt".to_string(),
            SupportedCodeBlockSyntax::ASP => "asa".to_string(),
            SupportedCodeBlockSyntax::HTML_ASP => "asp".to_string(),
            SupportedCodeBlockSyntax::ActionScript => "as".to_string(),
            SupportedCodeBlockSyntax::AppleScript => "applescript".to_string(),
            SupportedCodeBlockSyntax::Batch_File => "bat".to_string(),
            SupportedCodeBlockSyntax::NAnt_Build_File => "build".to_string(),
            SupportedCodeBlockSyntax::CSharp => "cs".to_string(),
            SupportedCodeBlockSyntax::Cpp => "cpp".to_string(),
            SupportedCodeBlockSyntax::C => "c".to_string(),
            SupportedCodeBlockSyntax::CSS => "css".to_string(),
            SupportedCodeBlockSyntax::Clojure => "clj".to_string(),
            SupportedCodeBlockSyntax::D => "d".to_string(),
            SupportedCodeBlockSyntax::Diff => "diff".to_string(),
            SupportedCodeBlockSyntax::Erlang => "erl".to_string(),
            SupportedCodeBlockSyntax::HTML_Erlang => "yaws".to_string(),
            SupportedCodeBlockSyntax::Git_Attributes => "attributes".to_string(),
            SupportedCodeBlockSyntax::Git_Commit => "COMMIT_EDITMSG".to_string(),
            SupportedCodeBlockSyntax::Git_Config => "gitconfig".to_string(),
            SupportedCodeBlockSyntax::Git_Ignore => "exclude".to_string(),
            SupportedCodeBlockSyntax::Git_Link => ".git".to_string(),
            SupportedCodeBlockSyntax::Git_Log => "gitlog".to_string(),
            SupportedCodeBlockSyntax::Git_Mailmap => ".mailmap".to_string(),
            SupportedCodeBlockSyntax::Git_Rebase_Todo => "git-rebase-todo".to_string(),
            SupportedCodeBlockSyntax::Go => "go".to_string(),
            SupportedCodeBlockSyntax::Graphviz_DOT => "dot".to_string(),
            SupportedCodeBlockSyntax::Groovy => "groovy".to_string(),
            SupportedCodeBlockSyntax::HTML => "html".to_string(),
            SupportedCodeBlockSyntax::Haskell => "hs".to_string(),
            SupportedCodeBlockSyntax::Literate_Haskell => "lhs".to_string(),
            SupportedCodeBlockSyntax::JSON => "json".to_string(),
            SupportedCodeBlockSyntax::Java_Server_Page_JSP => "jsp".to_string(),
            SupportedCodeBlockSyntax::Java => "java".to_string(),
            SupportedCodeBlockSyntax::Java_Properties => "properties".to_string(),
            SupportedCodeBlockSyntax::JavaScript => "js".to_string(),
            SupportedCodeBlockSyntax::BibTeX => "bib".to_string(),
            SupportedCodeBlockSyntax::LaTeX => "tex".to_string(),
            SupportedCodeBlockSyntax::TeX => "sty".to_string(),
            SupportedCodeBlockSyntax::Lisp => "lisp".to_string(),
            SupportedCodeBlockSyntax::Lua => "lua".to_string(),
            SupportedCodeBlockSyntax::Makefile => "make".to_string(),
            SupportedCodeBlockSyntax::Markdown => "md".to_string(),
            SupportedCodeBlockSyntax::MATLAB => "matlab".to_string(),
            SupportedCodeBlockSyntax::OCaml => "ml".to_string(),
            SupportedCodeBlockSyntax::OCamllex => "mll".to_string(),
            SupportedCodeBlockSyntax::OCamlyacc => "mly".to_string(),
            SupportedCodeBlockSyntax::Objective_Cpp => "mm".to_string(),
            SupportedCodeBlockSyntax::Objective_C => "m".to_string(),
            SupportedCodeBlockSyntax::PHP => "php".to_string(),
            SupportedCodeBlockSyntax::Pascal => "pas".to_string(),
            SupportedCodeBlockSyntax::Perl => "pl".to_string(),
            SupportedCodeBlockSyntax::Python => "py".to_string(),
            SupportedCodeBlockSyntax::R => "R".to_string(),
            SupportedCodeBlockSyntax::Rd_R_Documentation => "rd".to_string(),
            SupportedCodeBlockSyntax::HTML_Rails => "rails".to_string(),
            SupportedCodeBlockSyntax::JavaScript_Rails => "js.erb".to_string(),
            SupportedCodeBlockSyntax::Ruby_Haml => "haml".to_string(),
            SupportedCodeBlockSyntax::Ruby_on_Rails => "rxml".to_string(),
            SupportedCodeBlockSyntax::SQL_Rails => "erbsql".to_string(),
            SupportedCodeBlockSyntax::Regular_Expression => "re".to_string(),
            SupportedCodeBlockSyntax::ReStructuredText => "rst".to_string(),
            SupportedCodeBlockSyntax::Ruby => "rb".to_string(),
            SupportedCodeBlockSyntax::Rust => "rs".to_string(),
            SupportedCodeBlockSyntax::SQL => "sql".to_string(),
            SupportedCodeBlockSyntax::Scala => "scala".to_string(),
            SupportedCodeBlockSyntax::Bourne_Again_Shell_bash => "sh".to_string(),
            SupportedCodeBlockSyntax::HTML_Tcl => "adp".to_string(),
            SupportedCodeBlockSyntax::Tcl => "tcl".to_string(),
            SupportedCodeBlockSyntax::Textile => "textile".to_string(),
            SupportedCodeBlockSyntax::XML => "xml".to_string(),
            SupportedCodeBlockSyntax::YAML => "yaml".to_string(),
            SupportedCodeBlockSyntax::AWK => "awk".to_string(),
            SupportedCodeBlockSyntax::Ada => "adb".to_string(),
            SupportedCodeBlockSyntax::Apache_Conf => "envvars".to_string(),
            SupportedCodeBlockSyntax::AsciiDoc_Asciidoctor => "adoc".to_string(),
            SupportedCodeBlockSyntax::ARM_Assembly => "s".to_string(),
            SupportedCodeBlockSyntax::Assembly_x86_64 => "yasm".to_string(),
            SupportedCodeBlockSyntax::CMake_C_Header => "h.in".to_string(),
            SupportedCodeBlockSyntax::CMake_Cpp_Header => "hh.in".to_string(),
            SupportedCodeBlockSyntax::CMake => "CMakeLists.txt".to_string(),
            SupportedCodeBlockSyntax::CMakeCache => "CMakeCache.txt".to_string(),
            SupportedCodeBlockSyntax::Comma_Separated_Values => "csv".to_string(),
            SupportedCodeBlockSyntax::Cabal => "cabal".to_string(),
            SupportedCodeBlockSyntax::CoffeeScript => "coffee".to_string(),
            SupportedCodeBlockSyntax::CpuInfo => "cpuinfo".to_string(),
            SupportedCodeBlockSyntax::Crontab => "tab".to_string(),
            SupportedCodeBlockSyntax::Crystal => "cr".to_string(),
            SupportedCodeBlockSyntax::Dart => "dart".to_string(),
            SupportedCodeBlockSyntax::Dockerfile => "Dockerfile".to_string(),
            SupportedCodeBlockSyntax::DotENV => ".env".to_string(),
            SupportedCodeBlockSyntax::Elixir => "ex".to_string(),
            SupportedCodeBlockSyntax::HTML_EEx => "html.eex".to_string(),
            SupportedCodeBlockSyntax::Regular_Expressions_Elixir => "ex.re".to_string(),
            SupportedCodeBlockSyntax::Elm => "elm".to_string(),
            SupportedCodeBlockSyntax::Email => "eml".to_string(),
            SupportedCodeBlockSyntax::FSharp => "fs".to_string(),
            SupportedCodeBlockSyntax::Fish => "fish".to_string(),
            SupportedCodeBlockSyntax::Fortran_Fixed_Form => "f".to_string(),
            SupportedCodeBlockSyntax::Fortran_Modern => "f90".to_string(),
            SupportedCodeBlockSyntax::Fortran_Namelist => "namelist".to_string(),
            SupportedCodeBlockSyntax::Fstab => "fstab".to_string(),
            SupportedCodeBlockSyntax::GLSL => "vs".to_string(),
            SupportedCodeBlockSyntax::GraphQL => "graphql".to_string(),
            SupportedCodeBlockSyntax::Groff_troff => "groff".to_string(),
            SupportedCodeBlockSyntax::Group => "group".to_string(),
            SupportedCodeBlockSyntax::HTML_Twig => "twig".to_string(),
            SupportedCodeBlockSyntax::Hosts => "hosts".to_string(),
            SupportedCodeBlockSyntax::INI => "ini".to_string(),
            SupportedCodeBlockSyntax::JavaScript_Babel => "js".to_string(),
            SupportedCodeBlockSyntax::HTML_Jinja2 => "htm.j2".to_string(),
            SupportedCodeBlockSyntax::Jinja2 => "j2".to_string(),
            SupportedCodeBlockSyntax::Jsonnet => "jsonnet".to_string(),
            SupportedCodeBlockSyntax::Julia => "jl".to_string(),
            SupportedCodeBlockSyntax::Kotlin => "kt".to_string(),
            SupportedCodeBlockSyntax::Less => "less".to_string(),
            SupportedCodeBlockSyntax::LLVM => "ll".to_string(),
            SupportedCodeBlockSyntax::Lean => "lean".to_string(),
            SupportedCodeBlockSyntax::LiveScript => "ls".to_string(),
            SupportedCodeBlockSyntax::Manpage => "man".to_string(),
            SupportedCodeBlockSyntax::MediaWiki => "mediawiki".to_string(),
            SupportedCodeBlockSyntax::MemInfo => "meminfo".to_string(),
            SupportedCodeBlockSyntax::Nginx => "conf.erb".to_string(),
            SupportedCodeBlockSyntax::Nim => "nim".to_string(),
            SupportedCodeBlockSyntax::Ninja => "ninja".to_string(),
            SupportedCodeBlockSyntax::Nix => "nix".to_string(),
            SupportedCodeBlockSyntax::Orgmode => "org".to_string(),
            SupportedCodeBlockSyntax::Passwd => "passwd".to_string(),
            SupportedCodeBlockSyntax::PowerShell => "ps1".to_string(),
            SupportedCodeBlockSyntax::Protocol_Buffer => "proto".to_string(),
            SupportedCodeBlockSyntax::Protocol_Buffer_TEXT => "pb.txt".to_string(),
            SupportedCodeBlockSyntax::Puppet => "pp".to_string(),
            SupportedCodeBlockSyntax::PureScript => "purs".to_string(),
            SupportedCodeBlockSyntax::QML => "qml".to_string(),
            SupportedCodeBlockSyntax::Racket => "rkt".to_string(),
            SupportedCodeBlockSyntax::Rego => "rego".to_string(),
            SupportedCodeBlockSyntax::RequirementsDottxt => "requirements.txt".to_string(),
            SupportedCodeBlockSyntax::Resolv => "resolv.conf".to_string(),
            SupportedCodeBlockSyntax::Robot_Framework => "robot".to_string(),
            SupportedCodeBlockSyntax::SCSS => "scss".to_string(),
            SupportedCodeBlockSyntax::Sass => "sass".to_string(),
            SupportedCodeBlockSyntax::Salt_State_SLS => "sls".to_string(),
            SupportedCodeBlockSyntax::SML => "sml".to_string(),
            SupportedCodeBlockSyntax::Ruby_Slim => "slim".to_string(),
            SupportedCodeBlockSyntax::Strace => "strace".to_string(),
            SupportedCodeBlockSyntax::Stylus => "styl".to_string(),
            SupportedCodeBlockSyntax::Solidity => "sol".to_string(),
            SupportedCodeBlockSyntax::Vyper => "vy".to_string(),
            SupportedCodeBlockSyntax::JQ => "jq".to_string(),
            SupportedCodeBlockSyntax::Svelte => "svlt".to_string(),
            SupportedCodeBlockSyntax::Swift => "swift".to_string(),
            SupportedCodeBlockSyntax::SystemVerilog => "sv".to_string(),
            SupportedCodeBlockSyntax::TOML => "toml".to_string(),
            SupportedCodeBlockSyntax::JSON_Terraform => "tfstate".to_string(),
            SupportedCodeBlockSyntax::Terraform => "tf".to_string(),
            SupportedCodeBlockSyntax::TodoDottxt => "todo.txt".to_string(),
            SupportedCodeBlockSyntax::TypeScript => "ts".to_string(),
            SupportedCodeBlockSyntax::TypeScriptReact => "tsx".to_string(),
            SupportedCodeBlockSyntax::Verilog => "v".to_string(),
            SupportedCodeBlockSyntax::VimHelp => "vimhelp".to_string(),
            SupportedCodeBlockSyntax::VimL => "vim".to_string(),
            SupportedCodeBlockSyntax::Vue_Component => "vue".to_string(),
            SupportedCodeBlockSyntax::Zig => "zig".to_string(),
            SupportedCodeBlockSyntax::Command_Help => "cmd-help".to_string(),
            SupportedCodeBlockSyntax::Gnuplot => "gp".to_string(),
            SupportedCodeBlockSyntax::HTTP_Request_and_Response => "http".to_string(),
            SupportedCodeBlockSyntax::Log => "log".to_string(),
            SupportedCodeBlockSyntax::Highlight_non_printables => "show-nonprintable".to_string(),
            SupportedCodeBlockSyntax::Authorized_Keys => "authorized_keys".to_string(),
            SupportedCodeBlockSyntax::Known_Hosts => "known_hosts".to_string(),
            SupportedCodeBlockSyntax::SSH_Config => "ssh_config".to_string(),
            SupportedCodeBlockSyntax::SSHD_Config => "sshd_config".to_string(),
            SupportedCodeBlockSyntax::Syslog => "syslog".to_string(),
            SupportedCodeBlockSyntax::Varlink => "varlink".to_string(),
            _ => "txt".to_string(),
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

    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn gets_basic_languages() {
        SupportedCodeBlockSyntax::from_str("javascript").expect("Parses alias successfully.");
    }

    #[test]
    fn renders_all_icons() {
        SupportedCodeBlockSyntax::iter().for_each(|s| {
                                            let svg = s.devicon(&true);
                                            println!("Svg (dark) for {}: {}", s, svg);
                                        });

        SupportedCodeBlockSyntax::iter().for_each(|s| {
                                            let svg = s.devicon(&false);
                                            println!("Svg (light) for {}: {}", s, svg);
                                        });
    }
}

