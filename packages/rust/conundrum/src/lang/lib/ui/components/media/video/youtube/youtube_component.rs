use std::sync::Arc;

use askama::Template;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            components::component_trait::ConundrumComponent,
            shared_props::sizable::{SizablePropsGroup, SizablePropsOutputTarget},
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
/// For embedding YouTube videos directly in your notes. The video renders in a
/// responsive 16:9 player (capped at `768px` wide) loaded from
/// `youtube-nocookie.com`, and accepts every [`SizablePropsGroup`] property so
/// it can be floated, bordered, rounded, and so on.
///
/// The full user-facing documentation lives in
/// `embedded/component_docs/youtube-full.mdx` and is registered through
/// [`youtube_component_config`](super::youtube_config::youtube_component_config).
///
/// ### Properties
///
/// - `url` (required): any common YouTube url (`watch?v=`, `youtu.be/`,
///   `/embed/`, `/shorts/`, `/live/`) or a bare 11 character video id. A start
///   time in the url (`t=90`, `t=90s`, `start=90`) is honored.
/// - `id` (optional): a user defined id used for timestamp links.
/// - Any sizable property.
///
/// ### Example
///
/// ```tsx
/// <Youtube
///    url="https://youtube.com/watch?v=someVideoId"
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

const YOUTUBE_ID_LEN: usize = 11;

fn is_valid_youtube_id(candidate: &str) -> bool {
    candidate.len() == YOUTUBE_ID_LEN && candidate.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

impl YoutubeComponent {
    /// Pulls the 11 character video id out of any common YouTube url shape, or
    /// accepts a bare id. Returns `None` when no valid id can be found.
    pub fn video_id(&self) -> Option<String> {
        let raw = self.url.0.trim();
        if is_valid_youtube_id(raw) {
            return Some(raw.to_string());
        }

        let without_scheme = raw.split("://").last().unwrap_or(raw);
        let (path_part, query) = match without_scheme.split_once('?') {
            Some((p, q)) => (p, q),
            None => (without_scheme, ""),
        };
        let path_part = path_part.split('#').next().unwrap_or(path_part);

        // `watch?v=ID`
        if let Some(v) = query.split(['&', '#']).find_map(|kv| kv.strip_prefix("v=")) {
            if is_valid_youtube_id(v) {
                return Some(v.to_string());
            }
        }

        // `youtu.be/ID`, `/embed/ID`, `/shorts/ID`, `/live/ID`, `/v/ID`
        let segments: Vec<&str> = path_part.split('/').filter(|s| !s.is_empty()).collect();
        let host = segments.first().copied().unwrap_or("");
        if host.ends_with("youtu.be") {
            return segments.get(1).filter(|s| is_valid_youtube_id(s)).map(|s| s.to_string());
        }
        segments.iter()
                .position(|s| matches!(*s, "embed" | "shorts" | "live" | "v"))
                .and_then(|i| segments.get(i + 1))
                .filter(|s| is_valid_youtube_id(s))
                .map(|s| s.to_string())
    }

    /// The start time, in seconds, found in the url (`t=90`, `t=90s`,
    /// `start=90`), if any.
    pub fn start_seconds(&self) -> Option<u32> {
        let (_, query) = self.url.0.split_once('?')?;
        query.split(['&', '#'])
             .filter_map(|kv| kv.strip_prefix("t=").or_else(|| kv.strip_prefix("start=")))
             .find_map(|v| v.trim_end_matches('s').parse::<u32>().ok())
    }

    /// The `youtube-nocookie.com` embed url. `enablejsapi` is on so that
    /// `video:` timestamp links can seek the player. An unparseable url yields
    /// an empty player rather than failing the whole note.
    pub fn embed_url(&self) -> String {
        let Some(id) = self.video_id() else {
            return String::new();
        };
        let mut url = format!("https://www.youtube-nocookie.com/embed/{}?enablejsapi=1&rel=0", id);
        if let Some(start) = self.start_seconds() {
            url.push_str(&format!("&start={}", start));
        }
        url
    }

    /// Tailwind classes for the player's container, merged with the user's
    /// sizable properties.
    pub fn container_classes(&self) -> String {
        tw_merge::tw_merge!("video-container aspect-video max-w-[768px] w-full",
                            self.sizable.as_class(SizablePropsOutputTarget::General))
    }
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
                   _: crate::lang::runtime::traits::conundrum_input::ArcState)
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
        let url = props.get_string("url", Some("A url is a required field for the Youtube component and must be a string."))
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
