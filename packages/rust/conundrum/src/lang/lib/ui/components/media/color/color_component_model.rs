use askama::{PrimitiveType, Template};
use serde::Serialize;
use tw_merge::*;
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            components::component_trait::ConundrumComponent, shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_template::HTMLTemplateRepresentable, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::{color::conundrum_color::ConundrumColor, logic::object::object::ConundrumObject},
};

#[typeshare]
#[derive(Debug, Serialize, serde::Deserialize, Clone)]
pub struct ColorComponent {
    pub background_dark: ConundrumColor,
    pub foreground_dark: Option<ConundrumColor>,
    pub background_light: Option<ConundrumColor>,
    pub foreground_light: Option<ConundrumColor>,
    pub sizable: SizablePropsGroup,
}

#[derive(Template)]
pub enum ColorComponentTemplateVariants {
    #[template(path = "components/color/single_color.html", ext = "html")]
    SingleColor {
        color: ConundrumColor,
        sizable: SizablePropsGroup,
    },
    #[template(path = "components/color/foreground_background.html", ext = "html")]
    ForegroundBackground {
        foreground: ConundrumColor,
        background: ConundrumColor,
        sizable: SizablePropsGroup,
    },
    #[template(path = "components/color/complete_color.html", ext = "html")]
    CompleteColor {
        foreground_light: ConundrumColor,
        background_light: ConundrumColor,
        foreground_dark: ConundrumColor,
        background_dark: ConundrumColor,
        sizable: SizablePropsGroup,
    },
}

impl PlainTextComponentResult for ColorComponent {
    fn to_plain_text(&self,
                     res: crate::lang::runtime::traits::conundrum_input::ArcState)
                     -> ConundrumModalResult<String> {
        todo!()
    }
}

impl MarkdownComponentResult for ColorComponent {
    fn to_markdown(&self,
                   res: crate::lang::runtime::traits::conundrum_input::ArcState)
                   -> ConundrumModalResult<String> {
        todo!()
    }
}

impl ConundrumComponentResult for ColorComponent {
    fn to_conundrum_component(&self,
                              res: crate::lang::runtime::traits::conundrum_input::ArcState)
                              -> ConundrumModalResult<String> {
        todo!()
    }
}

impl HTMLTemplateRepresentable<ColorComponentTemplateVariants> for ColorComponent {
    fn as_template(&self) -> ColorComponentTemplateVariants {
        if let Some(foreground_dark) = &self.foreground_dark {
            if let Some(background_light) = &self.background_light
               && let Some(foreground_light) = &self.foreground_light
            {
                ColorComponentTemplateVariants::CompleteColor { foreground_light: foreground_light.clone(),
                                                                background_light: background_light.clone(),
                                                                foreground_dark: foreground_dark.clone(),
                                                                background_dark: self.background_dark.clone(),
                                                                sizable: self.sizable.clone() }
            } else {
                ColorComponentTemplateVariants::ForegroundBackground { foreground: foreground_dark.clone(),
                                                                       background: self.background_dark.clone(),
                                                                       sizable: self.sizable.clone() }
            }
        } else {
            ColorComponentTemplateVariants::SingleColor { color: self.background_dark.clone(),
                                                          sizable: self.sizable.clone() }
        }
    }
}

impl HtmlJsComponentResult for ColorComponent {
    fn to_html_js_component(&self,
                            res: crate::lang::runtime::traits::conundrum_input::ArcState)
                            -> ConundrumModalResult<String> {
        self.as_template().render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl ConundrumComponent for ColorComponent {
    fn get_component_id() -> crate::output::general::component_constants::any_component_id::AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Color)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>,
                  state: crate::lang::runtime::traits::conundrum_input::ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        todo!()
    }
}

enum FirstColorSource {
    BackgroundDark,
    Background,
    Color,
}

impl FromJsxPropsOptional for ColorComponent {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        let (bg_color, color_source) = match props.get_string("backgroundDark", None) {
            Ok(r) => (r, FirstColorSource::BackgroundDark),
            Err(e) => match props.get_string("background", None) {
                Ok(s) => (s, FirstColorSource::Background),
                Err(_) => (props.get_string("color", None).map_err(|e| {
                    ErrMode::Cut(
                        ConundrumErrorVariant::MissingRequiredComponentProperty { component: Self::get_component_id(), key: String::from("color | background | backgroundDark"), expected: "Color".to_string() }
                    )
                })?, FirstColorSource::Color),
            },
        };
        let fg_color = match color_source {
            FirstColorSource::Background => props.get_string("foreground", None).ok(),
            FirstColorSource::BackgroundDark => props.get_string("foregroundDark", None).ok(),
            _ => None,
        };
        let fg_light = props.get_string("foregroundLight", None).ok();
        let bg_light = props.get_string("backgroundLight", None).ok();
        let template_inline = props.get_boolean("inline", None).ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").unwrap_or_default();
        Ok(ColorComponent { background_dark: bg_color.try_into()?,
                            foreground_dark: match fg_color {
                                Some(s) => Some(s.try_into()?),
                                None => None,
                            },
                            background_light: match bg_light {
                                Some(s) => Some(s.try_into()?),
                                None => None,
                            },
                            foreground_light: match fg_light {
                                Some(s) => Some(s.try_into()?),
                                None => None,
                            },
                            sizable })
    }
}
