use askama::Template;
use serde::Serialize;
use std::sync::Arc;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::{
                attention::admonition::admonition_html_template::AdmonitionHtmlTemplate,
                component_trait::ConundrumComponent,
            },
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{
                children::Children, common_component_property_key::CommonComponentPropertyKey, emphasis::Emphasis,
                heading_depth::HeadingDepth,
            },
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::{ConundrumCompileTarget, ConundrumModifier},
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::{
        general::component_constants::{any_component_id::AnyComponentName, component_names::EmbeddableComponentName},
        html::dom::dom_id::DOMId,
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, number::conundrum_int::ConundrumInt, object::object::ConundrumObject,
        string::conundrum_string::ConundrumString,
    },
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Clone)]
pub struct Admonition {
    pub title: Children,
    pub id: DOMId,
    pub children: Children,
    /// The title depth between 1-6 for the markdown output. This will have no
    /// effect on mdx, jsx, or plain text output. Defaults to 5
    pub markdown_title_depth: Option<HeadingDepth>,
    pub emphasis: Option<Emphasis>,
    pub sizable: Option<SizablePropsGroup>,
    /// Set to true to make the admonition foldable. Default: `false`
    pub foldable: Option<ConundrumBoolean>,
    /// If the admonition is `foldable`, `folded` can make the admonition folded
    /// by default. If the `foldable` property is false, this property does
    /// nothing.
    pub folded: Option<ConundrumBoolean>,
}

impl JsxComponentResult for Admonition {
    fn to_jsx_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let cloned_state = Arc::clone(&res);
        let title_string = self.title.to_jsx_prop("title", cloned_state)?;
        let mut props = vec![title_string];
        if let Some(emphasis) = &self.emphasis {
            props.push(emphasis.to_string())
        }
        if self.folded.is_some_and(|x| x.0) {
            props.push("folded".to_string())
        }
        if self.foldable.is_some_and(|x| x.0) {
            props.push("foldable".to_string())
        }
        if let Some(sizable) = &self.sizable {
            props.push(sizable.to_jsx_prop())
        }
        Ok(format!(
                   r#"<{} {}>
{}
</{}>"#,
                   EmbeddableComponentName::Admonition,
                   props.join(" "),
                   self.children.render(res)?,
                   EmbeddableComponentName::Admonition
        ))
    }
}

impl MarkdownComponentResult for Admonition {
    fn to_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        let depth = self.markdown_title_depth.unwrap_or(HeadingDepth(ConundrumInt(5)));
        let cloned_state = Arc::clone(&res);
        let title_string = self.title.render(cloned_state)?;
        Ok(format!(
                   r#"{} {}

{}"#,
                   depth,
                   title_string,
                   self.children.render(res)?
        ))
    }
}

impl HtmlJsComponentResult for Admonition {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = AdmonitionHtmlTemplate { sizable: self.sizable.as_ref().cloned().unwrap_or_default(),
                                             foldable: self.foldable.unwrap_or_default(),
                                             id: self.id.clone(),
                                             folded: self.folded.unwrap_or_default(),
                                             title_children: self.title.render(Arc::clone(&res))?,
                                             body_children: self.children.render(Arc::clone(&res))?,
                                             emphasis: self.emphasis.as_ref().cloned().unwrap_or(Emphasis::Primary) };
        templ.render().map_err(|e| {
                    eprintln!("Error: {:#?}", e);
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl InlineMarkdownComponentResult for Admonition {
    fn to_inline_markdown(&self, res: ArcState) -> ConundrumModalResult<String> {
        self.children.render_bypassed(Arc::clone(&res));
        let title_string = self.title.render(res)?;
        Ok(title_string)
    }
}

impl PlainTextComponentResult for Admonition {
    fn to_plain_text(&self, res: ArcState) -> ConundrumModalResult<String> {
        let cloned_res = Arc::clone(&res);
        let title_string = self.title.render(cloned_res)?;
        Ok(format!(
                   r#"{}
{}"#,
                   title_string,
                   self.children.render(res)?
        ))
    }
}

impl ConundrumComponentResult for Admonition {
    fn to_conundrum_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let state = res.read_arc();
        if state.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            drop(state);
            self.to_inline_markdown(res)
        } else if state.targets_markdown() {
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

impl ConundrumComponent for Admonition {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Admonition)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  state: ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let markdown_title_depth =
            HeadingDepth::from_props(&props, CommonComponentPropertyKey::MarkdownHeading.to_string().as_str()).ok();

        let emphasis = Emphasis::from_jsx_props(&props, "").ok();

        let title = ConundrumString::from_jsx_props(&props, "title")?;

        let folded = ConundrumBoolean::from_jsx_props(&props, "folded").ok();
        let foldable = ConundrumBoolean::from_jsx_props(&props, "foldable").ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();

        let title_children = title.to_children(Arc::clone(&state))?;
        let mut locked_state = state.write_arc();
        let id = locked_state.dom.new_id();
        drop(locked_state);

        Ok(Admonition { title: title_children,
                        children: children.map(Children).unwrap_or_else(Children::new_empty),
                        markdown_title_depth,
                        emphasis,
                        folded,
                        foldable,
                        id,
                        sizable })
    }
}
