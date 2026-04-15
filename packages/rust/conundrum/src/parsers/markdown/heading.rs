use serde::{Deserialize, Serialize};
use winnow::{
    Parser,
    ascii::{multispace0, space1, till_line_ending},
    combinator::{alt, delimited, opt, preceded, repeat},
    error::ErrMode,
    stream::{AsChar, Stream},
    token::{literal, take_till, take_while},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                conundrum_input::{ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::{
        general::component_constants::auto_inserted_component_name::AutoInsertedComponentName,
        output_components::output_utils::{format_embedded_object_property, format_markdown_fragment_property},
    },
    parsers::{conundrum::logic::string::conundrum_string::ConundrumString, parser_trait::ConundrumParser},
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct MarkdownHeadingResult {
    pub depth: u16,
    /// The indentation level to be used when displaying this in an inline table
    /// of contents, not necessarily the level of the heading since some heading
    /// lists will skip a depth.
    pub tab_depth: u16,
    pub children: Vec<ParsedElement>,
    pub subtitle: Option<Vec<ParsedElement>>,
    pub id: ConundrumString,
}

/// The same as the `MarkdownHeadingResult` struct but wth the children
/// rendered.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, uniffi::Record)]
pub struct MarkdownHeadingStringifiedResult {
    pub depth: u16,
    pub tab_depth: u16,
    pub content: String,
    pub id: String,
}

impl MarkdownHeadingResult {
    pub fn to_stringified_result(&self,
                                 res: &mut ParseState)
                                 -> ConundrumModalResult<MarkdownHeadingStringifiedResult> {
        let children_string = compile_elements(&self.children, res)?;
        Ok(MarkdownHeadingStringifiedResult { depth: self.depth,
                                              tab_depth: self.tab_depth,
                                              content: children_string,
                                              id: self.id.0.clone() })
    }
}

impl InlineMarkdownComponentResult for MarkdownHeadingResult {
    fn to_inline_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        compile_elements(&self.children, res)
    }
}

impl MarkdownComponentResult for MarkdownHeadingResult {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let mut s = String::from("");
        for _ in 1..self.depth {
            s += "#"
        }
        let children = compile_elements(&self.children, res)?;
        Ok(format!("{} {}", s, children))
    }
}

impl MarkdownHeadingResult {
    fn set_state(res: &mut ParseState, heading: &mut MarkdownHeadingResult) {
        if let Ok(mut x) =
            heading.to_stringified_result(res).inspect_err(|e| eprintln!("Error serializing heading: {:#?}", e))
        {
            if heading.depth > res.last_heading_depth {
                res.last_heading_tab_depth += 1;
            } else if heading.depth < res.last_heading_depth && res.last_heading_tab_depth > 0 {
                res.last_heading_tab_depth -= 1;
            }
            res.last_heading_depth = x.depth;
            x.tab_depth = res.last_heading_tab_depth;
            res.data.toc.push(x.clone());
        }
    }
}

impl PlainTextComponentResult for MarkdownHeadingResult {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        compile_elements(&self.children, res)
    }
}

impl ConundrumComponentResult for MarkdownHeadingResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else if res.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            self.to_inline_markdown(res)
        } else if res.contains_modifier(&ConundrumModifier::PreferMarkdownSyntax) {
            self.to_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for MarkdownHeadingResult {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let children_string = compile_elements(&self.children, res)?;
        let subtitle_string = match &self.subtitle {
            Some(s) => {
                let x = compile_elements(s, res)?;
                let y = x.as_str();
                format_markdown_fragment_property(y)
            }
            None => "{null}".to_string(),
        };

        Ok(format!("<{} depth={} id={} subtitle={} >{}</{}>",
                   AutoInsertedComponentName::AutoInsertedHeading,
                   format_embedded_object_property(self.depth.to_string()),
                   self.id.to_quoted_string().map_err(ErrMode::Backtrack)?,
                   subtitle_string,
                   children_string,
                   AutoInsertedComponentName::AutoInsertedHeading))
    }
}

pub fn heading_subtitle_line(input: &mut ConundrumInput) -> ConundrumModalResult<Vec<ParsedElement>> {
    let start = input.input.checkpoint();
    '\n'.parse_next(input).inspect_err(|_| {
                               input.input.reset(&start);
                           })?;

    '>'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;
    take_while(1.., AsChar::is_space).void().parse_next(input).inspect_err(|_| {
                                                                   input.input.reset(&start);
                                                               })?;
    let content = till_line_ending.parse_next(input).inspect_err(|_| {
                                                         input.input.reset(&start);
                                                     })?;

    let state = input.state.borrow();

    let mut new_input = get_conundrum_input(content, state.modifiers.clone(), state.ui_params.clone());
    let children = parse_elements(&mut new_input)?;
    Ok(children)
}

impl ConundrumParser<MarkdownHeadingResult> for MarkdownHeadingResult {
    fn parse_input_string<'a>(input: &mut ConundrumInput<'a>) -> ConundrumModalResult<MarkdownHeadingResult> {
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

        let content_string = content.trim().to_string();

        let subtitle = opt(heading_subtitle_line).parse_next(input).inspect_err(|_| {
                                                                        input.input.reset(&start);
                                                                    })?;

        let mut state = input.state.borrow_mut();

        let mut new_input =
            get_conundrum_input(content_string.as_str(), state.modifiers.clone(), state.ui_params.clone());
        let children = parse_elements(&mut new_input)?;

        let heading = MarkdownHeadingResult { depth: level.len() as u16,
                                              children,
                                              subtitle,
                                              tab_depth: state.last_heading_tab_depth,
                                              id: id.map(|x| ConundrumString(x.to_string()))
                                                    .unwrap_or_else(|| ConundrumString(state.slugger.slug(content))) };

        MarkdownHeadingResult::set_state(&mut state, &mut heading.clone());
        Ok(heading)
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
        let test_content = "### My heading with $\\delta$";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            MarkdownHeadingResult::parse_input_string(&mut test_data).expect("Parses markdown heading without failing");
        // assert!(res.id.is_none(), "Finds no heading id when none is present.");
        assert!(res.depth == 3, "Finds the proper heading depth when no id is present..");
        let mut state = test_data.state.borrow_mut();

        let children_string = compile_elements(&res.children, &mut state).expect("Compiles to valid mdx");
        // TODO: Add this test back in both of these tests for the renderd
        // children instead of the stringified content.
        insta::assert_snapshot!(children_string);
        let title_slug = state.slugger.slug("My heading with $\\delta$");

        assert!(title_slug != res.id.0,
                "Increments id, or at least they don't match... this is a super unreliable test but it might catch some stray errors.");
    }

    #[test]
    fn parses_markdown_heading_with_id() {
        let test_content = "## My heading depth 2 {#myId}";
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            MarkdownHeadingResult::parse_input_string(&mut test_data).expect("Parses markdown heading without failing");
        assert!(res.id == "myId", "Finds heading id when one is present.");
        assert!(res.depth == 2, "Finds the proper heading depth when no id is present..");

        let mut state = test_data.state.borrow_mut();
        let children_string = compile_elements(&res.children, &mut state).expect("Compiles to valid mdx");
        assert!(children_string == "My heading depth 2", "Finds the proper heading content when no id is present.");
    }

    #[test]
    fn fails_with_line_break_between_sub_title() {
        let test_content = r#"## My heading depth 2 {#myId}

> My note has a subtitle!"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            MarkdownHeadingResult::parse_input_string(&mut test_data).expect("Parses markdown heading without failing");
        assert!(res.id == "myId", "Finds heading id when one is present.");
        assert!(res.depth == 2, "Finds the proper heading depth when no id is present..");

        let mut state = test_data.state.borrow_mut();
        let children_string = compile_elements(&res.children, &mut state).expect("Compiles to valid mdx");
        assert!(children_string == "My heading depth 2", "Finds the proper heading content when no id is present.");
        assert!(res.subtitle.is_none(), "Finds no subtitle when there is a line break.")
    }

    #[test]
    fn parses_markdown_heading_with_subtitle() {
        let test_content = r#"## My heading depth 2 {#myId}
> My note has a subtitle!"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            MarkdownHeadingResult::parse_input_string(&mut test_data).expect("Parses markdown heading without failing");
        assert!(res.id == "myId", "Finds heading id when one is present.");
        assert!(res.depth == 2, "Finds the proper heading depth when no id is present..");

        let mut state = test_data.state.borrow_mut();
        let children_string = compile_elements(&res.children, &mut state).expect("Compiles to valid mdx");
        assert!(children_string == "My heading depth 2", "Finds the proper heading content when no id is present.");

        let subtitle = res.subtitle.expect("Should have a subtitle.");

        let subtitle_string = compile_elements(&subtitle, &mut state).expect("Compiles subtitle successfully.");
        assert!(subtitle_string == "My note has a subtitle!", "Finds the note's subtitle.")
    }
}
