use askama::Template;
use winnow::error::ErrMode;

use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::ui::{
            components::{
                component_trait::ConundrumComponent,
                layout::tabs::{
                    tab_group_html_template::TabGroupHtmlTemplate,
                    tab_html_template::{TabButtonHtmlTemplate, TabContentTemplate},
                },
            },
            shared_props::sizable::SizablePropsGroup,
            ui_traits::jsx_prop_representable::FromJsxPropsOptional,
            ui_types::{children::Children, emphasis::Emphasis},
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
                parse_state::{ConundrumCompileTarget, ConundrumModifier, ParseState},
            },
            traits::{
                fluster_component_result::ConundrumComponentResult, html_js_component_result::HtmlJsComponentResult,
                jsx_component_result::JsxComponentResult, markdown_component_result::MarkdownComponentResult,
                mdx_component_result::MdxComponentResult, plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::{
        general::component_constants::{any_component_id::AnyComponentName, component_names::EmbeddableComponentName},
        html::dom::dom_id::DOMId,
    },
    parsers::{
        conundrum::logic::{bool::boolean::ConundrumBoolean, object::object::ConundrumObject},
        react::conundrum_component::ConundrumComponentType,
    },
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
    pub id: DOMId,
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

impl HtmlJsComponentResult for TabsGroup {
    fn to_html_js_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        let group_id = self.id.to_string();
        let group = TabGroupHtmlTemplate { children: self.children.render(res)?,
                                           tab_group_id: group_id.clone(),
                                           tabs: self.get_tabs(&group_id, res)? };

        group.render().map_err(|e| {
            eprintln!("Error: {:#?}", e);
            ErrMode::Cut(
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Compiler error", "Well this is new, usually the errors are on the parser side of things. If this continues, please file an issue on [Github](https://github.com/flusterIO) so I can resolve it immediately."))
                )
        })
    }
}

impl ConundrumComponentResult for TabsGroup {
    fn to_conundrum_component(&self, res: &mut ParseState) -> ConundrumModalResult<String> {
        if res.targets_markdown() {
            self.to_markdown(res)
        } else if res.compile_target == ConundrumCompileTarget::PlainText {
            self.to_plain_text(res)
        } else {
            self.to_mdx_component(res)
        }
    }
}

impl TabsGroup {
    pub fn get_tabs(&self, group_id: &str, res: &mut ParseState) -> ConundrumModalResult<Vec<TabButtonHtmlTemplate>> {
        let mut children = Vec::new();
        for (i, x) in self.children.0.iter().enumerate() {
            if let Some(tab) = match x {
                ParsedElement::ReactComponentWithChildren(n) => match &n.component {
                    ConundrumComponentType::Tab(t) => {
                        Some(TabButtonHtmlTemplate { btn_children: t.label.to_children(res.clone())?.render(res)?,
                                                     tab_group_id: group_id.to_string(),
                                                     idx: i as u8,
                                                     initial: t.initial.unwrap_or(ConundrumBoolean(false)),
                                                     tab_children: TabContentTemplate { idx: i as u8,
                                                                                        group_id:
                                                                                            group_id.to_string(),
                                                                                        content: t.children
                                                                                                  .render(res)? } })
                    }
                    _ => None,
                },
                _ => None,
            } {
                children.push(tab);
            } else {
                eprintln!("Found a component that is not a tab. Should definitely notify the user.")
            }
        }
        Ok(children)
    }
}

impl ConundrumComponent for TabsGroup {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Tabs)
    }

    /// Pass in state here so it can be modified, and then clean up this file
    /// before rendering shit directly to html all day tommorow. The
    /// feedback loop is pretty tight so it should be a good experience.
    fn from_props(props: ConundrumObject, children: Option<Vec<ParsedElement>>) -> ConundrumModalResult<Self> {
        let emphasis = Emphasis::from_jsx_props(&props, "").ok();
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();
        let children = Children(children.unwrap_or_default());
        let id = DOMId::new("my-temp-broken-id".to_string());
        Ok(TabsGroup { emphasis,
                       sizable,
                       children,
                       id })
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
