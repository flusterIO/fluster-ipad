use std::sync::Arc;

use askama::Template;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            components::component_trait::ConundrumComponent, shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::{
            state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
            traits::{
                fluster_component_result::ConundrumComponentResult, html_js_component_result::HtmlJsComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::string::conundrum_string::ConundrumString,
};

/// ## Youtube
///
/// For embedding youtube videos directly in your notes.
///
/// ### Example
///
/// ```tsx
/// <Youtube
///    url="https://youtube.com/someVideoUrl"
///    id="myOptionalId"
/// />
/// ```
///
/// You can then use the id of the video to link to specific time stamps using
/// the `[my link content](video:myOptionalId@4:32)` to jump to 4 minutes and 32
/// seconds.
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone, Template)]
#[template(path = "components/media/video/youtube.html")]
pub struct YoutubeComponent {
    pub url: ConundrumString,
    /// An optional user defined id used for video timestamp links.
    pub id: Option<ConundrumString>,
    pub sizable: SizablePropsGroup,
}

impl HtmlJsComponentResult for YoutubeComponent {
    fn to_html_js_component(&self,
                            _: crate::lang::runtime::traits::conundrum_input::ArcState)
                            -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl PlainTextComponentResult for YoutubeComponent {
    fn to_plain_text(&self,
                     _: crate::lang::runtime::traits::conundrum_input::ArcState)
                     -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl MarkdownComponentResult for YoutubeComponent {
    fn to_markdown(&self,
                   res: crate::lang::runtime::traits::conundrum_input::ArcState)
                   -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl ConundrumComponent for YoutubeComponent {
    fn get_component_id() -> crate::output::general::component_constants::any_component_id::AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Youtube)
    }

    fn from_props(props: crate::parsers::conundrum::logic::object::object::ConundrumObject,
                  _: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>,
                  _: crate::lang::runtime::traits::conundrum_input::ArcState)
                  -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<Self>
        where Self: Sized {
        let url = props.get_string("url", Some("A url is a required field for the Youtube and must be a string."))
                       .map_err(|e| ErrMode::Cut(e))?;
        let id = props.get_string("id", None).ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").unwrap_or_default();
        Ok(YoutubeComponent { url,
                              id,
                              sizable })
    }
}

impl ConundrumComponentResult for YoutubeComponent {
    fn to_conundrum_component(&self,
                              res: crate::lang::runtime::traits::conundrum_input::ArcState)
                              -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String>
    {
        let state = res.read_arc();
        if state.targets_html_js() {
            drop(state);
            self.to_html_js_component(Arc::clone(&res))
        } else {
            Err(ErrMode::Backtrack(ConundrumErrorVariant::NotImplemented))
        }
    }
}
