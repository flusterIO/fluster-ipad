use std::{cell::RefCell, str::FromStr};

use serde::Serialize;
use typeshare::typeshare;
use winnow::{
    Parser, Stateful,
    ascii::alphanumeric1,
    combinator::{delimited, repeat},
    error::ErrMode,
    stream::{AsChar, Stream},
    token::{literal, take_until, take_while},
};

use crate::{
    lang::runtime::{
        parse_conundrum_string::parse_elements,
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            parse_state::ParseState,
        },
        traits::{
            conundrum_input::{ConundrumInput, get_conundrum_input},
            fluster_component_result::ConundrumComponentResult,
            mdx_component_result::MdxComponentResult,
        },
    },
    output::general::component_constants::component_names::EmbeddableComponentName,
    parsers::{
        as_char_extensions::is_space_or_newline,
        conundrum::logic::object::object::ConundrumObject,
        javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        parser_components::white_space_delimited::white_space_delimited,
        parser_trait::ConundrumParser,
        react::{
            components::COMPONENT_MAP, conundrum_component::ConundrumComponentType,
            parser_components::jsx_properties::any_jsx_property::any_jsx_property,
        },
    },
};

#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct ReactComponentWithChildrenResult {
    pub full_text: String,
    pub component: ConundrumComponentType,
}

impl ConundrumComponentResult for ReactComponentWithChildrenResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.component.to_conundrum_component(res)
    }
}

impl MdxComponentResult for ReactComponentWithChildrenResult {
    // PERFORMANCE: Consider moving the `get_component_map` regex query to a hashmap
    // or something in Rust equivalent to a Set for speedy lookup. This would
    // require that the component parser is working at basically 100%
    // reliability, which it currently won't be with this `>` limitation, but
    // after that is handled add a field to the conundrum state to
    // allow creating the list of included components here.
    fn to_mdx_component(&self, _: &mut ParseState) -> ConundrumModalResult<String> {
        Ok(self.full_text.clone())
    }
}

fn react_closing_tag_parser_by_name(component_name: &str) -> impl Fn(&mut ConundrumInput) -> ConundrumModalResult<()> {
    move |input| {
        let _ = delimited(literal("</"),
                          (take_while(0.., is_space_or_newline),
                           literal(component_name),
                           take_while(0.., is_space_or_newline)),
                          '>').parse_next(input)?;
        Ok(())
    }
}

fn parse_react_component_with_children(input: &mut ConundrumInput)
                                       -> ConundrumModalResult<ReactComponentWithChildrenResult> {
    let start = input.input.checkpoint();

    '<'.void().parse_next(input).inspect_err(|_| {
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
    let component_name = EmbeddableComponentName::from_str(component_name_string.as_str()).map_err(|e| {
                                                                                              input.input.reset(&start);
                                                                                              ErrMode::Backtrack(e)
                                                                                          })?;

    let props_kv_pairs: Vec<JavascriptObjectKeyValuePair> =
        repeat(0.., white_space_delimited(any_jsx_property)).parse_next(input).inspect_err(|_| {
                                                                                   input.input.reset(&start);
                                                                               })?;

    let props = ConundrumObject::from_kv_pair_vec(props_kv_pairs);

    '>'.parse_next(input).inspect_err(|_| {
                              input.input.reset(&start);
                          })?;

    let children_string = take_until(0.., "</").parse_next(input).inspect_err(|_| {
                                                                      input.input.reset(&start);
                                                                  })?;

    take_while(0.., AsChar::is_space).void().parse_next(input).inspect_err(|_| {
                                                                   input.input.reset(&start);
                                                               })?;

    react_closing_tag_parser_by_name(component_name_string.as_str()).parse_next(input)?;

    let state = input.state.borrow_mut();
    let mut new_input: Stateful<&str, RefCell<ParseState>> =
        get_conundrum_input(children_string, state.modifiers.clone());
    let children = parse_elements(&mut new_input)?;

    let component_getter_kv = COMPONENT_MAP.get(&component_name.to_component_id()).ok_or_else( || {
            let e = ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Failed to find component", format!("You provided a component name of {} but that component is not supported by Conundrum.", component_name.clone()).as_str()));
            ErrMode::Cut(e)
    })?;

    let getter = component_getter_kv.value();

    let component = getter(props, Some(children))?;

    Ok(ReactComponentWithChildrenResult { full_text: "".to_string(), // This field will be
                                          // replaced below anyways.
                                          component })
}

impl ConundrumParser<ReactComponentWithChildrenResult> for ReactComponentWithChildrenResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumModalResult<ReactComponentWithChildrenResult> {
        let (mut res, taken) = parse_react_component_with_children.with_taken().parse_next(input)?;
        res.full_text = taken.to_string();
        Ok(res)
    }

    fn matches_first_char(char: char) -> bool {
        char == '<'
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::testing::wrap_test_content::wrap_test_conundrum_content;

    use super::*;

    #[test]
    fn react_component_parses_component_outline() {
        let test_content = r#"<Card title="My card's title" desc="My card's description!" >
This is my children markdown test content 
</Card>"#;

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = ReactComponentWithChildrenResult::parse_input_string(&mut test_data).expect("Parses vald component successfully.");

        let mut state = test_data.state.borrow_mut();
        let mdx_component = res.to_mdx_component(&mut state).expect("Compiles to mdx without throwing an error.");
        assert!(mdx_component == test_content, "Returns the input component as the mdx component for an mdx input.");
        assert!(test_data.is_empty(), "Consumes the entire input string.");
        assert_snapshot!(mdx_component);
        let children = match res.component {
            ConundrumComponentType::Card(c) => c.children.render(&mut state).expect("Compiles children successfully"),
            _ => panic!("Found a component that's not the `Card` that was inserted."),
        };
        assert_snapshot!(children);
    }
}
