use serde::Serialize;
use winnow::{Parser, combinator::delimited, error::ErrMode, token::take_while};

use crate::{
    lang::{
        lib::ui::{
            components::component_trait::ConundrumComponent, shared_props::sizable_option::SizableOption,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::ConundrumModifier,
            },
            traits::{
                fluster_component_result::ConundrumComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::component_names::EmbeddableComponentName,
    parsers::{
        as_char_extensions::is_space_or_newline, conundrum::logic::string::conundrum_string::ConundrumString,
        parser_trait::ConundrumParser,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct EmojiResult {
    pub name: ConundrumString,
    /// Default: "small", text sized.
    pub size: Option<SizableOption>,
}

impl EmojiResult {
    pub fn get_svg(&self) -> Option<String> {
        twemoji_assets::svg::SvgTwemojiAsset::from_name(&self.name.0).map(|res| res.data.0.to_string())
    }
}

impl PlainTextComponentResult for EmojiResult {
    fn to_plain_text(&self,
                     _: &mut crate::lang::runtime::state::parse_state::ParseState)
                     -> ConundrumModalResult<String> {
        Ok(String::from(""))
    }
}

impl MarkdownComponentResult for EmojiResult {
    fn to_markdown(&self,
                   _: &mut crate::lang::runtime::state::parse_state::ParseState)
                   -> ConundrumModalResult<String> {
        Ok(format!(":{}:", self.name))
    }
}

impl JsxComponentResult for EmojiResult {
    fn to_jsx_component(&self,
                        _: &mut crate::lang::runtime::state::parse_state::ParseState)
                        -> ConundrumModalResult<String> {
        let svg = self.get_svg()
            .ok_or_else(|| {
            ErrMode::Cut(
                ConundrumErrorVariant::UserFacingGeneralParserError(
                    ConundrumError::from_msg_and_details("Invalid emoji", format!("The `{}` key is not a valid emoji name. Use the `Emoji??` docs to see available Emojis.", self.name).as_str())
                )
            )
        })?;
        Ok(format!(
                   "<{} {} inline {}>{}</{}>",
                   EmbeddableComponentName::Emoji,
                   self.name.to_jsx_prop_as_string("name").map_err(|_| {
                       ErrMode::Cut(
    ConundrumErrorVariant::UserFacingGeneralParserError(
        ConundrumError::from_msg_and_details("Serialization error", "Conundrum failed to serialize an emoji name.")
    )
)
                   })?,
                   self.size.as_ref().unwrap_or_else(|| &SizableOption::Small).to_string(),
                   svg,
                   EmbeddableComponentName::Emoji
        ))
    }
}

impl ConundrumComponentResult for EmojiResult {
    fn to_conundrum_component(&self,
                              res: &mut crate::lang::runtime::state::parse_state::ParseState)
                              -> ConundrumModalResult<String> {
        if res.contains_modifier(&ConundrumModifier::ForSearchInput) {
            self.to_plain_text(res)
        } else if res.is_markdown() {
            self.to_markdown(res)
        } else {
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumParser<EmojiResult> for EmojiResult {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumModalResult<EmojiResult> {
        let value = delimited(':', take_while(1.., |c| !is_space_or_newline(c) && c != ':'), ':').parse_next(input)?;

        Ok(EmojiResult { name: ConundrumString(value.to_string()),
                         size: None })
    }

    fn matches_first_char(char: char) -> bool {
        char == ':'
    }
}

impl ConundrumComponent for EmojiResult {
    fn get_component_id() -> crate::output::general::component_constants::component_ids::EmbeddableComponentId {
        todo!()
    }

    fn from_props(props: crate::parsers::conundrum::logic::object::object::ConundrumObject,
                  _: Option<Vec<crate::lang::elements::parsed_elements::ParsedElement>>)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let name = ConundrumString::from_jsx_props(&props, "name")?;
        let size = SizableOption::from_jsx_props_bool_record(&props);
        Ok(EmojiResult { name,
                         size })
    }
}
