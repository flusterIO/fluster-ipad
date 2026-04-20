use std::sync::Arc;

use serde::Serialize;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, heading_depth::HeadingDepth},
        },
        runtime::{
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumCompileTarget, ConundrumModifier},
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{object::object::ConundrumObject, string::conundrum_string::ConundrumString},
};

fn default_markdown_title_depth() -> u8 {
    5
}

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct Card {
    pub title: ConundrumString,
    pub subtitle: Option<ConundrumString>,
    pub children: Children,
    /// The title depth between 1-6 for the markdown output. This will have no
    /// effect on mdx, jsx, or plain text output. Defaults to 5
    /// ```rust
    /// let card = Card {
    ///    title: vec![],
    ///    subtitle: None,
    ///    children: vec![],
    ///    markdown_title_depth: None
    /// }
    /// assert_eq!(card.markdown_title_depth, Some(5));
    /// ```
    #[serde(default = "default_markdown_title_depth")]
    pub markdown_title_depth: Option<HeadingDepth>,
}

impl JsxComponentResult for Card {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let title_string = self.title.to_children(Arc::clone(&res))?.to_jsx_fragment_string(Arc::clone(&res))?;

        let subtitle_string = match &self.subtitle {
            Some(s) => s.to_children(Arc::clone(&res))?.to_jsx_fragment_string(Arc::clone(&res))?,
            None => "".to_string(),
        };

        Ok(format!(
                   r#"<{} title={} subtitle={} >
{}
</{}>"#,
                   EmbeddableComponentName::Card,
                   title_string,
                   subtitle_string,
                   self.children.render(Arc::clone(&res))?,
                   EmbeddableComponentName::Card,
        ))
    }
}

impl InlineMarkdownComponentResult for Card {
    fn to_inline_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl PlainTextComponentResult for Card {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        let title = self.title.to_children(res.clone())?.render(Arc::clone(&res))?;
        let children = self.children.render(Arc::clone(&res))?;

        Ok(format!(
                   r#"{}
{}"#,
                   title, children
        ))
    }
}

impl MarkdownComponentResult for Card {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        let title_string = self.title.to_children(Arc::clone(&res))?.render(Arc::clone(&res))?;

        let subtitle_string = match &self.subtitle {
            Some(s) => {
                let subtitle_children_string = s.to_children(Arc::clone(&res))?.render(Arc::clone(&res))?;
                format!("\n\n> {}", subtitle_children_string)
            }
            None => "".to_string(),
        };
        let mut s = String::from("");
        let depth = self.markdown_title_depth.unwrap_or_default();

        for _ in 1..depth.0.0 {
            s += "#"
        }
        let children_string = self.children.render(Arc::clone(&res))?;

        Ok(format!(
                   r#"{} {}{}

{}"#,
                   s, title_string, subtitle_string, children_string
        ))
    }
}

impl ConundrumComponentResult for Card {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            drop(state);
            self.to_inline_markdown(res)
        } else if state.contains_modifier(&ConundrumModifier::ForAIInput) {
            drop(state);
            self.to_markdown(res)
        } else if state.contains_modifier_or_matches_target(vec![ConundrumModifier::ForSearchInput],
                                                            vec![ConundrumCompileTarget::PlainText])
        {
            drop(state);
            self.to_plain_text(res)
        } else {
            drop(state);
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumComponent for Card {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Card)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  _: ArcState)
                  -> ConundrumModalResult<Self> {
        let heading_depth = HeadingDepth::from_props(&props, "markdown_heading_depth").ok();
        let title = ConundrumString::from_jsx_props(&props, "title")?;

        let subtitle = ConundrumString::from_jsx_props(&props, "desc").ok();

        let unwrapped_children = children.unwrap_or_default();

        Ok(Card { markdown_title_depth: heading_depth,
                  children: Children(unwrapped_children),
                  title,
                  subtitle })
    }
}
