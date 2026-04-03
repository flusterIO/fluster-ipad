use std::{cell::RefCell, ops::Deref, str::FromStr};

use serde::Serialize;
use typeshare::typeshare;
use winnow::{
    ModalResult, Parser, Stateful,
    ascii::alphanumeric1,
    combinator::{delimited, repeat},
    stream::{AsChar, Stream},
    token::{literal, take_until, take_while},
};

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error::{ConundrumErrorVariant, ConundrumResult},
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                ai_input_component_result::AIInputComponentResult,
                conundrum_input::{ConundrumInput, get_conundrum_input},
                fluster_component_result::ConundrumComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
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
            components::{COMPONENT_MAP, ConundrumComponentType},
            parser_components::jsx_properties::any_jsx_property::any_jsx_property,
            react_component::ReactComponent,
        },
    },
};

#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct ReactComponentWithChildrenResult {
    pub full_text: String,
    pub component_name: EmbeddableComponentName,
    pub children: Vec<ParsedElement>,
    pub props: ConundrumObject,
}

impl ReactComponentWithChildrenResult {
    pub fn to_component(&self) -> ParsedElement {
        let x = self.deref();
        ParsedElement::ReactComponentWithChildren(x.clone())
    }
}

impl ReactComponent for ReactComponentWithChildrenResult {
    fn get_conundrum_from_react(&self) -> ConundrumResult<ConundrumComponentType> {
        let id = &self.component_name.to_component_id();
        if let Some(component) = COMPONENT_MAP.get(id) {
            let getter = component.value();
            getter(self.props.clone(), Some(self.children.clone()))
        } else {
            Err(ConundrumErrorVariant::FailToFindComponent(id.to_string()))
        }
    }
}

impl InlineMarkdownComponentResult for ReactComponentWithChildrenResult {
    fn to_inline_markdown(&self, res: &mut ParseState) -> String {
        compile_elements(&self.children, res)
    }
}

impl AIInputComponentResult for ReactComponentWithChildrenResult {
    fn to_ai_input(&self, res: &mut ParseState) -> String {
        compile_elements(&self.children, res)
    }
}

impl MarkdownComponentResult for ReactComponentWithChildrenResult {
    fn to_markdown(&self, res: &mut ParseState) -> String {
        compile_elements(&self.children, res)
    }
}

impl PlainTextComponentResult for ReactComponentWithChildrenResult {
    // TODO: Parse specific Fragment based properties as markdown and figure out a
    // way to format everything nicely here.
    fn to_plain_text(&self, res: &mut crate::lang::runtime::state::parse_state::ParseState) -> String {
        compile_elements(&self.children, res)
    }
}

impl ConundrumComponentResult for ReactComponentWithChildrenResult {
    fn to_conundrum_component(&self, res: &mut ParseState) -> String {
        // if let Some(conundrum_component) = COMPONENT_MAP.get() {}
        if res.contains_modifier(&ConundrumModifier::ForAIInput) {
            self.to_ai_input(res)
        } else if res.contains_modifier(&ConundrumModifier::PreferMarkdownSyntax) {
            self.to_markdown(res)
        } else if res.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            self.to_inline_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl MdxComponentResult for ReactComponentWithChildrenResult {
    // PERFORMANCE: Consider moving the `get_component_map` regex query to a hashmap
    // or something in Rust equivalent to a Set for speedy lookup. This would
    // require that the component parser is working at basically 100%
    // reliability, which it currently won't be with this `>` limitation, but
    // after that is handled add a field to the conundrum state to
    // allow creating the list of included components here.
    fn to_mdx_component(&self, _: &mut ParseState) -> String {
        self.full_text.clone()
    }
}

fn react_closing_tag_parser_by_name(component_name: &str) -> impl Fn(&mut ConundrumInput) -> ModalResult<()> {
    move |input| {
        let _ = delimited(literal("</"),
                          (take_while(0.., is_space_or_newline),
                           literal(component_name),
                           take_while(0.., is_space_or_newline)),
                          '>').parse_next(input)?;
        Ok(())
    }
}

fn parse_react_component_with_children(input: &mut ConundrumInput) -> ModalResult<ReactComponentWithChildrenResult> {
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
    let component_name = EmbeddableComponentName::from_str(component_name_string.as_str()).inspect_err(|_| {
                                                                                              input.input.reset(&start);
                                                                                          })?;

    let props: Vec<JavascriptObjectKeyValuePair> =
        repeat(0.., white_space_delimited(any_jsx_property)).parse_next(input).inspect_err(|_| {
                                                                                   input.input.reset(&start);
                                                                               })?;

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

    Ok(ReactComponentWithChildrenResult { full_text: "".to_string(), // This field will be
                                          // replaced below anyways.
                                          component_name,
                                          children,
                                          props: ConundrumObject::from_kv_pair_vec(props) })
}

impl ConundrumParser<ReactComponentWithChildrenResult> for ReactComponentWithChildrenResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ModalResult<ReactComponentWithChildrenResult> {
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
        let test_content = r#"<MyComponent myBool myObject={{}} myString="This will cause > chaos" >
This is my children markdown test content 
</MyComponent>"#;

        let mut test_data = wrap_test_conundrum_content(test_content);

        let res = ReactComponentWithChildrenResult::parse_input_string(&mut test_data).expect("Parses vald component successfully.");

        assert!(res.component_name == "MyComponent", "Finds the proper component name");
        let mut state = test_data.state.borrow_mut();
        let mdx_component = res.to_mdx_component(&mut state);
        assert!(mdx_component == test_content, "Returns the input component as the mdx component for an mdx input.");
        assert!(test_data.is_empty(), "Consumes the entire input string.");
        assert_snapshot!(mdx_component);
        let children = compile_elements(&res.children, &mut state);
        assert_snapshot!(children);
    }
}
