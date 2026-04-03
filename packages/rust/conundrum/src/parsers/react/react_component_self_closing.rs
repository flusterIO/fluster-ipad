use std::str::FromStr;

use serde::Serialize;
use typeshare::typeshare;
use winnow::{
    ModalResult, Parser,
    ascii::alphanumeric1,
    combinator::repeat,
    error::ErrMode,
    stream::{AsChar, Stream},
    token::{literal, take_while},
};

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::{ConundrumErrorVariant, ConundrumResult},
            parse_state::{ConundrumModifier, ParseState},
        },
        traits::{
            ai_input_component_result::AIInputComponentResult, conundrum_input::ConundrumInput,
            fluster_component_result::ConundrumComponentResult, markdown_component_result::MarkdownComponentResult,
            mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
    output::general::component_constants::component_names::EmbeddableComponentName,
    parsers::{
        conundrum::logic::object::object::ConundrumObject,
        javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        parser_components::white_space_delimited::white_space_delimited,
        parser_trait::ConundrumParser,
        react::{
            components::{COMPONENT_MAP, ConundrumComponentType},
            parser_components::jsx_properties::any_jsx_property::any_jsx_property,
            react_component::ReactComponent,
        },
    },
};

#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct ReactComponentSelfClosingResult {
    pub full_text: String,
    pub component_name: EmbeddableComponentName,
    pub props: ConundrumObject,
}

impl ReactComponent for ReactComponentSelfClosingResult {
    fn get_conundrum_from_react(&self) -> ConundrumResult<ConundrumComponentType> {
        let id = self.component_name.to_component_id();
        if let Some(component) = COMPONENT_MAP.get(&id) {
            let getter = component.value();
            let res = getter(self.props.clone(), None);
            res
        } else {
            Err(ConundrumErrorVariant::FailToFindComponent(id.to_string()))
        }
    }
}

impl AIInputComponentResult for ReactComponentSelfClosingResult {
    fn to_ai_input(&self, _: &mut ParseState) -> String {
        String::from("")
    }
}

impl MarkdownComponentResult for ReactComponentSelfClosingResult {
    fn to_markdown(&self, _: &mut ParseState) -> String {
        String::from("")
    }
}

impl PlainTextComponentResult for ReactComponentSelfClosingResult {
    // TODO: Parse specific Fragment based properties as markdown and figure out a
    // way to format everything nicely here.
    fn to_plain_text(&self, _: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        String::from("")
    }
}

impl MdxComponentResult for ReactComponentSelfClosingResult {
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        self.full_text.clone()
    }
}

impl ConundrumComponentResult for ReactComponentSelfClosingResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        if res.contains_modifier(&ConundrumModifier::ForAIInput) {
            self.to_ai_input(res)
        } else if res.contains_one_of_modifiers(vec![ConundrumModifier::PreferMarkdownSyntax,
                                                     ConundrumModifier::PreferInlineMarkdownSyntax])
        {
            self.to_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

fn parse_self_closing_react_component(input: &mut ConundrumInput) -> ModalResult<ReactComponentSelfClosingResult> {
    let start = input.input.checkpoint();

    let _ = '<'.parse_next(input).inspect_err(|_| {
                                      input.input.reset(&start);
                                  })?;

    let component_leading_char = take_while(1, AsChar::is_alpha).verify(|c: &str| {
                                                                    let s = c.to_string();
                                                                    s == s.to_uppercase()
                                                                })
                                                                .parse_next(input)
                                                                .inspect_err(|_| {
                                                                    input.input.reset(&start);
                                                                })?;

    let rest_component_name: Vec<&str> = repeat(1.., alphanumeric1).parse_next(input).inspect_err(|_| {
                                                                                          input.input.reset(&start);
                                                                                      })?;

    let component_name_string = format!("{}{}", component_leading_char, rest_component_name.join(""));
    // RESUME: Pick back up here by handling the passing of th unique errors back to
    // the user. Currently only the top leve, 'woops we fucked up' error is
    // being returned.
    // let component_name =
    // EmbeddableComponentName::from_str(component_name_string.as_str()).map_err(|e|
    // {
    // input.input.reset(&start);
    // let x =
    // ErrMode::Backtrack(e);
    // x.convert()
    // })?;
    let props: Vec<JavascriptObjectKeyValuePair> =
        repeat(0.., white_space_delimited(any_jsx_property)).parse_next(input).inspect_err(|_| {
                                                                                   input.input.reset(&start);
                                                                               })?;

    let _ = literal("/>").parse_next(input).inspect_err(|_| {
                                                input.input.reset(&start);
                                            })?;
    Ok(ReactComponentSelfClosingResult { full_text: "".to_string(),
                                         component_name: EmbeddableComponentName::Hl,
                                         props: ConundrumObject::from_kv_pair_vec(props) })
}

impl ConundrumParser<ReactComponentSelfClosingResult> for ReactComponentSelfClosingResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ModalResult<ReactComponentSelfClosingResult> {
        let (mut res, taken) = parse_self_closing_react_component.with_taken().parse_next(input)?;
        res.full_text = taken.to_string();
        Ok(res)
    }

    fn matches_first_char(char: char) -> bool {
        char == '<'
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn parses_self_closing_react_component() {
        let test_content = r#"<Card myBool myObject={{}} myString="Here is a string" />"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            ReactComponentSelfClosingResult::parse_input_string(&mut test_data).expect("Parses valid self closing react component without throwing an error");
        assert!(test_data.input.is_empty(), "Consumes the entire component string.");
        assert!(res.full_text == test_content, "Returns the complete test content");
        let mut state = test_data.state.borrow_mut();
        let mdx_component = res.to_mdx_component(&mut state);
        assert!(mdx_component == test_content, "Returns the input component as an mdx input");
        assert!(res.component_name == EmbeddableComponentName::Card, "Returns the proper component  name");
        // assert_eq!(result, 4);
    }

    #[test]
    fn parses_self_closing_react_component_that_contains_closing_tag_in_str() {
        let test_content = r#"<Container myBool myObject={{}} myString="Here is /> a string" />"#;
        let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            ReactComponentSelfClosingResult::parse_input_string(&mut test_data).expect("Parses valid self closing react component without throwing an error.");
        assert!(test_data.input.is_empty(), "Consumes the entire component string.");
        assert!(res.full_text == test_content, "Returns the complete test content");
        let mut state = test_data.state.borrow_mut();
        let mdx_component = res.to_mdx_component(&mut state);
        assert!(mdx_component == test_content, "Returns the input component as an mdx input");
        assert!(res.component_name == EmbeddableComponentName::UtlityContainer, "Returns the proper component  name");
        // assert_eq!(result, 4);
    }
}
