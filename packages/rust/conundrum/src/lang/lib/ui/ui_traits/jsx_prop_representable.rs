use crate::{
    lang::runtime::state::conundrum_error_variant::{ConundrumModalResult, ConundrumResult},
    parsers::conundrum::logic::object::object::ConundrumObject,
};

pub trait JsxPropRepresentable {
    fn to_jsx_prop(&self, key: &str) -> String;
}

pub trait CssStyleRepresentable {
    /// To a string that can be embedded in an html style string
    fn to_css_style(&self, key: &str) -> String;
}

/// Returns a _property_ type, such as a HeadingDepth by reading data from a
/// JavascriptObjectResult if it exists, and throws an error if it does not.
pub trait FromJsxPropsOptional {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized;
}
