use crate::{
    lang::lib::ui::components::{
        component_config::{
            component_config::ComponentConfig, component_docs::ComponentDocs, component_kind::ComponentKind,
        },
        component_trait::ConundrumComponent,
        media::video::youtube::youtube_component::YoutubeComponent,
    },
};

/// The in-content documentation for the `Youtube` component, kept next to every
/// other embedded component doc so that `Youtube?` and `Youtube??` show exactly
/// the same text the AI tooling sees.
const YOUTUBE_DOCS_FULL: &str = include_str!("../../../../../../../embedded/component_docs/youtube-full.mdx");
const YOUTUBE_DOCS_SHORT: &str = include_str!("../../../../../../../embedded/component_docs/youtube-short.mdx");

/// ## Youtube config
///
/// The [`ComponentConfig`] for [`YoutubeComponent`]: its registry key, kind and
/// documentation.
pub fn youtube_component_config() -> ComponentConfig {
    ComponentConfig { name: YoutubeComponent::get_component_id().to_component_key(),
                      component_type: ComponentKind::UserEmbeddable,
                      docs: ComponentDocs { full: YOUTUBE_DOCS_FULL.to_string(),
                                            short: YOUTUBE_DOCS_SHORT.to_string() } }
}
