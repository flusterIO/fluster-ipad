use std::fmt::Display;

use askama::FastWritable;
use serde::Serialize;

use crate::{
    lang::{
        lib::ui::{
            shared_props::sizable_option::SizableOption,
            ui_traits::jsx_prop_representable::{FromJsxPropsOptional, JsxPropRepresentable},
        },
        runtime::state::conundrum_error_variant::ConundrumModalResult,
    },
    parsers::conundrum::logic::{bool::boolean::ConundrumBoolean, object::object::ConundrumObject},
};

/// This is applicable to pretty much any component where changing it's size
/// makes sense. Not so much text, where you're changing the content of the text
/// itself, but rather containers, where the size is changing irrespective of
/// the content inside of it.
/// This shouldn't be confused though, since this struct contains a **lot** more
/// properties than just those that can modify _size._ You can also modify
/// color, padding, margin, borders, and more.
#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
pub struct SizablePropsGroup {
    /// Hides the MathJax labels in all child components.
    pub hide_math_labels: Option<ConundrumBoolean>,
    /// 'Floats' the component to the right. This is often combined with `width`
    /// or the `sidebar` property to create sidebar layouts.
    pub right: Option<ConundrumBoolean>,
    /// 'Floats' the component to the left. This is often combined with `width`
    /// or the `sidebar` property to create sidebar layouts.
    pub left: Option<ConundrumBoolean>,
    /// A utility property that sets a responsive max-width to create sidebar
    /// like layouts on large screens while allowing for full-width when the
    /// window is smaller.
    pub sidebar: Option<ConundrumBoolean>,
    pub center_self: Option<ConundrumBoolean>,
    /// Centers the content of this component's children, not the component
    /// itself.    pub center_self: Option<ConundrumBoolean>,
    pub center_content: Option<ConundrumBoolean>,
    /// Add a small, muted border to the object.
    pub border: Option<ConundrumBoolean>,
    /// Tells the container to not forcefully create a new line and instead to
    /// flow with the rest of the content. Unless you're trying to apply
    /// properties to text inside of a paragraph using the Container component
    /// this is most likely not what you are looking for. If you want text to
    /// wrap around an element, use the `right` or `left` properties paired with
    /// a desired `width`.
    pub inline: Option<ConundrumBoolean>,
    /// Casts an inset shadow from the object.
    pub in_shadow: Option<SizableOption>,
    /// Casts a shadow from the object.
    pub shadow: Option<SizableOption>,
    /// Rounds the corners of the container. Use `rounded=\
    pub rounded: Option<SizableOption>,
    /// Change the text content of the container's children. Beware though, some
    /// edge cases might not respond as expected.
    pub text: Option<SizableOption>,
    /// Set some custom width properties to create responsive layouts.
    pub width: Option<SizableOption>,
    /// Set some custom height properties to create responsive layouts.
    pub height: Option<SizableOption>,
    /// Add some padding around the **outside** of an object. If you are looking
    /// to create space on the _inside_ of an object you are looking for
    /// `padding`.
    pub margin: Option<SizableOption>,
    pub margin_left: Option<SizableOption>,
    pub margin_right: Option<SizableOption>,
    pub margin_top: Option<SizableOption>,
    pub margin_bottom: Option<SizableOption>,
    pub margin_y: Option<SizableOption>,
    pub margin_x: Option<SizableOption>,
    /// Create padding on the _inside_ of an object. If you are trying to create
    /// space _around_ an object. you are probably looking for `margin`.
    pub padding: Option<SizableOption>,
    pub padding_left: Option<SizableOption>,
    pub padding_right: Option<SizableOption>,
    pub padding_top: Option<SizableOption>,
    pub padding_bottom: Option<SizableOption>,
    pub padding_y: Option<SizableOption>,
    pub padding_x: Option<SizableOption>,
    /// When in Grid mode or in some other select layouts, this property create
    /// a gap between _all_ children.
    pub gap: Option<SizableOption>,
    pub gap_y: Option<SizableOption>,
    pub gap_x: Option<SizableOption>,
}

#[allow(clippy::derivable_impls)]
impl Default for SizablePropsGroup {
    fn default() -> Self {
        Self { hide_math_labels: None,
               right: None,
               left: None,
               sidebar: None,
               center_self: None,
               center_content: None,
               border: None,
               inline: None,
               in_shadow: None,
               shadow: None,
               rounded: None,
               text: None,
               width: None,
               height: None,
               margin: None,
               margin_left: None,
               margin_right: None,
               margin_top: None,
               margin_bottom: None,
               margin_y: None,
               margin_x: None,
               padding: None,
               padding_left: None,
               padding_right: None,
               padding_top: None,
               padding_bottom: None,
               padding_y: None,
               padding_x: None,
               gap: None,
               gap_y: None,
               gap_x: None }
    }
}

impl SizablePropsGroup {
    pub fn to_jsx_prop(&self) -> String {
        let items: Vec<Option<String>> = vec![self.hide_math_labels.map(|x| x.to_jsx_prop("hideMathLabels")),
                                              self.right.map(|x| x.to_jsx_prop("right")),
                                              self.left.map(|x| x.to_jsx_prop("left")),
                                              self.sidebar.map(|x| x.to_jsx_prop("sidebar")),
                                              self.center_self.map(|x| x.to_jsx_prop("centerSelf")),
                                              self.center_content.map(|x| x.to_jsx_prop("centerContent")),
                                              self.border.map(|x| x.to_jsx_prop("border")),
                                              self.inline.map(|x| x.to_jsx_prop("inline")),
                                              self.in_shadow.as_ref().map(|x| x.to_jsx_prop("inShadow")),
                                              self.shadow.as_ref().map(|x| x.to_jsx_prop("shadow")),
                                              self.rounded.as_ref().map(|x| x.to_jsx_prop("rounded")),
                                              self.text.as_ref().map(|x| x.to_jsx_prop("text")),
                                              self.width.as_ref().map(|x| x.to_jsx_prop("width")),
                                              self.height.as_ref().map(|x| x.to_jsx_prop("height")),
                                              self.margin.as_ref().map(|x| x.to_jsx_prop("margin")),
                                              self.margin_left.as_ref().map(|x| x.to_jsx_prop("marginLeft")),
                                              self.margin_right.as_ref().map(|x| x.to_jsx_prop("marginRight")),
                                              self.margin_top.as_ref().map(|x| x.to_jsx_prop("marginTop")),
                                              self.margin_bottom.as_ref().map(|x| x.to_jsx_prop("marginBottom")),
                                              self.margin_y.as_ref().map(|x| x.to_jsx_prop("marginY")),
                                              self.margin_x.as_ref().map(|x| x.to_jsx_prop("marginX")),
                                              self.padding.as_ref().map(|x| x.to_jsx_prop("padding")),
                                              self.padding_left.as_ref().map(|x| x.to_jsx_prop("paddingLeft")),
                                              self.padding_right.as_ref().map(|x| x.to_jsx_prop("paddingRight")),
                                              self.padding_top.as_ref().map(|x| x.to_jsx_prop("paddingTop")),
                                              self.padding_bottom.as_ref().map(|x| x.to_jsx_prop("paddingBottom")),
                                              self.padding_y.as_ref().map(|x| x.to_jsx_prop("paddingY")),
                                              self.padding_x.as_ref().map(|x| x.to_jsx_prop("paddingX")),
                                              self.gap.as_ref().map(|x| x.to_jsx_prop("gap")),
                                              self.gap_y.as_ref().map(|x| x.to_jsx_prop("gapY")),
                                              self.gap_x.as_ref().map(|x| x.to_jsx_prop("gapX")),];

        items.iter().filter_map(|c| c.clone()).collect::<Vec<String>>().join(" ")
    }

    pub fn to_css_classes(&self) -> String {
        let mut classes: Vec<&str> = Vec::new();

        if self.hide_math_labels.is_some_and(|x| x.0) {
            classes.push("hide-math-labels");
        }
        if self.right.is_some_and(|x| x.0) {
            classes.push("float-right ml-4 mr-0");
        }
        if self.left.is_some_and(|x| x.0) {
            classes.push("float-left mr-4 ml-0");
        }
        if self.sidebar.is_some_and(|x| x.0) {
            classes.push("w-full min-w-full @[768px]/mdx:w-1/3 @[768px]:min-w-[450px]");
        }
        if self.center_self.is_some_and(|x| x.0) {
            classes.push("mx-auto block");
        }
        if self.center_content.is_some_and(|x| x.0) {
            classes.push("flex flex-col justify-center items-center text-center [&>p]:text-center");
        }
        if self.border.is_some_and(|x| x.0) {
            classes.push("border");
        }
        if self.inline.is_some_and(|x| x.0) {
            classes.push("inline")
        }
        if let Some(in_shadow) = &self.in_shadow {
            classes.push(match in_shadow {
                             SizableOption::None => "inset-shadow-none",
                             SizableOption::Small => "inset-shadow-xxs",
                             SizableOption::Smedium => "inset-shadow-xxs",
                             SizableOption::Medium => "inset-shadow-xs",
                             SizableOption::Large => "inset-shadow-xs",
                             SizableOption::Xl => "inset-shadow-xs",
                             SizableOption::Xxl => "inset-shadow-sm",
                             SizableOption::Full => "inset-shadow-sm",
                             SizableOption::Fit => "inset-shadow-current",
                         });
        }
        if let Some(shadow) = &self.shadow {
            classes.push(match shadow {
                             SizableOption::None => "shadow-none",
                             SizableOption::Small => "shadow-2xs",
                             SizableOption::Smedium => "shadow-sm",
                             SizableOption::Medium => "shadow-md",
                             SizableOption::Large => "shadow-lg",
                             SizableOption::Xl => "shadow-2xl",
                             SizableOption::Xxl => "shadow-2xl",
                             SizableOption::Full => "shadow-2xl",
                             SizableOption::Fit => "shadow-md",
                         });
        }
        if let Some(rounded) = &self.rounded {
            classes.push(match rounded {
                             SizableOption::None => "rounded-none",
                             SizableOption::Small => "rounded-sm",
                             SizableOption::Smedium => "rounded-sm",
                             SizableOption::Medium => "rounded-md",
                             SizableOption::Large => "rounded-lg",
                             SizableOption::Xl => "rounded-2xl",
                             SizableOption::Xxl => "rounded-4xl",
                             SizableOption::Full => "rounded-full",
                             SizableOption::Fit => "rounded-[100%]",
                         });
        }
        if let Some(text) = &self.text {
            classes.push(match text {
                             SizableOption::None => "text-xs",
                             SizableOption::Small => "text-sm",
                             SizableOption::Smedium => "text-base",
                             SizableOption::Medium => "text-lg",
                             SizableOption::Large => "text-xl",
                             SizableOption::Xl => "text-2xl",
                             SizableOption::Xxl => "text-4xl",
                             SizableOption::Full => "text-7xl",
                             SizableOption::Fit => "w-full text-center",
                         });
        }
        if let Some(width) = &self.width {
            classes.push(match width {
                             SizableOption::None => "w-full @[768px]/mdx:hidden",
                             SizableOption::Small => "w-full @[450px]/mdx:w-[320px]",
                             SizableOption::Smedium => "w-full @[550px]/mdx:w-[384px]]",
                             SizableOption::Medium => "w-full @[650px]/mdx:w-[448px]",
                             SizableOption::Large => "w-full @[768px]/mdx:w-[576px]",
                             SizableOption::Xl => "w-full @[1080px]/mdx:w-[672px]",
                             SizableOption::Xxl => "w-full @[1080px]/mdx:w-[896px]",
                             SizableOption::Full => "w-full",
                             SizableOption::Fit => "w-fit",
                         });
        }
        if let Some(height) = &self.height {
            classes.push(match height {
                             SizableOption::None => "h-fit",
                             SizableOption::Small => "h-24",
                             SizableOption::Smedium => "h-32",
                             SizableOption::Medium => "h-48",
                             SizableOption::Large => "h-64",
                             SizableOption::Xl => "h-96",
                             SizableOption::Xxl => "h-screen",
                             SizableOption::Full => "h-full",
                             SizableOption::Fit => "h-fit",
                         });
        }
        if let Some(margin) = &self.margin {
            classes.push(match margin {
                             SizableOption::None => "m-0",
                             SizableOption::Small => "m-2",
                             SizableOption::Smedium => "m-3",
                             SizableOption::Medium => "m-4",
                             SizableOption::Large => "m-6",
                             SizableOption::Xl => "m-8",
                             SizableOption::Xxl => "m-12",
                             SizableOption::Full => "m-16",
                             SizableOption::Fit => "m-0",
                         });
        }
        if let Some(margin_left) = &self.margin_left {
            classes.push(match margin_left {
                             SizableOption::None => "ml-0",
                             SizableOption::Small => "ml-2",
                             SizableOption::Smedium => "ml-3",
                             SizableOption::Medium => "ml-4",
                             SizableOption::Large => "ml-6",
                             SizableOption::Xl => "ml-8",
                             SizableOption::Xxl => "ml-12",
                             SizableOption::Full => "ml-16",
                             SizableOption::Fit => "ml-auto",
                         });
        }
        if let Some(margin_right) = &self.margin_right {
            classes.push(match margin_right {
                             SizableOption::None => "mr-0",
                             SizableOption::Small => "mr-2",
                             SizableOption::Smedium => "mr-3",
                             SizableOption::Medium => "mr-4",
                             SizableOption::Large => "mr-6",
                             SizableOption::Xl => "mr-8",
                             SizableOption::Xxl => "mr-12",
                             SizableOption::Full => "mr-16",
                             SizableOption::Fit => "mr-auto",
                         });
        }
        if let Some(margin_top) = &self.margin_top {
            classes.push(match margin_top {
                             SizableOption::None => "mt-0",
                             SizableOption::Small => "mt-2",
                             SizableOption::Smedium => "mt-3",
                             SizableOption::Medium => "mt-4",
                             SizableOption::Large => "mt-6",
                             SizableOption::Xl => "mt-8",
                             SizableOption::Xxl => "mt-12",
                             SizableOption::Full => "mt-16",
                             SizableOption::Fit => "mt-auto",
                         });
        }
        if let Some(margin_bottom) = &self.margin_bottom {
            classes.push(match margin_bottom {
                             SizableOption::None => "h-fit",
                             SizableOption::Small => "h-24",
                             SizableOption::Smedium => "h-32",
                             SizableOption::Medium => "h-48",
                             SizableOption::Large => "h-64",
                             SizableOption::Xl => "h-96",
                             SizableOption::Xxl => "h-screen",
                             SizableOption::Full => "h-full",
                             SizableOption::Fit => "h-fit",
                         });
        }
        if let Some(margin_y) = &self.margin_y {
            classes.push(match margin_y {
                             SizableOption::None => "my-0",
                             SizableOption::Small => "my-2",
                             SizableOption::Smedium => "my-3",
                             SizableOption::Medium => "my-4",
                             SizableOption::Large => "my-6",
                             SizableOption::Xl => "my-8",
                             SizableOption::Xxl => "my-12",
                             SizableOption::Full => "my-16",
                             SizableOption::Fit => "my-auto",
                         });
        }
        if let Some(margin_x) = &self.margin_x {
            classes.push(match margin_x {
                             SizableOption::None => "mx-0",
                             SizableOption::Small => "mx-2",
                             SizableOption::Smedium => "mx-3",
                             SizableOption::Medium => "mx-4",
                             SizableOption::Large => "mx-6",
                             SizableOption::Xl => "mx-8",
                             SizableOption::Xxl => "mx-12",
                             SizableOption::Full => "mx-16",
                             SizableOption::Fit => "mx-auto",
                         });
        }
        if let Some(padding) = &self.padding {
            classes.push(match padding {
                             SizableOption::None => "p-0",
                             SizableOption::Small => "p-2",
                             SizableOption::Smedium => "p-3",
                             SizableOption::Medium => "p-4",
                             SizableOption::Large => "p-6",
                             SizableOption::Xl => "p-8",
                             SizableOption::Xxl => "p-12",
                             SizableOption::Full => "p-16",
                             SizableOption::Fit => "p-0",
                         });
        }
        if let Some(padding_left) = &self.padding_left {
            classes.push(match padding_left {
                             SizableOption::None => "pl-0",
                             SizableOption::Small => "pl-2",
                             SizableOption::Smedium => "pl-3",
                             SizableOption::Medium => "pl-4",
                             SizableOption::Large => "pl-6",
                             SizableOption::Xl => "pl-8",
                             SizableOption::Xxl => "pl-12",
                             SizableOption::Full => "pl-16",
                             SizableOption::Fit => "pl-0",
                         });
        }
        if let Some(padding_right) = &self.padding_right {
            classes.push(match padding_right {
                             SizableOption::None => "pr-0",
                             SizableOption::Small => "pr-2",
                             SizableOption::Smedium => "pr-3",
                             SizableOption::Medium => "pr-4",
                             SizableOption::Large => "pr-6",
                             SizableOption::Xl => "pr-8",
                             SizableOption::Xxl => "pr-12",
                             SizableOption::Full => "pr-16",
                             SizableOption::Fit => "pr-0",
                         });
        }
        if let Some(padding_top) = &self.padding_top {
            classes.push(match padding_top {
                             SizableOption::None => "pt-0",
                             SizableOption::Small => "pt-2",
                             SizableOption::Smedium => "pt-3",
                             SizableOption::Medium => "pt-4",
                             SizableOption::Large => "pt-6",
                             SizableOption::Xl => "pt-8",
                             SizableOption::Xxl => "pt-12",
                             SizableOption::Full => "pt-16",
                             SizableOption::Fit => "pt-0",
                         });
        }
        if let Some(padding_bottom) = &self.padding_bottom {
            classes.push(match padding_bottom {
                             SizableOption::None => "pb-0",
                             SizableOption::Small => "pb-2",
                             SizableOption::Smedium => "pb-3",
                             SizableOption::Medium => "pb-4",
                             SizableOption::Large => "pb-6",
                             SizableOption::Xl => "pb-8",
                             SizableOption::Xxl => "pb-12",
                             SizableOption::Full => "pb-16",
                             SizableOption::Fit => "pb-0",
                         });
        }
        if let Some(padding_y) = &self.padding_y {
            classes.push(match padding_y {
                             SizableOption::None => "py-0",
                             SizableOption::Small => "py-2",
                             SizableOption::Smedium => "py-3",
                             SizableOption::Medium => "py-4",
                             SizableOption::Large => "py-6",
                             SizableOption::Xl => "py-8",
                             SizableOption::Xxl => "py-12",
                             SizableOption::Full => "py-16",
                             SizableOption::Fit => "py-0",
                         });
        }
        if let Some(padding_x) = &self.padding_x {
            classes.push(match padding_x {
                             SizableOption::None => "px-0",
                             SizableOption::Small => "px-2",
                             SizableOption::Smedium => "px-3",
                             SizableOption::Medium => "px-4",
                             SizableOption::Large => "px-6",
                             SizableOption::Xl => "px-8",
                             SizableOption::Xxl => "px-12",
                             SizableOption::Full => "px-16",
                             SizableOption::Fit => "px-0",
                         });
        }
        if let Some(gap) = &self.gap {
            classes.push(match gap {
                             SizableOption::None => "gap-0",
                             SizableOption::Small => "gap-2",
                             SizableOption::Smedium => "gap-3",
                             SizableOption::Medium => "gap-4",
                             SizableOption::Large => "gap-6",
                             SizableOption::Xl => "gap-8",
                             SizableOption::Xxl => "gap-12",
                             SizableOption::Full => "gap-16",
                             SizableOption::Fit => "gap-0",
                         });
        }
        if let Some(gap_y) = &self.gap_y {
            classes.push(match gap_y {
                             SizableOption::None => "gap-y-0",
                             SizableOption::Small => "gap-y-2",
                             SizableOption::Smedium => "gap-y-3",
                             SizableOption::Medium => "gap-y-4",
                             SizableOption::Large => "gap-y-6",
                             SizableOption::Xl => "gap-y-8",
                             SizableOption::Xxl => "gap-y-12",
                             SizableOption::Full => "gap-y-16",
                             SizableOption::Fit => "gap-0",
                         });
        }
        if let Some(gap_x) = &self.gap_x {
            classes.push(match gap_x {
                             SizableOption::None => "gap-x-0",
                             SizableOption::Small => "gap-x-2",
                             SizableOption::Smedium => "gap-x-3",
                             SizableOption::Medium => "gap-x-4",
                             SizableOption::Large => "gap-x-6",
                             SizableOption::Xl => "gap-x-8",
                             SizableOption::Xxl => "gap-x-12",
                             SizableOption::Full => "gap-x-16",
                             SizableOption::Fit => "gap-x-0",
                         });
        }

        classes.join(" ")
    }
}

impl FromJsxPropsOptional for SizablePropsGroup {
    /// The 'key' property is completely irrelevant here has the
    /// SizablePropsGroup represents many propertieis.
    fn from_jsx_props(props: &ConundrumObject, _: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        Ok(SizablePropsGroup { hide_math_labels: props.get_boolean("hideMathLabels", None).ok(),
                               right: props.get_boolean("right", None).ok(),
                               left: props.get_boolean("left", None).ok(),
                               sidebar: props.get_boolean("sidebar", None).ok(),
                               center_self: props.get_boolean("centerSelf", None).ok(),
                               center_content: props.get_boolean("centerContent", None).ok(),
                               border: props.get_boolean("border", None).ok(),
                               inline: props.get_boolean("inline", None).ok(),
                               in_shadow: props.get_sizable_option_at_key("inShadow").ok(),
                               shadow: props.get_sizable_option_at_key("shadow").ok(),
                               rounded: props.get_sizable_option_at_key("rounded").ok(),
                               text: props.get_sizable_option_at_key("text").ok(),
                               width: props.get_sizable_option_at_key("width").ok(),
                               height: props.get_sizable_option_at_key("height").ok(),
                               margin: props.get_sizable_option_at_key("margin").ok(),
                               margin_left: props.get_sizable_option_at_key("marginLeft").ok(),
                               margin_right: props.get_sizable_option_at_key("marginRight").ok(),
                               margin_top: props.get_sizable_option_at_key("marginTop").ok(),
                               margin_bottom: props.get_sizable_option_at_key("marginBottom").ok(),
                               margin_y: props.get_sizable_option_at_key("marginY").ok(),
                               margin_x: props.get_sizable_option_at_key("marginX").ok(),
                               padding: props.get_sizable_option_at_key("padding").ok(),
                               padding_left: props.get_sizable_option_at_key("paddingLeft").ok(),
                               padding_right: props.get_sizable_option_at_key("paddingRight").ok(),
                               padding_top: props.get_sizable_option_at_key("paddingTop").ok(),
                               padding_bottom: props.get_sizable_option_at_key("paddingBottom").ok(),
                               padding_y: props.get_sizable_option_at_key("paddingY").ok(),
                               padding_x: props.get_sizable_option_at_key("paddingX").ok(),
                               gap: props.get_sizable_option_at_key("gap").ok(),
                               gap_y: props.get_sizable_option_at_key("gapY").ok(),
                               gap_x: props.get_sizable_option_at_key("gapX").ok() })
    }
}

impl Display for SizablePropsGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_css_classes())
    }
}

impl FastWritable for SizablePropsGroup {
    fn write_into<W: core::fmt::Write + ?Sized>(&self,
                                                dest: &mut W,
                                                values: &dyn askama::Values)
                                                -> askama::Result<()> {
        self.to_css_classes().as_str().write_into(dest, values)
    }
}
