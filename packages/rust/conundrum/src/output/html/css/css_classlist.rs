use std::fmt::Display;

use askama::FastWritable;
pub struct CSSClassList(Vec<String>);

impl CSSClassList {
    pub fn new(initial_classes: Vec<String>) -> Self {
        CSSClassList(initial_classes)
    }

    pub fn append(&mut self, item: String) {
        if !self.contains(&item) {
            self.0.push(item);
        }
    }

    pub fn contains(&mut self, item: &str) -> bool {
        self.0.iter().any(|x| x.as_str() == item)
    }
}

impl Display for CSSClassList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0.clone().into_iter().collect::<Vec<String>>().join(" ");
        write!(f, "{}", s)
    }
}

impl FastWritable for CSSClassList {
    fn write_into<W: core::fmt::Write + ?Sized>(&self,
                                                dest: &mut W,
                                                values: &dyn askama::Values)
                                                -> askama::Result<()> {
        let css_string = self.0.clone().into_iter().collect::<Vec<String>>().join(" ");
        css_string.write_into(dest, values)
    }
}
