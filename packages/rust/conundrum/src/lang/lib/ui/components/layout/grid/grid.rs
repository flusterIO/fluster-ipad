use crate::{
    lang::{
        elements::parsed_elements::ParsedElement,
        lib::{
            shared::common_unions::number_or_sizable::NumberOrSizable,
            ui::{
                components::{
                    component_trait::ConundrumComponent,
                    layout::grid::{grid_columns::GridColumnProps, grid_html_template::GridHtmlTemplate},
                },
                shared_props::sizable::SizablePropsGroup,
                ui_traits::jsx_prop_representable::FromJsxPropsOptional,
                ui_types::{children::Children, emphasis::Emphasis},
            },
        },
        runtime::{
            state::{
                conundrum_error::ConundrumError,
                conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
            },
            traits::{
                conundrum_input::ArcState, fluster_component_result::ConundrumComponentResult,
                html_js_component_result::HtmlJsComponentResult, jsx_component_result::JsxComponentResult,
                markdown_component_result::MarkdownComponentResult, mdx_component_result::MdxComponentResult,
                plain_text_component_result::PlainTextComponentResult,
            },
        },
    },
    output::general::component_constants::{
        any_component_id::AnyComponentName, component_names::EmbeddableComponentName,
    },
    parsers::conundrum::logic::{bool::boolean::ConundrumBoolean, object::object::ConundrumObject},
};
use askama::Template;
use serde::Serialize;
use std::sync::Arc;
use winnow::error::ErrMode;

/// ## Example Usage
///
/// ```tsx
/// <Grid small={1} large={2}>
/// <Container>
/// My item one
/// </Container>
/// <Container>
/// My item one
/// </Container>
/// </Grid>
/// ```
/// Or if you don't need responsiveness:
/// ```tsx
/// <Grid columns={2}>
/// <Container>
/// My item one
/// </Container>
/// <Container>
/// My item one
/// </Container>
/// </Grid>
/// ```
#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct ResponsiveGrid {
    pub sizable: Option<SizablePropsGroup>,
    pub columns: GridColumnProps,
    pub children: Children,
    pub responsive: Option<NumberOrSizable>,
    pub fit: Option<ConundrumBoolean>,
    pub emphasis: Option<Emphasis>,
}

impl HtmlJsComponentResult for ResponsiveGrid {
    fn to_html_js_component(&self, res: ArcState) -> ConundrumModalResult<String> {
        let templ = GridHtmlTemplate { children: self.children.render(Arc::clone(&res))?,
                                       columns: self.columns.clone(),
                                       sizable: self.sizable.clone(),
                                       responsive: self.responsive.clone(),
                                       fit: self.fit.is_some_and(|x| x.0),
                                       emphasis_classes: self.emphasis
                                                             .as_ref()
                                                             .cloned()
                                                             .map(|e| e.to_background_color_classes())
                                                             .unwrap_or_default() };
        templ.render().map_err(|e| {
                    ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::general_render_error()))
                })
    }
}

impl MarkdownComponentResult for ResponsiveGrid {
    fn to_markdown(&self,
                   res: ArcState)
                   -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl PlainTextComponentResult for ResponsiveGrid {
    fn to_plain_text(&self,
                     res: ArcState)
                     -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.children.render(res)
    }
}

impl JsxComponentResult for ResponsiveGrid {
    fn to_jsx_component(&self,
                        res: ArcState)
                        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        let props: Vec<String> = vec![];
        let children_string = self.children.render(Arc::clone(&res))?;
        Ok(format!(
                   "<{} {}>
{}
</{}>",
                   EmbeddableComponentName::Grid,
                   props.join(" "),
                   children_string,
                   EmbeddableComponentName::Grid
        ))
    }
}

impl MdxComponentResult for ResponsiveGrid {
    fn to_mdx_component(&self,
                        res: ArcState)
                        -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String> {
        self.to_jsx_component(res)
    }
}

impl ConundrumComponentResult for ResponsiveGrid {
    fn to_conundrum_component(&self,
                              res: ArcState)
                              -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<String>
    {
        let state = res.read_arc();
        if state.is_markdown_or_search_or_ai() {
            drop(state);
            self.to_markdown(res)
        } else {
            drop(state);
            self.to_mdx_component(res)
        }
    }
}

impl ConundrumComponent for ResponsiveGrid {
    fn get_component_id() -> AnyComponentName {
        AnyComponentName::UserEmbedded(EmbeddableComponentName::Grid)
    }

    fn from_props(props: ConundrumObject,
                  children: Option<Vec<ParsedElement>>,
                  _: ArcState)
                  -> ConundrumModalResult<Self>
        where Self: Sized {
        let sizable = SizablePropsGroup::from_jsx_props(&props, "").ok();
        let children = Children(children.unwrap_or_default());
        let columns = GridColumnProps::from_jsx_props(&props, "")?;
        let responsive = NumberOrSizable::from_jsx_props(&props, "responsive").ok();
        let fit = ConundrumBoolean::from_jsx_props(&props, "fit").ok();
        let emphasis = Emphasis::from_jsx_props(&props, "").ok();
        Ok(ResponsiveGrid { sizable,
                            columns,
                            responsive,
                            fit,
                            emphasis,
                            children })
    }
}
