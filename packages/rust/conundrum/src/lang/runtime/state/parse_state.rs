use std::sync::Arc;

use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use syntect_assets::assets::HighlightingAssets;
use typeshare::typeshare;
use uniffi::Enum;

use crate::{
    lang::runtime::state::{citation_list::CitationList, dom_data::DomData, ui_params::UIParams},
    output::{
        general::component_constants::parser_ids::ParserId, parsing_result::mdx_parsing_result::MdxParsingResult,
    },
    parsers::markdown::heading_sluggger::Slugger,
};

/// The goal with these modifiers is to have a few different compile targets
/// (markdown, mdx, jsx, eventually straight to html/js, and then to an AST that
/// can be handled by other UI ecosystems), and these modifiers is how we can
/// achieve that.
///
/// All modifiers that start with `TargetXYZ` are applied as sort of a
/// collection of other modifiers, or at least that's how they're intended to
/// work.
#[typeshare::typeshare]
#[derive(Enum, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum ConundrumModifier {
    HideEmojis,
    /// The goal with this flag is to make **some** components collapsable to be
    /// inline, even when they traditionally are not. This will likely be
    /// buggy, producing some good output in some cases but some
    /// questionable output in others.
    ///
    /// As the list of component properties grows, this output will become
    /// customizable directly in your note.
    PreferInlineMarkdownSyntax,
    /// This is really only useful for when your environment can't support any
    /// other output format.
    DecoratedPlainText,
    /// Set this flag when the output is intended to be consumed by AI, probably
    /// with the `.PreferMarkdownSyntax` flag.
    ForAIInput,
    /// When this component is to be used for search text input, all of the
    /// component jsx and mdx syntax will be removed, leaving only
    /// searchable text.
    ForSearchInput,
    /// Don't touch the code blocks, just return them exactly as is.
    CodeBlocksAsIs,
    /// Output a standalone application capable of being viewed completely
    /// offline, with all fonts, javascript and rendered math embedded directly
    /// into a single html file.
    Standalone,
    /// This is currently required in React environments to work around the
    /// React render cycle. Since almost everything is wrapepd in a series
    /// of iiife's, kind of working like React's useEffect hook, embedding
    /// the javascript as a script tag will ensure that the components
    /// all have the proper listeners applied. So far it comes out to only about
    /// 200 lines of javascript.
    EmbedJavascript,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, uniffi::Enum, Default, Clone, Eq, PartialEq)]
pub enum ConundrumCompileTarget {
    Jsx,
    #[default]
    Html,
    Markdown,
    PlainText,
    Mdx,
}

#[derive(Debug, Clone)]
pub struct ParseState {
    pub data: MdxParsingResult,
    pub bib: CitationList,
    // TODO: Convert this to a map. Allow users to pass things in as a list of enums, but it
    // should be converted to a map in the `run_conundrum` function before everything else
    // executes for quick lookup, since there is a **lot** of lookup in this list.
    pub modifiers: Vec<ConundrumModifier>,
    /// The equation count, for those sweet numbered equations..
    pub eq_count: u32,
    pub slugger: Slugger,
    pub last_heading_depth: u16,
    pub last_heading_tab_depth: u16,
    /// A list of footnote indices that is appended to during parsing, and then
    /// validated against during compilation.
    pub valid_footnote_indices: Vec<u32>,
    pub ui_params: UIParams,
    pub dom: DomData,
    pub compile_target: ConundrumCompileTarget,
    pub trusted: bool,
    pub highlight_assets: Arc<Mutex<HighlightingAssets>>,
}

impl Default for ParseState {
    fn default() -> Self {
        Self { data: Default::default(),
               bib: Default::default(),
               modifiers: Default::default(),
               eq_count: Default::default(),
               slugger: Default::default(),
               last_heading_depth: Default::default(),
               last_heading_tab_depth: Default::default(),
               valid_footnote_indices: Default::default(),
               ui_params: Default::default(),
               dom: Default::default(),
               compile_target: Default::default(),
               trusted: Default::default(),
               highlight_assets: Arc::new(Mutex::new(HighlightingAssets::from_binary())) }
    }
}

impl ParseState {
    /// Applies the nested state found within a child element to the parent
    /// state.
    pub fn apply_nested_state(&mut self, s: Self) {
        s.data.tags.iter().for_each(|tag| self.data.tags.push(tag.clone()));

        s.data.outgoing_links.iter().for_each(|ol| self.data.outgoing_links.push(ol.clone()));

        if s.data.ignore_all_parsers {
            self.data.ignore_all_parsers = true;
        }

        s.data.ai_secondary_parse_requests.iter().for_each(|pr| {
                                                     self.data.ai_secondary_parse_requests.push(pr.clone());
                                                 });

        s.data.dictionary_entries.iter().for_each(|dict| {
                                            self.data.dictionary_entries.push(dict.clone());
                                        })
    }

    pub fn contains_one_of_modifiers(&self, modifiers: Vec<ConundrumModifier>) -> bool {
        for modifier in modifiers {
            if self.contains_modifier(&modifier) {
                return true;
            }
        }
        false
    }

    pub fn targets_one_of(&self, targets: Vec<ConundrumCompileTarget>) -> bool {
        for t in targets {
            if self.compile_target == t {
                return true;
            }
        }
        false
    }

    /// A simple utility wrapper around the `TargetXYZ` flags.
    pub fn targets_jsx(&self) -> bool {
        self.compile_target == ConundrumCompileTarget::Jsx
    }

    /// A simple utility wrapper around the `TargetXYZ` flags.
    pub fn targets_html_js(&self) -> bool {
        self.compile_target == ConundrumCompileTarget::Html
    }

    /// A simple utility wrapper around the `TargetXYZ` flags.
    pub fn targets_mdx(&self) -> bool {
        self.compile_target == ConundrumCompileTarget::Mdx
    }

    /// Will be `true` for either inline or normal markdown.
    pub fn targets_markdown(&self) -> bool {
        self.compile_target == ConundrumCompileTarget::Markdown
    }

    /// Will be `true` when the flags indicate that the output is either for
    /// markdown, for search input, or to be consumed by AI.
    pub fn is_markdown_or_search_or_ai(&self) -> bool {
        if self.targets_markdown() {
            true
        } else {
            self.contains_one_of_modifiers(vec![ConundrumModifier::PreferInlineMarkdownSyntax,
                                                ConundrumModifier::ForAIInput,
                                                ConundrumModifier::ForSearchInput])
        }
    }

    /// Will be `true` when the flags indicate that the output is either for
    /// markdown or for plain text.
    pub fn is_markdown_or_plain_text(&self) -> bool {
        if self.targets_markdown() {
            true
        } else {
            self.targets_one_of(vec![ConundrumCompileTarget::Markdown, ConundrumCompileTarget::PlainText])
        }
    }

    /// A lazy utility to check if the modifiers list indicates that the output
    /// is targeting a plain text environment.
    pub fn is_plain_text(&self) -> bool {
        self.compile_target == ConundrumCompileTarget::PlainText
    }

    pub fn is_standalone(&self) -> bool {
        self.contains_modifier(&ConundrumModifier::Standalone)
    }

    pub fn contains_modifier(&self, modifier: &ConundrumModifier) -> bool {
        self.modifiers.iter().any(|x| x == modifier)
    }

    pub fn contains_modifier_or_matches_target(&self,
                                               modifiers: Vec<ConundrumModifier>,
                                               targets: Vec<ConundrumCompileTarget>)
                                               -> bool {
        if self.contains_one_of_modifiers(modifiers) {
            return true;
        } else {
            for t in targets {
                if t == self.compile_target {
                    return true;
                }
            }
        }
        false
    }

    pub fn should_ignore_parser(&self, id: &ParserId) -> bool {
        self.data.front_matter.as_ref().is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &id.to_string()))
    }
}
