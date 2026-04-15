use serde::Serialize;
use typeshare::typeshare;
use winnow::{
    Parser,
    ascii::alphanumeric1,
    combinator::{delimited, repeat},
    stream::{AsChar, Stream},
    token::{literal, take_while},
};

use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ConundrumInput},
    output::general::component_constants::any_component_id::AnyComponentName,
    parsers::{
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
pub struct ReactComponentSelfClosingResult {
    pub full_text: String,
    pub component: ConundrumComponentType,
}

fn parse_self_closing_react_component(input: &mut ConundrumInput)
                                      -> ConundrumModalResult<ReactComponentSelfClosingResult> {
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
    let component_name = AnyComponentName::get_component_name(component_name_string.as_str()).inspect_err(|e| {
                                                                                                 input.input
                                                                                                      .reset(&start);
                                                                                             })?;

    let props_kv: Vec<JavascriptObjectKeyValuePair> = delimited(space_or_newline0,
                                                                repeat(0.., white_space_delimited(any_jsx_property)),
                                                                space_or_newline0).parse_next(input)
                                                                                  .inspect_err(|_| {
                                                                                      input.input.reset(&start);
                                                                                  })?;

    let props = ConundrumObject::from_kv_pair_vec(props_kv);

    let component = COMPONENT_MAP.get_by_component_name(&component_name, props, None)?;

    let _ = literal("/>").parse_next(input).inspect_err(|_| {
                                                input.input.reset(&start);
                                            })?;

    let mut state = input.state.borrow_mut();

    state.data.append_embeddable_component(&component_name);
    Ok(ReactComponentSelfClosingResult { full_text: "".to_string(), // Get's replaced anyways,
                                         component })
}

impl ConundrumParser<ReactComponentSelfClosingResult> for ReactComponentSelfClosingResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumModalResult<ReactComponentSelfClosingResult> {
        let (mut res, taken) = parse_self_closing_react_component.with_taken().parse_next(input)?;
        res.full_text = taken.to_string();
        Ok(res)
    }

    fn matches_first_char(char: char) -> bool {
        char == '<'
    }
}

// TODO: Turn these tests back on when we've enabled a self-closing react
// component on the Rust side.
#[cfg(test)]
mod tests {
    use crate::lang::runtime::{
        run_conundrum::{ParseConundrumOptions, run_conundrum},
        state::ui_params::UIParams,
    };

    use super::*;

    #[tokio::test]
    async fn parses_self_closing_react_component() {
        let test_content = r#"
$$ {#myId}
e=mc^2
$$
My equation <EqRef id="myId" super />."#;
        // let mut test_data = wrap_test_conundrum_content(test_content);
        let res =
            run_conundrum(ParseConundrumOptions {
            content: test_content.to_string(),
            modifiers: vec![],
            note_id: None,
            ui_params: UIParams::default(),
            hide_components: vec![]
            }).await.expect("Parses valid self closing react component without throwing
an error");

        assert!(res.eq_ref_map.get("myId").is_some_and(|n| *n == 0),
                "Returns the proper index for the EqRef component.");

        insta::assert_snapshot!(res.content)
    }
}
