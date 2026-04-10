use serde::Serialize;

use crate::{
    lang::{
        lib::ui::{
            components::component_trait::ConundrumComponent, shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional, ui_types::children::Children,
        },
        runtime::traits::{
            fluster_component_result::ConundrumComponentResult, jsx_component_result::JsxComponentResult,
            markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
            plain_text_component_result::PlainTextComponentResult,
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_ids::EmbeddableComponentId,
        component_names::EmbeddableComponentName,
    },
};

#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct ResponsiveGrid {
    pub sizable: Option<SizablePropsGroup>,
    pub children: Children,
}

impl MarkdownComponentResult for ResponsiveGrid {
    fn to_markdown(&self,
                   res: &mut crate::lang::runtime::state::parse_state::ParseState)
                   -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl PlainTextComponentResult for ResponsiveGrid {
    fn to_plain_text(&self,
                     res: &mut crate::lang::runtime::state::parse_state::ParseState)
                     -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl JsxComponentResult for ResponsiveGrid {
    fn to_jsx_component(&self,
                        res: &mut crate::lang::runtime::state::parse_state::ParseState)
                        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let props: Vec<String> = vec![];
        Ok(format!(
                   "<{} {}>
{}
</{}>",
                   EmbeddableComponentName::Grid,
                   props.join(" "),
                   self.children.render(res)?,
                   EmbeddableComponentName::Grid
        ))
    }
}

impl MdxComponentResult for ResponsiveGrid {
    fn to_mdx_component(&self,
                        res: &mut crate::lang::runtime::state::parse_state::ParseState)
                        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumComponentResult for ResponsiveGrid {
    fn to_conundrum_component(&self,
                              res: &mut crate::lang::runtime::state::parse_state::ParseState)
                              -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String>
    {
        if res.is_markdown_or_search_or_ai() {
            self.to_markdown(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for ResponsiveGrid {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Grid)
    }

    fn from_props(props: crate::parsers::conundrum::logic::object::object::ConundrumObject,
                  children: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>)
                  -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<Self>
        where Self: Sized {
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();
        let children = Children(children.unwrap_or_default());
        Ok(ResponsiveGrid { sizable,
                            children })
    }
}
