use std::cell::RefCell;

use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use serde::{Deserialize, Serialize};
use winnow::{
    ModalResult, Parser, Stateful,
    ascii::{multispace0, space1, till_line_ending},
    combinator::{alt, delimited, preceded, repeat},
    stream::Stream,
    token::{literal, take_till, take_while},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            state::parse_state::{ConundrumModifier, ParseState},
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
                state_modifier::ConundrumStateModifier,
            },
        },
    },
    output::output_components::output_utils::{format_embedded_object_property, javascript_null_prop},
    parsers::parser_trait::ConundrumParser,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize)]
pub struct MarkdownHeadingResult {
    pub depth: u16,
    pub children: Vec<ParsedElement>,
    pub id: Option<String>,
}

/// The same as the `MarkdownHeadingResult` struct but wth the children
/// rendered.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct MarkdownHeadingStringifiedResult {
    pub depth: u16,
    pub content: String,
    pub id: Option<String>,
}

impl MarkdownHeadingResult {
    pub fn to_stringified_result(&self, res: &mut ParseState) -> MarkdownHeadingStringifiedResult {
        let children_string = compile_elements(&self.children, res);
        MarkdownHeadingStringifiedResult { depth: self.depth.clone(),
                                           content: children_string,
                                           id: self.id.clone() }
    }
}

impl ConundrumStateModifier for MarkdownHeadingResult {
    fn set_state(&self, res: &mut ParseState) {
        let x = self.to_stringified_result(res);
        res.data.toc.push(x)
    }
}

impl PlainTextComponentResult for MarkdownHeadingResult {
    fn to_plain_text(&self, res: &mut ParseState) -> String {
        compile_elements(&self.children, res)
    }
}

impl ConundrumComponentResult for MarkdownHeadingResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        self.set_state(res);
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for MarkdownHeadingResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> String {
        let children_string = compile_elements(&self.children, res);

        format!("<{} depth={} id={}>{}</{}>",
                AutoInsertedComponentName::AutoInsertedHeading,
                format_embedded_object_property(self.depth.to_string()),
                self.id.clone().map(|s| format!("\"{}\"", s)).unwrap_or(javascript_null_prop()),
                children_string,
                AutoInsertedComponentName::AutoInsertedHeading)
    }
}

impl ConundrumParser<MarkdownHeadingResult> for MarkdownHeadingResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<MarkdownHeadingResult> {
        let start = input.input.checkpoint();
        let level: Vec<char> = repeat(1..=6, '#').parse_next(input).inspect_err(|_| {
                                                                        input.input.reset(&start);
                                                                    })?;

        // 2. Expect at least one space (mandatory in standard Markdown)
        let _ = space1.parse_next(input).inspect_err(|_| {
                                             input.input.reset(&start);
                                         })?;

        // 3. Consume everything until the end of the line
        // We use terminated to ensure we consume the newline character if it exists
        let (content, id) =
            alt((// Case A: Heading has an ID
                 (take_till(1.., |c: char| c == '{' || c == '\n'), // Take text until the brace
                  preceded(multispace0,
                           delimited(literal("{#"),
                                     take_while(1.., |c: char| c.is_alphanumeric() || c == '-' || c == '_'),
                                     '}')))
                                           // Case B: Normal heading (turn this back on once this parser can be typed
                                           // again)
                                           .map(|(c, id): (&'a str, &'a str)| (c.trim(), Some(id))),
                 till_line_ending.map(|c: &'a str| (c.trim(), None)))).parse_next(input)
                                                                      .inspect_err(|_| {
                                                                          input.input.reset(&start);
                                                                      })?;

        let content = content.trim().to_string();

        let state = input.state.borrow();

        let mut new_input: Stateful<&str, RefCell<ParseState>> =
            get_conundrum_input(content.as_str(), state.modifiers.clone());
        let children = parse_elements(&mut new_input)?;

        Ok(MarkdownHeadingResult { depth: level.len() as u16,
                                   children,
                                   id: id.map(|x| x.to_string()) })
    }

    fn matches_first_char(char: char) -> bool {
        char == '#'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_markdown_heading() {
        let test_content = "### My heading";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            MarkdownHeadingResult::parse_input_string(&mut test_data).expect("Parses markdown heading without failing");
        assert!(res.id.is_none(), "Finds no heading id when none is present.");
        assert!(res.depth == 3, "Finds the proper heading depth when no id is present..");
        let mut state = test_data.state.borrow_mut();
        let children_string = compile_elements(&res.children, &mut state);

        // TODO: Add this test back in both of these tests for the renderd
        // children instead of the stringified content.
        assert!(children_string == "My heading", "Finds the proper heading content when no id is present.");
    }

    #[test]
    fn parses_markdown_heading_with_id() {
        let test_content = "## My heading depth 2 {#myId}";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            MarkdownHeadingResult::parse_input_string(&mut test_data).expect("Parses markdown heading without failing");
        assert!(res.id.is_some_and(|id| id == "myId"), "Finds heading id when one is present.");
        assert!(res.depth == 2, "Finds the proper heading depth when no id is present..");

        let mut state = test_data.state.borrow_mut();
        let children_string = compile_elements(&res.children, &mut state);
        assert!(children_string == "My heading depth 2", "Finds the proper heading content when no id is present.");
    }
}
