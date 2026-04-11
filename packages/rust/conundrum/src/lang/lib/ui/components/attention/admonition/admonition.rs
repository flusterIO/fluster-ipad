use serde::Serialize;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent,
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{
                children::Children, common_component_property_key::CommonComponentPropertyKey, emphasis::Emphasis,
                heading_depth::HeadingDepth,
            },
        },
        runtime::{
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                fluster_component_result::ConundrumComponentResult,
                inline_markdown_component_result::InlineMarkdownComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{
        bool::boolean::ConundrumBoolean, number::conundrum_int::ConundrumInt, object::object::ConundrumObject,
        string::conundrum_string::ConundrumString,
    },
};

fn default_markdown_title_depth() -> HeadingDepth {
    HeadingDepth(ConundrumInt(5))
}

#[typeshare::typeshare]
#[derive(Debug, Serialize, Default, Clone)]
pub struct Admonition {
    pub title: ConundrumString,
    pub children: Children,
    /// The title depth between 1-6 for the markdown output. This will have no
    /// effect on mdx, jsx, or plain text output. Defaults to 5
    /// ```rust
    /// let admonition = Admonition {
    ///    title: vec![],
    ///    children: vec![],
    ///    markdown_title_depth: None
    /// }
    /// assert_eq!(admonition.markdown_title_depth, Some(5));
    /// ```
    #[serde(default = "default_markdown_title_depth")]
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
    fn to_jsx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let title_string = self.title.to_children(res.modifiers.clone())?.to_jsx_prop("title", res)?;
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
    fn to_markdown(&self,
                   res: &mut crate::lang::runtime::state::parse_state::ParseState)
                   -> ConundrumModalResult<String> {
        let depth = self.markdown_title_depth.unwrap_or(HeadingDepth(ConundrumInt(5)));
        let title_string = self.title.to_children(res.modifiers.clone())?.render(res)?;
        Ok(format!(
                   r#"{} {}

{}"#,
                   depth,
                   title_string,
                   self.children.render(res)?
        ))
    }
}

impl InlineMarkdownComponentResult for Admonition {
    fn to_inline_markdown(&self,
                          res: &mut crate::lang::runtime::state::parse_state::ParseState)
                          -> ConundrumModalResult<String> {
        self.children.render_bypassed(res);
        let title_string = self.title.to_children(res.modifiers.clone())?.render(res)?;
        Ok(title_string)
    }
}

impl PlainTextComponentResult for Admonition {
    fn to_plain_text(&self,
                     res: &mut crate::lang::runtime::state::parse_state::ParseState)
                     -> ConundrumModalResult<String> {
        let title_string = self.title.to_children(res.modifiers.clone())?.render(res)?;
        Ok(format!(
                   r#"{}
{}"#,
                   title_string,
                   self.children.render(res)?
        ))
    }
}

impl ConundrumComponentResult for Admonition {
    fn to_conundrum_component(&self,
                              res: &mut crate::lang::runtime::state::parse_state::ParseState)
                              -> ConundrumModalResult<String> {
        if res.contains_modifier(&ConundrumModifier::PreferInlineMarkdownSyntax) {
            self.to_inline_markdown(res)
        } else if res.contains_modifier(&ConundrumModifier::PreferMarkdownSyntax) {
            self.to_markdown(res)
        } else if res.contains_one_of_modifiers(vec![ConundrumModifier::ForcePlainText,
                                                     ConundrumModifier::ForSearchInput])
        {
            self.to_plain_text(res)
        } else {
            self.to_jsx_component(res)
        }
    }
}

impl ConundrumComponent for Admonition {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Admonition)
    }

    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self>
        where Self: Sized {
        let markdown_title_depth =
            HeadingDepth::from_props(&props, CommonComponentPropertyKey::MarkdownHeading.to_string().as_str()).ok();

        let emphasis = Emphasis::from_jsx_props(&props, "").ok();

        let title = ConundrumString::from_jsx_props(&props, "title")?;

        let folded = ConundrumBoolean::from_jsx_props(&props, "folded").ok();
        let foldable = ConundrumBoolean::from_jsx_props(&props, "foldable").ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();

        Ok(Admonition { title,
                        children: children.map(Children).unwrap_or_else(Children::new_empty),
                        markdown_title_depth,
                        emphasis,
                        folded,
                        foldable,
                        sizable })
    }
}
