use std::sync::Arc;

use serde::Serialize;
use typeshare::typeshare;
use winnow::{
    Parser,
    ascii::alphanumeric1,
    combinator::{delimited, repeat},
    stream::{AsChar, Stream},
    token::{literal, take_until, take_while},
};

use crate::{
    lang::runtime::{
        parse_conundrum_string::parse_elements,
        state::conundrum_error_variant::ConundrumModalResult,
        traits::{
            conundrum_input::{ArcState, ConundrumInput},
            fluster_component_result::ConundrumComponentResult,
        },
    },
    output::general::component_constants::any_component_id::AnyComponentName,
    parsers::{
        as_char_extensions::is_space_or_newline,
        conundrum::logic::object::object::ConundrumObject,
        javascript::object::javascript_key_value_pair::JavascriptObjectKeyValuePair,
        parser_components::white_space_delimited::white_space_delimited,
        parser_trait::ConundrumParser,
        parsers_shared::space_or_new_line::space_or_newline0,
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
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.component.to_conundrum_component(res)
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
    let component_name = AnyComponentName::get_component_name(component_name_string.as_str()).inspect_err(|_| {
                                                                                                 input.input
                                                                                                      .reset(&start);
                                                                                             })?;

    let props_kv_pairs: Vec<JavascriptObjectKeyValuePair> = delimited(space_or_newline0,
                                                                      repeat(0..,
                                                                             white_space_delimited(any_jsx_property)),
                                                                      space_or_newline0).parse_next(input)
                                                                                        .inspect_err(|_| {
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
    //
    let rc = Arc::clone(&input.state);

    let mut new_input = ConundrumInput { input: children_string,
                                         state: rc };
    let children = parse_elements(&mut new_input)?;

    let cloned_state = Arc::clone(&input.state);

    let component = COMPONENT_MAP.get_by_component_name(&component_name, props, Some(children), cloned_state)?;

    let mut state = input.state.write();
    state.data.append_embeddable_component(&component_name.to_component_key());
    drop(state);

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

        let mdx_component = res.component
                               .to_conundrum_component(Arc::clone(&test_data.state))
                               .expect("Compiles to mdx without throwing an error.");
        assert!(test_data.is_empty(), "Consumes the entire input string.");
        assert_snapshot!(mdx_component);
        let children = match res.component {
            ConundrumComponentType::Card(c) => {
                c.children.render(Arc::clone(&test_data.state)).expect("Compiles children successfully")
            }
            _ => panic!("Found a component that's not the `Card` that was inserted."),
        };
        assert_snapshot!(children);
    }
}
