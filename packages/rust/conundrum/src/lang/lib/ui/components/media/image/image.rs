use std::sync::Arc;

use askama::Template;
use serde::Serialize;
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::{
                component_trait::ConundrumComponent,
                media::image::{
                    any_image_html_templ::AnyImageHtmlTemplate, image_html_templ::ImageHtmlTemplate,
                    image_html_templ_with_caption::ImageHtmlTemplateWithCaption,
                },
            },
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::children::Children,
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumModifier,
            },
            traits::{
                conundrum_input::ArcState, conundrum_template::ConundrumTemplateRepresentable,
                fluster_component_result::ConundrumComponentResult, html_js_component_result::HtmlJsComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::{
        general::component_constants::{any_component_id::AnyComponentName, component_names::EmbeddableComponentName},
        html::dom::dom_id::DOMId,
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, object::object::ConundrumObject, string::conundrum_string::ConundrumString,
    },
};

#[typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct Image {
    pub sizable: Option<SizablePropsGroup>,
    pub src: ConundrumString,
    pub caption: Option<Children>,
    pub caption_left: Option<ConundrumBoolean>,
    pub caption_right: Option<ConundrumBoolean>,
    pub contain: Option<ConundrumBoolean>,
    pub cover: Option<ConundrumBoolean>,
    pub alt: Option<ConundrumString>,
    /// An id is required to make sure all figures are counted.
    pub id: DOMId,
}

impl ConundrumTemplateRepresentable<AnyImageHtmlTemplate> for Image {
    fn to_template(&self, state: ArcState) -> ConundrumModalResult<AnyImageHtmlTemplate> {
        if let Some(caption) = &self.caption {
            Ok(AnyImageHtmlTemplate::WithCaption(ImageHtmlTemplateWithCaption { caption:
                caption.render(Arc::clone(&state))?,
                caption_left:
                    self.caption_left
                    .map(|x| x.0)
                    .unwrap_or_default(),
                    caption_right:
                        self.caption_right
                        .map(|x| x.0)
                        .unwrap_or_default(),
                        sizable: self.sizable.clone(),
                        img_templ:
                            ImageHtmlTemplate { id:
                                self.id
                                    .clone(),
                                    sizable: self.sizable.as_ref().cloned().unwrap_or_default(),
                                    src:
                                        self.src
                                        .clone(),
                                        cover: self.cover
                                            .map(|x| x.0)
                                            .unwrap_or_default(),
                                            contain: self.contain
                                                .map(|x| x.0)
                                                .unwrap_or_default(),
                                                alt:
                                                    self.alt
                                                    .clone() } }))
        } else {
            Ok(AnyImageHtmlTemplate::NoCaption(ImageHtmlTemplate { id: self.id.clone(),
                                                                   sizable: self.sizable
                                                                                .as_ref()
                                                                                .cloned()
                                                                                .unwrap_or_default(),
                                                                   src: self.src.clone(),
                                                                   cover: self.cover
                                                                              .map(|x| x.0)
                                                                              .unwrap_or_default(),
                                                                   contain: self.contain
                                                                                .map(|x| x.0)
                                                                                .unwrap_or_default(),
                                                                   alt: self.alt.clone() }))
        }
    }
}

impl InlineMarkdownComponentResult for Image {
    fn to_inline_markdown(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl MarkdownComponentResult for Image {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        let read_state = res.read_arc();
        if read_state.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            drop(read_state);
            self.to_inline_markdown(res)
        } else {
            Ok(format!("![{}]({})", self.alt.as_ref().cloned().unwrap_or_default(), self.src))
        }
    }
}

impl PlainTextComponentResult for Image {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl HtmlJsComponentResult for Image {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = self.to_template(Arc::clone(&res))?;
        templ.render().map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
        })
    }
}

impl ConundrumComponentResult for Image {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.targets_markdown() {
            drop(state);
            self.to_markdown(res)
        } else if state.targets_html_js() {
            drop(state);
            self.to_html_js_component(res)
        } else {
            drop(state);
            Ok(String::from(""))
        }
    }
}

impl ConundrumComponent for Image {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Image)
    }

    fn from_props(props: ConundrumObject,
                  _: Option<Vec<ParsedElement>>,
                  state: ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let src = ConundrumString::from_jsx_props(&props, "src").map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Missing Source", "Every image requires a valid `src` property. This property should be a string and point to an image url.")))
                })?;
        let alt = ConundrumString::from_jsx_props(&props, "alt").ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();
        let caption = Children::from_jsx_props(&props, "caption").ok();
        let caption_left = ConundrumBoolean::from_jsx_props(&props, "captionLeft").ok();
        let caption_right = ConundrumBoolean::from_jsx_props(&props, "captionRight").ok();
        let contain = ConundrumBoolean::from_jsx_props(&props, "contain").ok();
        let cover = ConundrumBoolean::from_jsx_props(&props, "cover").ok();
        let mut write_state = state.write_arc();
        let id = write_state.dom.new_id();
        drop(write_state);
        Ok(Image { sizable,
                   caption,
                   caption_left,
                   caption_right,
                   contain,
                   cover,
                   src,
                   alt,
                   id })
    }
}
