use std::cell::RefCell;

use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use serde::Serialize;
use winnow::{
    ModalResult, Parser, Stateful,
    ascii::{line_ending, space0, till_line_ending},
    combinator::{alt, eof, repeat},
    stream::Stream,
    token::{literal, take_while},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            state::parse_state::ParseState,
            traits::{
                conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::parsing_result::mdx_parsing_result::MdxParsingResult,
    parsers::parser_trait::ConundrumParser,
};

// ---------------------------------------------------------------------------
// Result type
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct BlockQuoteResult {
    /// The inner content already fully parsed into `ParsedElement`s.
    /// Nesting is handled recursively: a `> > ...` line becomes a
    /// `BlockQuote` variant inside this `Vec`.
    pub children: Vec<ParsedElement>,
    /// The full original source text that was consumed.
    pub full_match: String,
}

// ---------------------------------------------------------------------------
// MdxComponentResult
// ---------------------------------------------------------------------------

impl MdxComponentResult for BlockQuoteResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        // Recursively render inner elements through the same MdxParsingResult
        // so side-effects (citations, tags, etc.) are collected correctly.
        let children_string = compile_elements(&self.children, res);

        format!("\n<{component}>\n{inner}\n</{component}>\n",
                component = AutoInsertedComponentName::AutoInsertedBlockQuote,
                inner = children_string,)
    }
}

// ---------------------------------------------------------------------------
// Line-level helper
// ---------------------------------------------------------------------------

/// Parses a single block-quote line and returns the body after the `>` marker.
///
/// Grammar (GFM spec §5.1):
///   0-3-spaces  `>`  optional-space  rest-of-line  line-ending-or-eof
fn parse_bq_line(input: &mut ConundrumInput) -> ModalResult<String> {
    let _ = take_while(0..=3, |c: char| c == ' ').parse_next(input)?;
    let _ = literal(">").parse_next(input)?;
    let _ = space0.parse_next(input)?;
    let body = till_line_ending.parse_next(input)?;
    let _ = alt((line_ending, eof.value(""))).parse_next(input)?;
    Ok(body.to_string())
}

// ---------------------------------------------------------------------------
// ConundrumParser impl
// ---------------------------------------------------------------------------

impl ConundrumParser<BlockQuoteResult> for BlockQuoteResult {
    fn parse_input_string<'a>(input_outer: &mut ConundrumInput<'a>) -> ModalResult<BlockQuoteResult> {
        let (parsed_content, full_match) =
            (|input: &mut ConundrumInput<'a>| {
                let start = input.input.checkpoint();
                let _ = literal("\n").parse_next(input).inspect_err(|_| {
                                                            input.input.reset(&start);
                                                        })?;
                let lines: Vec<String> = repeat(1.., parse_bq_line).parse_next(input).inspect_err(|_| {
                                                                                          input.input.reset(&start);
                                                                                      })?;

                // Join stripped lines then recursively parse the inner content
                // so math, citations, nested block quotes, etc. are recognised.
                let inner_src = lines.join("\n");
                let mut new_state: Stateful<&str, RefCell<ParseState>> =
                    ConundrumInput { input: &inner_src,
                                     state: RefCell::new(ParseState::default()) };

                let new_parsed_content =
                    parse_elements(&mut new_state).unwrap_or_else(|_| vec![ParsedElement::Text(inner_src.clone())]);

                // WITH_WIFI: This needs to be applied here but I'm running into a type error
                // right now.
                // apply_nested_parser_state(input, &new_state);
                // RESUME: Pick back up here
                // by applying the nested
                // state to the parent state
                // via a trait method an
                // then move on to rest of
                // markdown parsers now that
                // nested parsing is working.
                // apply_nested_state(&mut input_outer.state, &mut new_state.state);

                Ok(new_parsed_content)
            }).with_taken()
              .parse_next(input_outer)?;

        Ok(BlockQuoteResult { children: parsed_content,
                              full_match: full_match.to_string() })
    }

    fn matches_first_char(char: char) -> bool {
        char == '>'
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        parsers::children_utils::get_children_content_text::get_children_content_text,
        testing::wrap_test_content::wrap_test_conundrum_content,
    };

    use super::*;

    #[test]
    fn simple_block_quote() {
        let input = "> Hello, world!\n";
        let mut test_props = wrap_test_conundrum_content(input);
        let res = BlockQuoteResult::parse_input_string(&mut test_props).expect("parses simple block quote");
        assert!(input.is_empty(), "all input consumed");
        assert_eq!(res.full_match, "> Hello, world!\n");
        assert_eq!(get_children_content_text(&res.children), "Hello, world!");
    }

    #[test]
    fn multi_line_block_quote() {
        let input = "> Line one\n> Line two\n> Line three\n";
        let mut test_props = wrap_test_conundrum_content(input);
        let res = BlockQuoteResult::parse_input_string(&mut test_props).expect("parses multi-line block quote");
        assert!(input.is_empty());
        assert_eq!(get_children_content_text(&res.children), "Line one\nLine two\nLine three");
    }

    #[test]
    fn nested_block_quote_produces_block_quote_element() {
        let input = "> Outer line\n> > Inner line\n";
        let mut test_props = wrap_test_conundrum_content(input);
        let res = BlockQuoteResult::parse_input_string(&mut test_props).expect("parses nested block quote");
        assert!(input.is_empty());
        let has_nested = res.children.iter().any(|e| matches!(e, ParsedElement::BlockQuote(_)));
        assert!(has_nested, "nested > produces a BlockQuote element");
    }

    #[test]
    fn gfm_alert_warning_multiline() {
        let input = "> [!WARNING]\n> First body line.\n> Second body line.\n";
        let mut test_props = wrap_test_conundrum_content(input);
        let res = BlockQuoteResult::parse_input_string(&mut test_props).expect("parses GFM WARNING alert");
        assert_eq!(get_children_content_text(&res.children), "First body line.\nSecond body line.");
    }

    #[test]
    fn block_quote_without_trailing_newline() {
        let input = "> No newline at end";
        let mut test_props = wrap_test_conundrum_content(input);
        let res = BlockQuoteResult::parse_input_string(&mut test_props).expect("parses block quote ending at EOF");
        assert!(input.is_empty());
        assert_eq!(get_children_content_text(&res.children), "No newline at end");
    }

    #[test]
    fn leading_spaces_allowed() {
        let input = "   > Indented marker\n";
        let mut test_props = wrap_test_conundrum_content(input);
        let res = BlockQuoteResult::parse_input_string(&mut test_props).expect("parses indented block quote marker");
        assert_eq!(get_children_content_text(&res.children), "Indented marker");
    }

    #[test]
    fn non_block_quote_does_not_parse() {
        let input = "Not a block quote\n";
        let mut test_props = wrap_test_conundrum_content(input);
        assert!(BlockQuoteResult::parse_input_string(&mut test_props).is_err(), "plain text must not match");
    }
}
