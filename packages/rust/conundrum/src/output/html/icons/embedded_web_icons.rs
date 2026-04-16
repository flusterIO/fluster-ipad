use std::fmt::Display;

use askama::FastWritable;

pub enum EmbeddedIcon {
    Copy,
}

impl EmbeddedIcon {
    pub fn to_web_svg(&self) -> String {
        match self {
            EmbeddedIcon::Copy => "FIX ME".to_string(),
        }
    }
}

impl Display for EmbeddedIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_web_svg())
    }
}

impl FastWritable for EmbeddedIcon {
    fn write_into<W: core::fmt::Write + ?Sized>(&self,
                                                dest: &mut W,
                                                values: &dyn askama::Values)
                                                -> askama::Result<()> {
        self.to_web_svg().as_str().write_into(dest, values)
    }
}
