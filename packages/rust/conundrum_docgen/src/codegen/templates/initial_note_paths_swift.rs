use askama::Template;

use crate::{traits::DocGenTemplate, workspace_utils::get_workspace_root_duplicate::get_workspace_root};

#[derive(Template)]
#[template(path = "swift/get_initial_note_paths.txt", ext = "jinja")]
pub struct InitialNotePathsSwift {
    paths: Vec<String>,
}

impl DocGenTemplate for InitialNotePathsSwift {
    fn descriptive_label() -> String {
        String::from("Intial note paths")
    }

    fn gather_data() -> Self {
        let root = get_workspace_root();
        let dir_path = std::path::Path::new(&root).join("docs").join("seed");
        let dir_path_string = dir_path.to_str().expect("COnverts docs dir to string").to_string();

        let files = dir_path.read_dir().expect("Reads documentation directory without throwing an error.");
        let mut paths = Vec::new();
        for f in files {
            let p = f.expect("Unwraps dir entry").path();
            let r = p.to_str().expect("Unwraps directory path.");
            if r.ends_with(".cdrm") {
                let s = r.trim_prefix(&dir_path_string).trim_suffix(".cdrm");
                paths.push(s.to_string());
            }
        }

        Self { paths }
    }
}
