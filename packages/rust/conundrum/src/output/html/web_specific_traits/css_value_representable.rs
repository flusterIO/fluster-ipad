use crate::{
    lang::runtime::state::conundrum_error_variant::ConundrumModalResult,
    parsers::conundrum::color::color_pair::ColorPair,
};

pub trait CSSValueRepresentable {
    fn as_css_value(&self) -> String;
}

pub trait CSSValuePossiblyRepresentable {
    fn to_css_value(&self) -> ConundrumModalResult<String>;
}

pub trait CSSInlineHtmlValueRepresentable {
    /// This returns a value that can complete the following template:
    /// ```jinja
    /// style="color: {{value}};"
    /// ```
    fn as_inline_style_value(&self) -> String;
}

pub trait CSSInlineHtmlValuePairRepresentable<T>
    where T: CSSValueRepresentable + Clone {
    /// This returns a value that can complete the following template:
    /// ```jinja
    /// style="color: {{value}};"
    /// ```
    fn as_inline_style_value_group(&self) -> ColorPair<T>;
}

pub trait CSSVariableRepresentable {
    fn to_css_variable(&self) -> String;
}

pub trait CSSVariablePairRepresentable {
    /// Returns the variable alone, without the `var` keyword.
    ///
    /// **Example return:** `--emphasis-warning-foreground`
    fn to_css_variable_group(&self) -> ColorPair<String>;
}
