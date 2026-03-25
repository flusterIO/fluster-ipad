use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use serde::Serialize;
use winnow::{
    ModalResult, Parser,
    ascii::{multispace0, space1, till_line_ending},
    combinator::{alt, delimited, opt, preceded, repeat},
    stream::Stream,
    token::{literal, take_till, take_while},
};

use crate::{
    lang::runtime::{
        state::parse_state::ParseState,
        traits::{conundrum_input::ConundrumInput, mdx_component_result::MdxComponentResult},
    },
    output::output_components::output_utils::{format_embedded_object_property, javascript_null_prop},
    parsers::parser_trait::ConundrumParser,
};

#[derive(Debug, Serialize)]
pub struct MarkdownHeadingResult {
    pub depth: usize,
    pub content: String,
    pub id: Option<String>,
}

impl MdxComponentResult for MarkdownHeadingResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        format!("<{} depth={} id={}>{}</{}>",
                AutoInsertedComponentName::AutoInsertedHeading,
                format_embedded_object_property(self.depth.to_string()),
                self.id.clone().map(|s| format!("\"{}\"", s)).unwrap_or(javascript_null_prop()),
                self.content,
                AutoInsertedComponentName::AutoInsertedHeading)
    }
}

impl ConundrumParser<MarkdownHeadingResult> for MarkdownHeadingResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ModalResult<MarkdownHeadingResult> {
        let start = input.input.checkpoint();
        let _ = opt(literal("\n")).parse_next(input).inspect_err(|_| {
                                                         input.input.reset(&start);
                                                     })?;
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

        Ok(MarkdownHeadingResult { depth: level.len(),
                                   content: content.clone(),
                                   id: id.map(|x| x.to_string()) })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
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
        assert!(res.content == "My heading", "Finds the proper heading content when no id is present.");
        // assert_eq!(result, 4);
    }

    #[test]
    fn parses_markdown_heading_with_id() {
        let test_content = "## My heading depth 2 {#myId}";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            MarkdownHeadingResult::parse_input_string(&mut test_data).expect("Parses markdown heading without failing");
        println!("Res: {:#?}", res);
        assert!(res.id.is_some_and(|id| id == "myId"), "Finds heading id when one is present.");
        assert!(res.depth == 2, "Finds the proper heading depth when no id is present..");
        assert!(res.content == "My heading depth 2", "Finds the proper heading content when no id is present.");
        // assert_eq!(result, 4);
    }
}
