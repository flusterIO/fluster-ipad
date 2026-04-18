use askama::Template;
use std::{fs, path::Path};

#[derive(Template)]
#[template(ext = "jinja", path = "test_sample_page.jinja")]
pub struct TestRenderPage {
    pub label: String,
    pub content: String,
    pub css_content: String,
    pub repo_root: String,
}

impl TestRenderPage {
    pub fn new(content: &str, label: &str) -> Self {
        Self { content: content.to_string(),
               label: label.to_string(),
               css_content: TestRenderPage::get_conundrum_css(),
               repo_root: env!("FLUSTER_IOS_ROOT").to_string() }
    }

    pub fn get_conundrum_css() -> String {
        let p = Path::new(env!("FLUSTER_IOS_ROOT"));
        let x = p.join("packages").join("rust").join("conundrum").join("generated").join("web").join("conundrum.css");
        fs::read_to_string(x).expect("Reads conundrum.css during testinging...")
    }

    pub fn write_output(&self) {
        let p = Path::new(env!("FLUSTER_IOS_ROOT"));
        let x = p.join("packages").join("rust").join("conundrum").join("test_output.html");

        let content = self.render().expect("Failed to render");
        fs::write(x, content).expect("Failed to write test output");
    }

    /// Because I'm two lazy to call lazy and then run separately...
    pub fn new_run(content: &str, label: &str) {
        let s = TestRenderPage::new(content, label);
        s.write_output();
    }
}
