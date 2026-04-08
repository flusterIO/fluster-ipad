use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::component_trait::ConundrumComponent,
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis},
        },
        runtime::{
            state::{
                conundrum_error_variant::ConundrumModalResult,
                parse_state::{ConundrumModifier, ParseState},
            },
            traits::{
                fluster_component_result::ConundrumComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        component_ids::EmbeddableComponentId, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::object::object::ConundrumObject,
};

/// Group and organize your notes by swappable tabs. Pretty self explanatory
/// honestly...
///
/// ### Usage
/// > I know it's called `TabsGroup` here, but in Conundrum it's actually
/// > implemented as `Tabs`.
///
/// ```jsx
/// <Tabs>
///     <Tab label="My label">
///         My tab content one
///     </Tab>
///     <Tab label="My label 2">
///         My tab content two
///     </Tab>
///     <Tab label="My label"
///         // This id is required because the
///         // label appears more than once!
///         id="myUniqueId">
///         My tab content three
///     </Tab>
/// </Tabs>
/// ```
#[typeshare::typeshare]
#[derive(Debug, serde::Serialize, Clone)]
pub struct TabsGroup {
    /// The styles applied to the active tab. Default: `.card`
    pub emphasis: Option<Emphasis>,
    /// The `Tabs` component extends the `SizablePropsGroup` struct, but be
    /// careful... you're getting into the struggles of a developer now as
    /// these responsive containers can be difficult to make look good on
    /// all screen sizes. You'll get the hang of it, but be sure not to be too
    /// ambitious and implement a look that _barely_ fits, because it almost
    /// surely **won't** fit when you rotate the device or open another
    /// window.
    pub sizable: Option<SizablePropsGroup>,
    pub children: Children,
}

impl PlainTextComponentResult for TabsGroup {
    fn to_plain_text(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl MdxComponentResult for TabsGroup {
    fn to_mdx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumComponentResult for TabsGroup {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.is_markdown() {
            self.to_markdown(res)
        } else if res.contains_modifier(&ConundrumModifier::ForcePlainText) {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for TabsGroup {
    fn get_component_id() -> EmbeddableComponentId {
        EmbeddableComponentId::Tabs
    }

    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self> {
        let emphasis = Emphasis::from_jsx_props(&props, "").ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();
        let children = Children(children.unwrap_or_default());
        Ok(TabsGroup { emphasis,
                       sizable,
                       children })
    }
}

impl JsxComponentResult for TabsGroup {
    fn to_jsx_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let mut props = vec![self.emphasis.as_ref().unwrap_or(&Emphasis::Card).to_string()];
        if let Some(sizable) = &self.sizable {
            props.push(sizable.to_jsx_prop())
        }
        Ok(format!(
                   r#"<{} {}>
{}
</{}>"#,
                   EmbeddableComponentName::Tabs,
                   props.join(" "),
                   self.children.render(res)?,
                   EmbeddableComponentName::Tabs,
        ))
    }
}

impl MarkdownComponentResult for TabsGroup {
    fn to_markdown(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        self.children.render(res)
    }
}
