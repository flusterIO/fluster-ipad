use serde::Serialize;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent,
            ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
            ui_types::emphasis::Emphasis,
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, object::object::ConundrumObject, string::conundrum_string::ConundrumString,
    },
};

/// ## Example Usage
///
/// ```mdx
/// <EqRef id="my-equation" anchor>
/// $$
/// e=mc^2
/// $$
/// </EqRef>
///
/// yada, yada, yada, My equation <EqRef id="my-equation" /> is awesome...
/// ```
///
/// The above will result in something like:
///
/// _yada, yada, yada, My equation 1 is awesome..._
///
/// Or whatever index is represented by that equation, allowing you to reference
/// equations by index without worrying about the structure of your note
/// changing.
///
/// #### Indices
///
/// The equation reference component uses 1 based indices for plain text and
/// markdown since they are likely going to be displayed directly, and zero
/// based indices for everything where the output will be handled by a developer
/// before being displayed (React, HTML, Swift, etc..) because 1 based indices
/// suck.
#[typeshare::typeshare]
#[derive(Serialize, uniffi::Record, Debug, Clone)]
pub struct EquationReference {
    /// The `id` field is a string that can be referenced later to use this
    /// equation's number.
    pub id: ConundrumString,
    /// This has no effect when `anchor=true`, but will have make the index
    /// subscript when `anchor` is not set.
    pub subscript: Option<ConundrumBoolean>,
    /// This has no effect when `anchor=true`, but will have make the index
    /// superscript when `anchor` is not set.
    pub superscript: Option<ConundrumBoolean>,
    /// Style the output index according to one of the available Emphasis styles
    /// if you like.
    pub emphasis: Option<Emphasis>,
}

impl EquationReference {
    pub fn get_equation_index(&self, res: ArcState) -> ConundrumModalResult<u32> {
        let state = res.read_arc();
        state.data.eq_ref_map.get(&self.id.0).cloned().ok_or_else(|| {
              let e = ConundrumErrorVariant::UserFacingGeneralParserError(ConundrumError::from_msg_and_details("Could not find equation", format!("Conundrum was trying to find an equation with the id of '{}' and couldn't find one in this document.", self.id.0.clone()).as_str()));
              ErrMode::Cut(e)
          })
    }
}

impl MarkdownComponentResult for EquationReference {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.get_equation_index(res).map(|s| (s + 1).to_string())
    }
}

impl PlainTextComponentResult for EquationReference {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.get_equation_index(res).map(|s| (s + 1).to_string())
    }
}

impl JsxComponentResult for EquationReference {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let x = self.get_equation_index(res)?;
        let id_string = self.id.to_jsx_prop_as_string("id").map_err(|e| {
            ErrMode::Backtrack(
                ConundrumErrorVariant::UserFacingGeneralParserError(
                    ConundrumError::from_msg_and_details("Serialization error", "Conundrum could not successfully seralize an id provided to the equation reference component")
                )
            )
        })?;
        let mut props = vec![format!("idx={{{}}}", x), id_string];
        if let Some(subscript) = self.subscript {
            props.push(subscript.to_jsx_prop("subscript"))
        }
        if let Some(superscript) = self.superscript {
            props.push(superscript.to_jsx_prop("superscript")) // Sent to js as superscript
            // instead of 'super' to avoid
            // keyword clashes.
        }
        if let Some(emphasis) = &self.emphasis {
            props.push(emphasis.to_string());
        }
        Ok(format!("<{} {} />", EmbeddableComponentName::EqRef, props.join(" ")))
    }
}

impl MdxComponentResult for EquationReference {
    fn to_mdx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumComponentResult for EquationReference {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.is_plain_text() {
            drop(state);
            self.to_plain_text(res)
        } else if state.is_markdown_or_search_or_ai() {
            drop(state);
            self.to_markdown(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for EquationReference {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::EqRef)
    }

    fn from_props(props: ConundrumObject, _: Option<Vec<ParsedElement>>, _: ArcState) -> ConundrumModalResult<Self>
        where Self: Sized {
        let id = ConundrumString::from_jsx_props(&props, "id")?;
        let subscript = ConundrumBoolean::from_jsx_props(&props, "sub").ok();
        let superscript = ConundrumBoolean::from_jsx_props(&props, "super").ok();
        let emphasis = Emphasis::from_jsx_props(&props, "").ok();
        Ok(EquationReference { id,
                               subscript,
                               emphasis,
                               superscript })
    }
}
