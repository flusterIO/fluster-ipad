use crate::{
    lang::lib::{
        shared::common_unions::number_or_sizable::NumberOrSizable,
        ui::{
            components::layout::grid::grid_columns::GridColumnProps, shared_props::{sizable::SizablePropsGroup, sizable_option::SizableOption}
        },
    },
    parsers::conundrum::logic::{
    },
};
use askama::Template;
use tw_merge::*;

/// ## Template (HTML)
/// ```askama
/// <div class="{{tw_merge!("grid w-full p-4 my-6 grid-cols-1", self.sizable.as_ref().cloned().map(|c| c.as_class()).unwrap_or_default(), self.emphasis_classes.clone(), self.columns.to_css_classes(), self.get_responsive_styles())}}">
/// {{children}}
/// </div>
/// ```
#[derive(Template)]
#[template(ext = "html", in_doc = true)]
pub struct GridHtmlTemplate {
    /// `SizablePropsGroup.as_class`` or an empty string because if statements
    /// in a template are a pain in the axx.
    pub sizable: Option<SizablePropsGroup>,
    /// `Emphasis.to_background_classes` or an empty string.
    pub emphasis_classes: String,
    /// The rendered children
    pub children: String,
    pub responsive: Option<NumberOrSizable>,
    pub fit: bool,
    pub columns: GridColumnProps,
}

impl GridHtmlTemplate {
    pub fn get_responsive_styles(&self) -> String {
        if let Some(responsive) = &self.responsive {
            let sizing = match self.fit {
                true => "auto-fit",
                false => "auto-fill",
            };
            let n = match responsive {
                NumberOrSizable::Number(n) => n.as_float(),
                NumberOrSizable::Sizable(s) => match s {
                    SizableOption::None => 0.0,
                    SizableOption::Small => 120.0,
                    SizableOption::Smedium => 180.0,
                    SizableOption::Medium => 240.0,
                    SizableOption::Large => 320.0,
                    SizableOption::Xl => 480.0,
                    SizableOption::Xxl => 640.0,
                    SizableOption::Fit => 800.0,
                    SizableOption::Full => 1200.0,
                },
            };
            format!("repeat({}, minmax({}px, 1fr))", sizing, n.floor())
        } else {
            String::from("")
        }
    }

    // pub fn get_grid_styles(&self) -> String {}
    pub fn get_container_classes(&self) -> String {
        let mut classes: Vec<&str> = Vec::new();
        // if self.fit && self.responsive {

        // }
        String::from_iter(classes)
    }
}
