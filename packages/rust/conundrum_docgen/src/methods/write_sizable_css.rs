use std::fs;

use askama::Template;
use conundrum::{
    lang::lib::ui::shared_props::sizable_option::SizableOption, output, testing::get_workspace_root::get_workspace_root,
};
use strum_macros::EnumIter;

#[derive(Template)]
#[template(ext = "txt", escape = "none", path = "css/sizable.txt")]
pub struct SizableCssTemplate {
    sizable_options: Vec<SizableOption>,
}

impl SizableCssTemplate {
    pub fn generate() {
        let templ = SizableCssTemplate { sizable_options: SizableOption::all_sizable_options() };
        let res = templ.render().expect("Renders sizable css template without throwing an error.");
        let output_path = std::path::Path::new(&get_workspace_root()).join("packages")
                                                                     .join("rust")
                                                                     .join("conundrum")
                                                                     .join("src")
                                                                     .join("output")
                                                                     .join("html")
                                                                     .join("standalone")
                                                                     .join("conundrum_input_sizable.scss");

        if let Err(err) = fs::write(output_path, res) {
            eprintln!("Error: {:#?}", err);
        }
    }

    pub fn get_classes(&self) -> String {
        let mut classes = Vec::new();
        SizableOption::all_sizable_options().iter().for_each(|size| {
                                                       for n in 1..12 {
                                                           classes.push(size.to_css_column_class(Some(n)));
                                                       }
                                                   });
        classes.join(" ")
    }
}
