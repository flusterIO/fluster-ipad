use serde::Serialize;
use std::sync::Arc;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::{
            shared::traits::from_with_state::FromWithState, ui::ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::{
            compile_conundrum::compile_elements,
            parse_conundrum_string::parse_elements,
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::conundrum_input::{ArcState, ConundrumInput},
        },
    },
    output::output_components::output_utils::format_markdown_fragment_property,
    parsers::conundrum::logic::object::object::ConundrumObject,
};

/// The representation of 'children', usually other markdown content. Unlike
/// other UI frameworks like React, `children` does not necessarily refer only
/// to the components that go within the main `children` property, but rather
/// any Conundrum content that is rendered inside of any of it's slots. A
/// `title`, a `subtitle` or a `label` might all be represented as children
/// since they accept Conundrum content and are represented in the UI, while
/// something like a `columns` property obviously is not.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Default, Clone)]
pub struct Children(pub Vec<ParsedElement>);

impl Children {
    pub fn new_empty() -> Self {
        Children(Vec::new())
    }

    /// Outputs to the various conundrum outputs depending on the associated
    /// flags.
    pub fn render(&self, res: ArcState) -> ConundrumModalResult<String> {
        let thread_state = Arc::clone(&res);
        let children = self.0.clone();
        let handle = std::thread::spawn(move || compile_elements(&children, &thread_state));
        if let Ok(joined_thread) = handle.join() {
            joined_thread
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::MultiThreadingError))
        }
    }

    /// Render will not be called fo these elements. To maintain an accurate
    /// state that represents the _inserted_ note, not the _rendered_ note
    /// we need to pass the state here anyways.
    pub fn render_bypassed(&self, _: ArcState) {
        todo!()
    }

    /// Returns the string that can be inserted directly into the `={<>...</>}`
    /// syntax. I would recommend everyone use this syntax if the syntax
    /// highlighter didn't break, but we will be able to fix that in the
    /// coming months once the Conundrum LSP & syntax highlighter are
    /// online.
    pub fn to_jsx_fragment_string(&self, res: ArcState) -> ConundrumModalResult<String> {
        let res = self.render(res)?;
        Ok(format_markdown_fragment_property(res.as_str()))
    }

    /// Inserts the prop not as a Fragment, but as a string.
    pub fn to_jsx_prop_as_string(&self, prop_name: &str, res: ArcState) -> ConundrumModalResult<String> {
        let children_string = self.render(res)?;
        let x = serde_json::to_string(children_string.as_str()).map_err(|e| {
                    println!("Error: {:#?}", e);
                    ErrMode::Backtrack(ConundrumErrorVariant::FailToGenerateString)
                })?;
        // String is already quoted, no need to wrap it in quotes.
        Ok(format!("{}={}", prop_name, x))
    }

    pub fn to_jsx_prop(&self, prop_name: &str, res: ArcState) -> ConundrumModalResult<String> {
        let s = self.to_jsx_fragment_string(res)?;
        Ok(format!("{}={}", prop_name, s))
    }
}

impl FromWithState<&str> for Children {
    fn from_with_state(value: &str, state: ArcState) -> ConundrumModalResult<Self> {
        let mut input = ConundrumInput { input: value,
                                         state };
        let res = parse_elements(&mut input)?;
        Ok(Children(res))
    }
}

impl FromJsxPropsOptional for Children {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        let res = props.data.get(key);
        match res {
            Some(r) => match &r.value() {
                ParsedElement::Children(c) => Ok(c.clone()),
                _ => {
                    Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Mistaken identity", format!("Conundrum was looking for more Conundrum content at the `{}` key but found something else. Review the `Syntax??` docs for more assistance. The `Jsx??` docs may be of some help as well, but they're making their way over to the `Conundrum??` docs as the Conundrum transpiler takes over for mdx all together. The answer is in there somewhere...", key).as_str()))))
                }
            },
            None => {
                    Err(ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Lost & Not Found", format!("Conundrum was looking for more Conundrum content at the `{}` key but couldn't find anything.", key).as_str()))))
            }
        }
    }
}
