use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uniffi::Enum;

use crate::{
    lang::runtime::state::citation_list::CitationList,
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
    /// This might as well be called `TargetMarkdown`, but this was created
    /// before the idea of 'targets' came to be merged with 'modifiers' and I'm
    /// in too much of a hurry to change it now.
    ///
    /// As with the `.PreferInlineMarkdownSyntax` flag, many components allow
    /// you to customize the bahavior of each component:
    ///
    /// ```jsx
    /// <Card
    ///     title="My title"
    ///     /// Set the heading title depth.
    ///     markdownHeading={3}
    /// >
    /// ...
    /// </Card>
    /// ```
    PreferMarkdownSyntax,
    /// Hide the emojis completely for platforms that don't support them.
    HideEmojis,
    /// Curretly does pretty much the same thing as .PreferMarkdownSyntax, but
    /// once the markdown parser has been completely migrated this will stop
    /// things like wrapping the outermost block in a paragraph.
    ///
    /// The goal with this is to behave much like SwiftUI's markdown support,
    /// with the ability to render only inline markdown for things like
    /// titles where a full `Admonition` wouldn't make sense.
    ///
    /// Keep in mind that many components allow you to customize the
    /// **markdown** output as well as the html/js output. You can do the
    /// following:
    ///
    /// ```jsx
    /// <Hl markdown="italic" highlight>My text</Hl>
    /// ```
    PreferInlineMarkdownSyntax,
    /// Useful for search related features, being able to match text without
    /// markdown syntax interfering. Not super useful for much else.
    ForcePlainText,
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
    /// Leave the markdown output mostly alone and render to mdx instead of jsx.
    /// This is a super easy gateway for developers to build around
    /// Conundrum as the compile target is **super** forgiving.
    ///
    /// This is the default target for now, but the current goal is to finish
    /// rest of the parser, which would make the dependence on mdx obsolete.
    TargetMdx,
    /// This is the goal, but this is still _super_ buggy and should in no way
    /// be used in any application that you expect someone to actually use,
    /// but we'll get there...
    TargetJsx,
    /// This is even more of a futuristic goal, but for ultimate performance we
    /// can even leave behind React. There are other priorities for now, but
    /// this is something that will naturally come together as the framework
    /// progresses.
    TargetHtmlJs,
}

#[derive(Debug, Default, Clone)]
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

    /// A simple utility wrapper around the `TargetXYZ` flags.
    pub fn targets_jsx(&self) -> bool {
        self.contains_modifier(&ConundrumModifier::TargetJsx)
    }

    /// A simple utility wrapper around the `TargetXYZ` flags.
    pub fn targets_html_js(&self) -> bool {
        self.contains_modifier(&ConundrumModifier::TargetHtmlJs)
    }

    /// A simple utility wrapper around the `TargetXYZ` flags.
    pub fn targets_mdx(&self) -> bool {
        self.contains_modifier(&ConundrumModifier::TargetMdx)
    }

    /// Will be `true` for either inline or normal markdown.
    pub fn is_markdown(&self) -> bool {
        self.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                            ConundrumModifier::PreferInlineMarkdownSyntax])
    }

    /// Will be `true` when the flags indicate that the output is either for
    /// markdown, for search input, or to be consumed by AI.
    pub fn is_markdown_or_search_or_ai(&self) -> bool {
        self.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                            ConundrumModifier::PreferInlineMarkdownSyntax,
                                            ConundrumModifier::ForAIInput,
                                            ConundrumModifier::ForSearchInput])
    }

    /// Will be `true` when the flags indicate that the output is either for
    /// markdown or for plain text.
    pub fn is_markdown_or_plain_text(&self) -> bool {
        self.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                            ConundrumModifier::PreferInlineMarkdownSyntax,
                                            ConundrumModifier::ForcePlainText])
    }

    /// A lazy utility to check if the modifiers list indicates that the output
    /// is targeting a plain text environment.
    pub fn is_plain_text(&self) -> bool {
        self.contains_one_of_modifiers(vec![ConundrumModifier::ForcePlainText])
    }

    pub fn contains_modifier(&self, modifier: &ConundrumModifier) -> bool {
        self.modifiers.iter().any(|x| x == modifier)
    }

    pub fn should_ignore_parser(&self, id: &ParserId) -> bool {
        self.data.front_matter.as_ref().is_some_and(|fm| fm.ignored_parsers.iter().any(|x| x == &id.to_string()))
    }
}
