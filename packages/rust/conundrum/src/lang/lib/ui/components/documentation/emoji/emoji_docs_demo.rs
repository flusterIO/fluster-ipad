use askama::Template;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tabled::{Table, settings::Style};
use winnow::error::ErrMode;

use crate::lang::{
    lib::ui::components::{
        attention::emoji::currently_supported_emoji_names::CURRENTLY_SUPPORTED_EMOJI_NAMES,
        documentation::emoji::emoji_data::EmojiData,
    },
    runtime::{
        state::{
            conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant,
            parse_state::ConundrumModifier,
        },
        traits::{
            fluster_component_result::ConundrumComponentResult, jsx_component_result::JsxComponentResult,
            markdown_component_result::MarkdownComponentResult, plain_text_component_result::PlainTextComponentResult,
        },
    },
};

///  ## The Jsx template
///
/// ```askama
/// <{{crate::output::general::component_constants::component_names::EmbeddableComponentName::Grid}}
///     responsive="medium"
///     gap="medium"
/// >
/// {% for emoji in get_emoji_data() %}
///     <{{crate::output::general::component_constants::documentation_component_name::DocumentationComponentName::EmojiDocumentationDemo}}} name="{{emoji.name}}">
///         {{ emoji.svg }}
///     </{{crate::output::general::component_constants::documentation_component_name::DocumentationComponentName::EmojiDocumentationDemo}}}>
/// {% endfor %}
/// </{{crate::output::general::component_constants::component_names::EmbeddableComponentName::Grid}}>
/// ```
#[derive(Template)]
#[template(ext = "jinja2", in_doc = true)]
pub struct EmojiDocsDemo {}

impl EmojiDocsDemo {
    pub fn get_emoji_data(&self) -> Vec<EmojiData> {
        CURRENTLY_SUPPORTED_EMOJI_NAMES.par_iter()
                                       .filter_map(|name| {
                                           twemoji_assets::svg::SvgTwemojiAsset::from_name(name).map(|item| {
                    EmojiData { name: name.to_string(), svg: item.data.0.to_string() }
                })
                                       })
                                       .collect::<Vec<EmojiData>>()
    }
}

impl MarkdownComponentResult for EmojiDocsDemo {
    fn to_markdown(&self,
                   _: &mut crate::lang::runtime::state::parse_state::ParseState)
                   -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let emojis =
            CURRENTLY_SUPPORTED_EMOJI_NAMES.par_iter()
                                           .filter_map(|name| {
                                               twemoji_assets::svg::SvgTwemojiAsset::from_name(name).map(|item| {
    EmojiData { name: name.to_string(), svg: item.data.0.to_string() }
})
                                           })
                                           .collect::<Vec<EmojiData>>();

        let mut table = Table::new(emojis);
        let style = Style::markdown();
        table.with(style);

        Ok(table.to_string())
    }
}

impl PlainTextComponentResult for EmojiDocsDemo {
    fn to_plain_text(&self,
                     _: &mut crate::lang::runtime::state::parse_state::ParseState)
                     -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let mut s = String::from("Emoji names:\n");
        for item in CURRENTLY_SUPPORTED_EMOJI_NAMES.iter() {
            s += format!("- \"{}\"\n", item).as_str();
        }
        Ok(s)
    }
}

impl JsxComponentResult for EmojiDocsDemo {
    fn to_jsx_component(&self,
                        _: &mut crate::lang::runtime::state::parse_state::ParseState)
                        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.render().map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Backtrack(ConundrumErrorVariant::InternalParserError(
                ConundrumError::from_message("Fail to render askama template.")
            ))
        })
    }
}

impl ConundrumComponentResult for EmojiDocsDemo {
    fn to_conundrum_component(&self,
                              res: &mut crate::lang::runtime::state::parse_state::ParseState)
                              -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String>
    {
        if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else if res.is_markdown() {
            self.to_markdown(res)
        } else {
            self.to_jsx_component(res)
        }
    }
}
