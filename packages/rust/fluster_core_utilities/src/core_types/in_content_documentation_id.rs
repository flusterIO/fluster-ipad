use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use typeshare::typeshare;

#[typeshare]
#[derive(strum_macros::Display, Clone, EnumIter, Serialize, Deserialize)]
pub enum InContentDocumentationSource {
    #[serde(rename = "component")]
    #[strum(to_string = "component")]
    ComponentDocs,
    #[serde(rename = "internal-docs")]
    #[strum(to_string = "internal-docs")]
    InternalDocs,
}

#[typeshare]
#[derive(strum_macros::Display, Clone, EnumIter, Serialize, Deserialize)]
pub enum InContentDocumentationFormat {
    #[serde(rename = "full")]
    #[strum(to_string = "full")]
    Full,
    #[serde(rename = "short")]
    #[strum(to_string = "short")]
    Short,
}

#[typeshare]
#[derive(strum_macros::Display, EnumIter, Serialize, Deserialize)]
pub enum InContentDocumentationId {
    #[serde(rename = "Markdown")]
    #[strum(to_string = "Markdown")]
    Markdown,
    #[serde(rename = "Docs")]
    #[strum(to_string = "Docs")]
    Docs,
}

impl InContentDocumentationId {
    pub fn to_embedded_file_name(self, format: InContentDocumentationFormat) -> String {
        let base_file_name = match self {
            InContentDocumentationId::Markdown => "markdown",
            InContentDocumentationId::Docs => "documentation-docs",
        };
        format!("{}-{}.mdx", base_file_name, format)
    }
}

/// Returns a list of file names, not the complete file path and completely ignores directories.
pub fn get_file_names_in_dir(dir_path: &PathBuf) -> std::io::Result<Vec<String>> {
    let mut file_names = Vec::new();

    // Read the directory entries, handling potential IO errors
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?; // Propagate the individual entry's IO error

        let path = entry.path();

        // Check if the entry is a file and not a directory
        if path.is_file() {
            // Extract the file name as an OsStr
            if let Some(file_name_os) = path.file_name() {
                // Convert OsStr to a String, handling potential non-UTF-8 names
                if let Some(file_name_str) = file_name_os.to_str() {
                    file_names.push(file_name_str.to_owned());
                }
            }
        }
    }

    Ok(file_names)
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::test_utilities::get_workspace_root::{self, get_workspace_root};

    use super::*;

    #[test]
    fn all_in_content_documentation_exists() {
        let root = get_workspace_root();
        let p = std::path::Path::new(&root)
            .join("packages")
            .join("rust")
            .join("fluster_pre_parser")
            .join("src")
            .join("embedded")
            .join("in_content_docs");
        let file_names = get_file_names_in_dir(&p)
            .expect("Reads 'in_content' notes directory without throwing an error.");
        for doc_format in InContentDocumentationFormat::iter() {
            for id in InContentDocumentationId::iter() {
                let file_name_should_exist = id.to_embedded_file_name(doc_format.clone());
                assert!(
                    file_names.iter().any(|x| x == &file_name_should_exist),
                    "The {} does not appear to exist.",
                    file_name_should_exist
                )
            }
        }

        // assert_eq!(result, 4);
    }
}
