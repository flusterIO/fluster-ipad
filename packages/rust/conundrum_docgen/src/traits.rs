use core::panic;

use askama::Template;

use crate::{
    errors::{DocGenError, DocGenResult},
    workspace_utils::get_workspace_root_duplicate::get_workspace_root,
};

pub trait DocGenTemplate: Template {
    fn descriptive_label() -> String;
    fn gather_data() -> Self;
    fn generate(&self, repo_root_relative_path: String) -> DocGenResult<()> {
        let res = self.render().map_err(|e| {
                                    eprintln!("Error: {:#?}", e);
                                    DocGenError::TemplateRenderError(Self::descriptive_label())
                                })?;
        let output_path = std::path::Path::new(&get_workspace_root()).join(repo_root_relative_path);
        std::fs::write(output_path.clone(), res).inspect_err(|_| {
                                                    panic!("Failed to write the {} output to the proper path at {:?}",
                                                           Self::descriptive_label(),
                                                           output_path)
                                                });

        Ok(())
    }
}
