use askama::Template;
use serde::Serialize;
use winnow::{Parser, combinator::delimited, error::ErrMode, token::take_while};

use crate::{
    lang::{
        lib::ui::{
            components::component_trait::ConundrumComponent,
            shared_props::{
                sizable::{self, SizablePropsGroup},
                sizable_option::SizableOption,
            },
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumModifier,
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::{
        as_char_extensions::is_space_or_newline,
        conundrum::logic::{bool::boolean::ConundrumBoolean, string::conundrum_string::ConundrumString},
        markdown::markdown_extensions::emoji::emoji_html_templ::EmojiHtmlTemplate,
        parser_trait::ConundrumParser,
    },
};

/// Use the built-in `:smile:` syntax to insert a text sized emoji, or use the
/// `Emoji` component to create a scalable and resizable emoji as an image.
#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct EmojiResult {
    pub name: ConundrumString,
    /// Because images are notoriously finicky to style, you should prefer the
    /// sizable boolean keys instead of the `width="small"` or similar
    /// properties. This means that your component might end up looking
    /// something like:
    ///
    /// ```tsx
    /// <Emoji name="smile" medium border ... />
    /// ```
    ///
    /// Instead of
    ///
    /// ```tsx
    /// <Emoji name="smile" width="medium" border ... />
    /// ```
    ///
    /// The other properties are still available, but these unique boolean
    /// properties will apply styles that will more reliably shape the
    /// underlying image.
    pub sizable: Option<SizablePropsGroup>,
    /// Default: "small", text sized.
    pub size: Option<SizableOption>,
}

impl EmojiResult {
    pub fn get_svg(&self) -> Option<String> {
        twemoji_assets::svg::SvgTwemojiAsset::from_name(&self.name.0).map(|res| res.data.0.to_string())
    }
}

impl PlainTextComponentResult for EmojiResult {
    fn to_plain_text(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl MarkdownComponentResult for EmojiResult {
    fn to_markdown(&self, _: ArcState) -> ConundrumModalResult<String> {
        Ok(format!(":{}:", self.name))
    }
}

impl HtmlJsComponentResult for EmojiResult {
    fn to_html_js_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        if let Some(svg) = self.get_svg() {
            let templ = EmojiHtmlTemplate { size: self.size.as_ref().cloned().unwrap_or(SizableOption::Small),
                                            sizable_classes: self.sizable
                                                                 .as_ref()
                                                                 .cloned()
                                                                 .map(|x| x.as_class())
                                                                 .unwrap_or_default(),
                                            tag: self.sizable
                                                     .as_ref()
                                                     .cloned()
                                                     .map(|sizable| {
                                                         if sizable.inline.is_some_and(|x| x.0) {
                                                             "span".to_string()
                                                         } else {
                                                             "div".to_string()
                                                         }
                                                     })
                                                     .unwrap_or_else(|| "div".to_string()),
                                            emoji: svg };
            templ.render().map_err(|e| {
                eprintln!("Error: {:#?}", e);
                ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
            })
        } else {
            Err(
                ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid Emoji", format!("Conundrum attempted to render an invalid emoji: `{}`", self.name ).as_str())))
            )
        }
    }
}

impl JsxComponentResult for EmojiResult {
    fn to_jsx_component(&self, _: ArcState) -> ConundrumModalResult<String> {
        let svg = self.get_svg()
                .ok_or_else(|| {
                    ErrMode::Cut(
                        ConundrumErrorVariant::UserFacingGeneralParserError(
                            ConundrumError::from_msg_and_details("Invalid emoji", format!("The `{}` key is not a valid emoji name. Use the `Emoji??` docs to see available Emojis.", self.name).as_str())
                        )
                    )
                })?;
        let s = self.name.to_jsx_prop_as_string("name").map_err(|_| {
                ErrMode::Cut(
                    ConundrumErrorVariant::UserFacingGeneralParserError(
                        ConundrumError::from_msg_and_details("Serialization error", "Conundrum failed to serialize an emoji name.")
                    )
                )
            })?;
        Ok(format!("<{} {} {} {}>{}</{}>",
                   EmbeddableComponentName::Emoji,
                   s,
                   self.size.as_ref().unwrap_or(&SizableOption::Small),
                   self.sizable.as_ref().cloned().map(|x| x.to_jsx_prop()).unwrap_or_default(),
                   svg,
                   EmbeddableComponentName::Emoji))
    }
}

impl ConundrumComponentResult for EmojiResult {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.contains_one_of_modifiers(vec![ConundrumModifier::ForSearchInput, ConundrumModifier::HideEmojis]) {
            drop(state);
            self.to_plain_text(res)
        } else if state.targets_markdown() {
            drop(state);
            self.to_markdown(res)
        } else {
            drop(state);
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumParser<EmojiResult> for EmojiResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumModalResult<EmojiResult> {
        let value = delimited(':', take_while(1.., |c| !is_space_or_newline(c) && c != ':'), ':').parse_next(input)?;

        let sizable = SizablePropsGroup { inline: Some(ConundrumBoolean(true)),
                                          ..Default::default() };

        Ok(EmojiResult { name: ConundrumString(value.to_string()),
                         size: None,
                         sizable: Some(sizable) })
    }

    fn matches_first_char(char: char) -> bool {
        char == ':' || char == '<'
    }
}

impl ConundrumComponent for EmojiResult {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Emoji)
    }

    fn from_props(props: crate::parsers::conundrum::logic::object::object::ConundrumObject,
                  _: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>,
                  _: ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let name = ConundrumString::from_jsx_props(&props, "name").map_err(|e| e.cut())?;
        let size = SizableOption::from_jsx_props_bool_record(&props).unwrap_or_else(|| {
                                                                        let x =
                                                                            SizableOption::from_jsx_props(&props,
                                                                                                          "size").ok();
                                                                        x.unwrap_or(SizableOption::Small)
                                                                    });
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();
        Ok(EmojiResult { name,
                         size: Some(size),
                         sizable })
    }
}
