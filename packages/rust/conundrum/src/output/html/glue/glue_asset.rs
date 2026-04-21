pub trait GlueCssAsset {
    fn append_to_css(&self, content: &mut String, standalone: &bool);
}

pub trait GlueJsAsset {
    fn append_to_js(&self, content: &mut String, standalone: &bool);
}

// /// A GlueAsset is a wrapper around an embedded path in the JS_GLUE_MAP or
// /// whatever it's called.
// pub struct GlueAsset(AnyComponentKey);

// impl GlueAsset {
//     pub fn read(&self) -> String {
//         JS_GLUE_CODE_MAP::get_by_key(self.0.clone()).expect("All glue assets
// must be available and well tested.")     }
// }
