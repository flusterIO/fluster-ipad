use fluster_core_utilities::core_types::component_constants::auto_inserted_component_name::AutoInsertedComponentName;
use winnow::{
    ModalResult, Parser, Result,
    ascii::{multispace0, space1, till_line_ending},
    combinator::{alt, delimited, preceded, repeat, trace},
    token::{literal, take_till, take_while},
};

use crate::{
    lang::runtime::traits::mdx_component_result::MdxComponentResult,
    output::output_components::output_utils::{format_embedded_object_property, javascript_null_prop},
    parsers::{parser_trait::ConundrumParser, utility_parsers::line_ending_or_eof::line_ending_or_end_of_file},
};

#[derive(Debug)]
pub struct MarkdownHeadingResult {
    pub depth: usize,
    pub content: String,
    pub id: Option<String>,
}

impl MdxComponentResult for MarkdownHeadingResult {
    fn to_mdx_component(&self, _: &mut crate::output::parsing_result::mdx_parsing_result::MdxParsingResult) -> String {
        format!("<{} depth={} id={}>{}</{}>",
                AutoInsertedComponentName::AutoInsertedHeading,
                format_embedded_object_property(self.depth.to_string()),
                self.id.clone().unwrap_or(javascript_null_prop()),
                self.content,
                AutoInsertedComponentName::AutoInsertedHeading)
    }
}

fn parse_id_block<'a>(input: &mut &'a str) -> Result<&'a str> {
    // Parses: {#some-id}
    delimited("{#", take_while(1.., |c: char| c.is_alphanumeric() || c == '-' || c == '_'), "}").parse_next(input)
}

impl ConundrumParser<MarkdownHeadingResult> for MarkdownHeadingResult {
    fn parse_input_string<'a>(input: &mut &'a str) -> ModalResult<MarkdownHeadingResult> {
        let level: Vec<char> = repeat(1..=6, '#').parse_next(input)?;

        // 2. Expect at least one space (mandatory in standard Markdown)
        let _ = space1.parse_next(input)?;

        // 3. Consume everything until the end of the line
        // We use terminated to ensure we consume the newline character if it exists
        let (content, id) =
            alt((// Case A: Heading has an ID
                 (take_till(1.., '{'), // Take text until the brace
                  preceded(multispace0,
                           delimited(literal("{#"),
                                     take_while(1.., |c: char| c.is_alphanumeric() || c == '-' || c == '_'),
                                     '}')))
                                           .map(|(c, id): (&'a str, &'a str)| (c.trim(), Some(id))),
                 // Case B: Normal heading
                 line_ending_or_end_of_file().map(|c: &'a str| (c.trim(), None)),
                 till_line_ending.map(|c: &'a str| (c.trim(), None)))).parse_next(input)?;

        Ok(MarkdownHeadingResult { depth: level.len(),
                                   content: content.trim().to_string(),
                                   id: id.map(|x| x.to_string()) })
    }

    fn matches_first_char(char: char) -> bool {
        char == '$'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_markdown_heading() {
        let mut test_content = "### My heading";
        let res = MarkdownHeadingResult::parse_input_string(&mut test_content).expect("Parses markdown heading without failing");
        assert!(res.id.is_none(), "Finds no heading id when none is present.");
        assert!(res.depth == 3, "Finds the proper heading depth when no id is present..");
        assert!(res.content == "My heading", "Finds the proper heading content when no id is present.");
        // assert_eq!(result, 4);
    }

    #[test]
    fn parses_markdown_heading_with_id() {
        let mut test_content = "## My heading depth 2 {#myId}";
        let res = MarkdownHeadingResult::parse_input_string(&mut test_content).expect("Parses markdown heading without failing");
        println!("Res: {:#?}", res);
        assert!(res.id.is_some_and(|id| id == "myId"), "Finds heading id when one is present.");
        assert!(res.depth == 2, "Finds the proper heading depth when no id is present..");
        assert!(res.content == "My heading depth 2", "Finds the proper heading content when no id is present.");
        // assert_eq!(result, 4);
    }
}
