use std::fmt::Display;

use askama::FastWritable;
use serde::{Deserialize, Serialize};

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
use conundrum_macro_traits::conundrum_macro::ConundrumMacroProperty;
use conundrum_macros::ConundrumMacro;

pub enum SizablePropsOutputTarget {
    Image,
    /// For images nested in another SizableProps container that is pulling
    /// properties from the same component, like the captioned image.
    ImageNested,
    General,
}

/// This is applicable to pretty much any component where changing it's size
/// makes sense. Not so much text, where you're changing the content of the text
/// itself, but rather containers, where the size is changing irrespective of
/// the content inside of it.
/// This shouldn't be confused though, since this struct contains a **lot** more
/// properties than just those that can modify _size._ You can also modify
/// color, padding, margin, borders, and more.
#[typeshare::typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, ConundrumMacro)]
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

    /// The `max-width` and `max-height``properties are especially useful for
    /// images and media.`
    pub max_width: Option<SizableOption>,
    /// The `max-width` and `max-height``properties are especially useful for
    /// images and media.`
    pub max_height: Option<SizableOption>,
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
               max_width: None,
               max_height: None,
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
        let mut properties = String::from("");
        if let Some(res) = &self.hide_math_labels.as_ref().cloned().map(|n| n.as_cdrm_property("#field_name")).flatten()
        {
            properties += res;
        }
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
                                              self.max_width.as_ref().map(|x| x.to_jsx_prop("maxWidth")),
                                              self.max_height.as_ref().map(|x| x.to_jsx_prop("maxHeight")),
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

    pub fn to_css_classes(&self, output: SizablePropsOutputTarget) -> Vec<String> {
        let mut classes: Vec<String> = Vec::new();
        let nested_image_classes = {
            if self.max_width.is_some() {
                String::from("w-fit max-w-full h-auto")
            } else if self.max_height.is_some() {
                String::from("h-fit max-h-full w-auto")
            } else {
                String::from("max-w-full max-h-full")
            }
        };

        if self.hide_math_labels.is_some_and(|x| x.0) {
            classes.push("hide-math-labels".to_string());
        }
        // TODO: Accept a Sizable property as the right and left key to move this
        // `768px` breakpoint.
        if self.right.is_some_and(|x| x.0) {
            match output {
                SizablePropsOutputTarget::Image => {
                    classes.push("@[768px]/mdx:float-right @[768px]/mdx:ml-4 mr-0".to_string());
                }
                SizablePropsOutputTarget::ImageNested => {
                    classes.push(nested_image_classes.clone());
                }
                SizablePropsOutputTarget::General => {
                    classes.push("float-right ml-4 mr-0".to_string());
                }
            }
        }
        if self.left.is_some_and(|x| x.0) {
            match output {
                SizablePropsOutputTarget::Image => {
                    classes.push("@[768px]/mdx:float-left @[768px]/mdx:mr-4 ml-0".to_string());
                }
                SizablePropsOutputTarget::ImageNested => {
                    classes.push(nested_image_classes.clone());
                }
                SizablePropsOutputTarget::General => {
                    classes.push("float-left mr-4 ml-0".to_string());
                }
            }
        }
        if self.sidebar.is_some_and(|x| x.0) {
            match output {
                SizablePropsOutputTarget::General => {
                    classes.push("w-full min-w-full @[768px]/mdx:w-1/3 @[768px]:min-w-[450px]".to_string());
                }
                SizablePropsOutputTarget::ImageNested => {
                    classes.push(nested_image_classes.clone());
                }
                SizablePropsOutputTarget::Image => {
                    classes.push("w-full min-w-full object-contain @[768px]/mdx:w-1/3 @[768px]:min-w-[450px]".to_string());
                }
            }
        }
        if self.center_self.is_some_and(|x| x.0) {
            classes.push("mx-auto block place-self-center".to_string());
        }
        if self.center_content.is_some_and(|x| x.0) {
            classes.push("flex flex-col justify-center items-center text-center [&>p]:text-center".to_string());
        }
        if self.border.is_some_and(|x| x.0) {
            classes.push("border".to_string());
        }
        if self.inline.is_some_and(|x| x.0) {
            classes.push("inline-block".to_string());
        }
        if let Some(in_shadow) = &self.in_shadow {
            classes.push(match in_shadow {
                             SizableOption::None => "inset-shadow-none".to_string(),
                             SizableOption::Small => "inset-shadow-xxs".to_string(),
                             SizableOption::Smedium => "inset-shadow-xxs".to_string(),
                             SizableOption::Medium => "inset-shadow-xs".to_string(),
                             SizableOption::Large => "inset-shadow-xs".to_string(),
                             SizableOption::Xl => "inset-shadow-xs".to_string(),
                             SizableOption::Xxl => "inset-shadow-sm".to_string(),
                             SizableOption::Full => "inset-shadow-sm".to_string(),
                             SizableOption::Fit => "inset-shadow-current".to_string(),
                         });
        }
        if let Some(shadow) = &self.shadow {
            classes.push(match shadow {
                             SizableOption::None => "shadow-none".to_string(),
                             SizableOption::Small => "shadow-2xs".to_string(),
                             SizableOption::Smedium => "shadow-sm".to_string(),
                             SizableOption::Medium => "shadow-md".to_string(),
                             SizableOption::Large => "shadow-lg".to_string(),
                             SizableOption::Xl => "shadow-2xl".to_string(),
                             SizableOption::Xxl => "shadow-2xl".to_string(),
                             SizableOption::Full => "shadow-2xl".to_string(),
                             SizableOption::Fit => "shadow-md".to_string(),
                         });
        }
        if let Some(rounded) = &self.rounded {
            classes.push(match rounded {
                             SizableOption::None => "rounded-none".to_string(),
                             SizableOption::Small => "rounded-sm".to_string(),
                             SizableOption::Smedium => "rounded-sm".to_string(),
                             SizableOption::Medium => "rounded-md".to_string(),
                             SizableOption::Large => "rounded-lg".to_string(),
                             SizableOption::Xl => "rounded-2xl".to_string(),
                             SizableOption::Xxl => "rounded-4xl".to_string(),
                             SizableOption::Full => "rounded-full".to_string(),
                             SizableOption::Fit => "rounded-[100%]".to_string(),
                         });
        }
        if let Some(text) = &self.text {
            classes.push(match text {
                             SizableOption::None => "text-xs".to_string(),
                             SizableOption::Small => "text-sm".to_string(),
                             SizableOption::Smedium => "text-base".to_string(),
                             SizableOption::Medium => "text-lg".to_string(),
                             SizableOption::Large => "text-xl".to_string(),
                             SizableOption::Xl => "text-2xl".to_string(),
                             SizableOption::Xxl => "text-4xl".to_string(),
                             SizableOption::Full => "text-7xl".to_string(),
                             SizableOption::Fit => "w-full text-center".to_string(),
                         });
        }
        if let Some(width) = &self.width {
            match output {
                SizablePropsOutputTarget::Image => {
                    classes.push(match width {
                                     SizableOption::None => {
                                         "w-full @[768px]/mdx:hidden h-auto max-h-[min(768px,90vh)]".to_string()
                                     }
                                     SizableOption::Small => {
                                         "max-w-full @[450px]/mdx:w-[320px] h-auto max-h-[min(768px,90vh)]".to_string()
                                     }
                                     SizableOption::Smedium => {
                                         "max-w-full @[550px]/mdx:w-[384px] h-auto max-h-[min(768px,90vh)]".to_string()
                                     }
                                     SizableOption::Medium => {
                                         "max-w-full @[650px]/mdx:w-[448px] h-auto max-h-[min(768px,90vh)]".to_string()
                                     }
                                     SizableOption::Large => {
                                         "max-w-full @[768px]/mdx:w-[576px] h-auto max-h-[min(768px,90vh)]".to_string()
                                     }
                                     SizableOption::Xl => {
                                         "max-w-full @[1080px]/mdx:w-[672px] h-auto max-h-[min(768px,90vh)]".to_string()
                                     }
                                     SizableOption::Xxl => {
                                         "max-w-full @[1080px]/mdx:w-[896px] h-auto max-h-[min(768px,90vh)]".to_string()
                                     }
                                     SizableOption::Full => "w-full".to_string(),
                                     SizableOption::Fit => "w-auto".to_string(),
                                 });
                }
                SizablePropsOutputTarget::ImageNested => {
                    classes.push(nested_image_classes.clone());
                }
                SizablePropsOutputTarget::General => {
                    classes.push(match width {
                                     SizableOption::None => "w-full @[768px]/mdx:hidden".to_string(),
                                     SizableOption::Small => "w-full @[450px]/mdx:w-[320px]".to_string(),
                                     SizableOption::Smedium => "w-full @[550px]/mdx:w-[384px]".to_string(),
                                     SizableOption::Medium => "w-full @[650px]/mdx:w-[448px]".to_string(),
                                     SizableOption::Large => "w-full @[768px]/mdx:w-[576px]".to_string(),
                                     SizableOption::Xl => "w-full @[1080px]/mdx:w-[672px]".to_string(),
                                     SizableOption::Xxl => "w-full @[1080px]/mdx:w-[896px]".to_string(),
                                     SizableOption::Full => "w-full".to_string(),
                                     SizableOption::Fit => "w-fit".to_string(),
                                 });
                }
            };
        }
        if let Some(max_height) = &self.max_height {
            match output {
                SizablePropsOutputTarget::Image => {
                    classes.push(match max_height {
                                     SizableOption::None => "max-h-[min(32px,90vh,100%)] w-auto".to_string(),
                                     SizableOption::Small => "max-h-[min(320px,90vh,100%)] w-auto".to_string(),
                                     SizableOption::Smedium => "max-h-[min(384px,90vh,100%)] w-auto".to_string(),
                                     SizableOption::Medium => "max-h-[min(448px,90vh,100%)] w-auto".to_string(),
                                     SizableOption::Large => "max-h-[min(576px,90vh,100%)] w-auto".to_string(),
                                     SizableOption::Xl => "max-h-[min(672px,90vh,100%)] w-auto".to_string(),
                                     SizableOption::Xxl => "max-h-[min(896px,90vh,100%)] w-auto".to_string(),
                                     SizableOption::Full => "max-h-[min(100%,100vh)] w-auto".to_string(),
                                     SizableOption::Fit => "max-h-fit w-auto".to_string(),
                                 });
                }
                SizablePropsOutputTarget::ImageNested => {
                    classes.push(nested_image_classes.clone());
                }
                SizablePropsOutputTarget::General => {
                    classes.push(match max_height {
                                     SizableOption::None => "max-h-[min(32px,90vh,100%)]".to_string(),
                                     SizableOption::Small => "max-h-[min(320px,90vh,100%)]".to_string(),
                                     SizableOption::Smedium => "max-h-[min(384px,90vh,100%)]".to_string(),
                                     SizableOption::Medium => "max-h-[min(448px,90vh,100%)]".to_string(),
                                     SizableOption::Large => "max-h-[min(576px,90vh,100%)]".to_string(),
                                     SizableOption::Xl => "max-h-[min(672px,90vh,100%)]".to_string(),
                                     SizableOption::Xxl => "max-h-[min(896px,90vh,100%)]".to_string(),
                                     SizableOption::Full => "max-h-[min(100%,100vh)]".to_string(),
                                     SizableOption::Fit => "max-h-fit".to_string(),
                                 });
                }
            }
        }
        if let Some(max_width) = &self.max_width {
            classes.push(match max_width {
                             SizableOption::None => "max-w-[32px]".to_string(),
                             SizableOption::Small => "max-w-[320px]".to_string(),
                             SizableOption::Smedium => "max-w-[384px]".to_string(),
                             SizableOption::Medium => "max-w-[448px]".to_string(),
                             SizableOption::Large => "max-w-[576px]".to_string(),
                             SizableOption::Xl => "max-w-[672px]".to_string(),
                             SizableOption::Xxl => "max-w-[896px]".to_string(),
                             SizableOption::Full => "max-w-full".to_string(),
                             SizableOption::Fit => "max-w-fit".to_string(),
                         });
        }
        if let Some(height) = &self.height {
            match output {
                SizablePropsOutputTarget::Image => {
                    classes.push(match height {
                                     SizableOption::None => "h-fit w-auto".to_string(),
                                     SizableOption::Small => "h-24 w-auto".to_string(),
                                     SizableOption::Smedium => "h-32 w-auto".to_string(),
                                     SizableOption::Medium => "h-48 w-auto".to_string(),
                                     SizableOption::Large => "h-64 w-auto".to_string(),
                                     SizableOption::Xl => "h-96 w-auto".to_string(),
                                     SizableOption::Xxl => "h-screen w-auto".to_string(),
                                     SizableOption::Full => "h-full w-auto".to_string(),
                                     SizableOption::Fit => "h-fit w-auto".to_string(),
                                 });
                }
                SizablePropsOutputTarget::ImageNested => {
                    classes.push(nested_image_classes);
                }
                SizablePropsOutputTarget::General => {
                    classes.push(match height {
                                     SizableOption::None => "h-fit".to_string(),
                                     SizableOption::Small => "h-24".to_string(),
                                     SizableOption::Smedium => "h-32".to_string(),
                                     SizableOption::Medium => "h-48".to_string(),
                                     SizableOption::Large => "h-64".to_string(),
                                     SizableOption::Xl => "h-96".to_string(),
                                     SizableOption::Xxl => "h-screen".to_string(),
                                     SizableOption::Full => "h-full".to_string(),
                                     SizableOption::Fit => "h-fit".to_string(),
                                 });
                }
            };
        }
        if let Some(margin) = &self.margin {
            classes.push(match margin {
                             SizableOption::None => "m-0".to_string(),
                             SizableOption::Small => "m-2".to_string(),
                             SizableOption::Smedium => "m-3".to_string(),
                             SizableOption::Medium => "m-4".to_string(),
                             SizableOption::Large => "m-6".to_string(),
                             SizableOption::Xl => "m-8".to_string(),
                             SizableOption::Xxl => "m-12".to_string(),
                             SizableOption::Full => "m-16".to_string(),
                             SizableOption::Fit => "m-0".to_string(),
                         });
        }
        if let Some(margin_left) = &self.margin_left {
            classes.push(match margin_left {
                             SizableOption::None => "ml-0".to_string(),
                             SizableOption::Small => "ml-2".to_string(),
                             SizableOption::Smedium => "ml-3".to_string(),
                             SizableOption::Medium => "ml-4".to_string(),
                             SizableOption::Large => "ml-6".to_string(),
                             SizableOption::Xl => "ml-8".to_string(),
                             SizableOption::Xxl => "ml-12".to_string(),
                             SizableOption::Full => "ml-16".to_string(),
                             SizableOption::Fit => "ml-auto".to_string(),
                         });
        }
        if let Some(margin_right) = &self.margin_right {
            classes.push(match margin_right {
                             SizableOption::None => "mr-0".to_string(),
                             SizableOption::Small => "mr-2".to_string(),
                             SizableOption::Smedium => "mr-3".to_string(),
                             SizableOption::Medium => "mr-4".to_string(),
                             SizableOption::Large => "mr-6".to_string(),
                             SizableOption::Xl => "mr-8".to_string(),
                             SizableOption::Xxl => "mr-12".to_string(),
                             SizableOption::Full => "mr-16".to_string(),
                             SizableOption::Fit => "mr-auto".to_string(),
                         });
        }
        if let Some(margin_top) = &self.margin_top {
            classes.push(match margin_top {
                             SizableOption::None => "mt-0".to_string(),
                             SizableOption::Small => "mt-2".to_string(),
                             SizableOption::Smedium => "mt-3".to_string(),
                             SizableOption::Medium => "mt-4".to_string(),
                             SizableOption::Large => "mt-6".to_string(),
                             SizableOption::Xl => "mt-8".to_string(),
                             SizableOption::Xxl => "mt-12".to_string(),
                             SizableOption::Full => "mt-16".to_string(),
                             SizableOption::Fit => "mt-auto".to_string(),
                         });
        }
        if let Some(margin_bottom) = &self.margin_bottom {
            classes.push(match margin_bottom {
                             SizableOption::None => "h-fit".to_string(),
                             SizableOption::Small => "h-24".to_string(),
                             SizableOption::Smedium => "h-32".to_string(),
                             SizableOption::Medium => "h-48".to_string(),
                             SizableOption::Large => "h-64".to_string(),
                             SizableOption::Xl => "h-96".to_string(),
                             SizableOption::Xxl => "h-screen".to_string(),
                             SizableOption::Full => "h-full".to_string(),
                             SizableOption::Fit => "h-fit".to_string(),
                         });
        }
        if let Some(margin_y) = &self.margin_y {
            classes.push(match margin_y {
                             SizableOption::None => "my-0".to_string(),
                             SizableOption::Small => "my-2".to_string(),
                             SizableOption::Smedium => "my-3".to_string(),
                             SizableOption::Medium => "my-4".to_string(),
                             SizableOption::Large => "my-6".to_string(),
                             SizableOption::Xl => "my-8".to_string(),
                             SizableOption::Xxl => "my-12".to_string(),
                             SizableOption::Full => "my-16".to_string(),
                             SizableOption::Fit => "my-auto".to_string(),
                         });
        }
        if let Some(margin_x) = &self.margin_x {
            classes.push(match margin_x {
                             SizableOption::None => "mx-0".to_string(),
                             SizableOption::Small => "mx-2".to_string(),
                             SizableOption::Smedium => "mx-3".to_string(),
                             SizableOption::Medium => "mx-4".to_string(),
                             SizableOption::Large => "mx-6".to_string(),
                             SizableOption::Xl => "mx-8".to_string(),
                             SizableOption::Xxl => "mx-12".to_string(),
                             SizableOption::Full => "mx-16".to_string(),
                             SizableOption::Fit => "mx-auto".to_string(),
                         });
        }
        if let Some(padding) = &self.padding {
            classes.push(match padding {
                             SizableOption::None => "p-0".to_string(),
                             SizableOption::Small => "p-2".to_string(),
                             SizableOption::Smedium => "p-3".to_string(),
                             SizableOption::Medium => "p-4".to_string(),
                             SizableOption::Large => "p-6".to_string(),
                             SizableOption::Xl => "p-8".to_string(),
                             SizableOption::Xxl => "p-12".to_string(),
                             SizableOption::Full => "p-16".to_string(),
                             SizableOption::Fit => "p-0".to_string(),
                         });
        }
        if let Some(padding_left) = &self.padding_left {
            classes.push(match padding_left {
                             SizableOption::None => "pl-0".to_string(),
                             SizableOption::Small => "pl-2".to_string(),
                             SizableOption::Smedium => "pl-3".to_string(),
                             SizableOption::Medium => "pl-4".to_string(),
                             SizableOption::Large => "pl-6".to_string(),
                             SizableOption::Xl => "pl-8".to_string(),
                             SizableOption::Xxl => "pl-12".to_string(),
                             SizableOption::Full => "pl-16".to_string(),
                             SizableOption::Fit => "pl-0".to_string(),
                         });
        }
        if let Some(padding_right) = &self.padding_right {
            classes.push(match padding_right {
                             SizableOption::None => "pr-0".to_string(),
                             SizableOption::Small => "pr-2".to_string(),
                             SizableOption::Smedium => "pr-3".to_string(),
                             SizableOption::Medium => "pr-4".to_string(),
                             SizableOption::Large => "pr-6".to_string(),
                             SizableOption::Xl => "pr-8".to_string(),
                             SizableOption::Xxl => "pr-12".to_string(),
                             SizableOption::Full => "pr-16".to_string(),
                             SizableOption::Fit => "pr-0".to_string(),
                         });
        }
        if let Some(padding_top) = &self.padding_top {
            classes.push(match padding_top {
                             SizableOption::None => "pt-0".to_string(),
                             SizableOption::Small => "pt-2".to_string(),
                             SizableOption::Smedium => "pt-3".to_string(),
                             SizableOption::Medium => "pt-4".to_string(),
                             SizableOption::Large => "pt-6".to_string(),
                             SizableOption::Xl => "pt-8".to_string(),
                             SizableOption::Xxl => "pt-12".to_string(),
                             SizableOption::Full => "pt-16".to_string(),
                             SizableOption::Fit => "pt-0".to_string(),
                         });
        }
        if let Some(padding_bottom) = &self.padding_bottom {
            classes.push(match padding_bottom {
                             SizableOption::None => "pb-0".to_string(),
                             SizableOption::Small => "pb-2".to_string(),
                             SizableOption::Smedium => "pb-3".to_string(),
                             SizableOption::Medium => "pb-4".to_string(),
                             SizableOption::Large => "pb-6".to_string(),
                             SizableOption::Xl => "pb-8".to_string(),
                             SizableOption::Xxl => "pb-12".to_string(),
                             SizableOption::Full => "pb-16".to_string(),
                             SizableOption::Fit => "pb-0".to_string(),
                         });
        }
        if let Some(padding_y) = &self.padding_y {
            classes.push(match padding_y {
                             SizableOption::None => "py-0".to_string(),
                             SizableOption::Small => "py-2".to_string(),
                             SizableOption::Smedium => "py-3".to_string(),
                             SizableOption::Medium => "py-4".to_string(),
                             SizableOption::Large => "py-6".to_string(),
                             SizableOption::Xl => "py-8".to_string(),
                             SizableOption::Xxl => "py-12".to_string(),
                             SizableOption::Full => "py-16".to_string(),
                             SizableOption::Fit => "py-0".to_string(),
                         });
        }
        if let Some(padding_x) = &self.padding_x {
            classes.push(match padding_x {
                             SizableOption::None => "px-0".to_string(),
                             SizableOption::Small => "px-2".to_string(),
                             SizableOption::Smedium => "px-3".to_string(),
                             SizableOption::Medium => "px-4".to_string(),
                             SizableOption::Large => "px-6".to_string(),
                             SizableOption::Xl => "px-8".to_string(),
                             SizableOption::Xxl => "px-12".to_string(),
                             SizableOption::Full => "px-16".to_string(),
                             SizableOption::Fit => "px-0".to_string(),
                         });
        }
        if let Some(gap) = &self.gap {
            classes.push(match gap {
                             SizableOption::None => "gap-0".to_string(),
                             SizableOption::Small => "gap-2".to_string(),
                             SizableOption::Smedium => "gap-3".to_string(),
                             SizableOption::Medium => "gap-4".to_string(),
                             SizableOption::Large => "gap-6".to_string(),
                             SizableOption::Xl => "gap-8".to_string(),
                             SizableOption::Xxl => "gap-12".to_string(),
                             SizableOption::Full => "gap-16".to_string(),
                             SizableOption::Fit => "gap-0".to_string(),
                         });
        }
        if let Some(gap_y) = &self.gap_y {
            classes.push(match gap_y {
                             SizableOption::None => "gap-y-0".to_string(),
                             SizableOption::Small => "gap-y-2".to_string(),
                             SizableOption::Smedium => "gap-y-3".to_string(),
                             SizableOption::Medium => "gap-y-4".to_string(),
                             SizableOption::Large => "gap-y-6".to_string(),
                             SizableOption::Xl => "gap-y-8".to_string(),
                             SizableOption::Xxl => "gap-y-12".to_string(),
                             SizableOption::Full => "gap-y-16".to_string(),
                             SizableOption::Fit => "gap-0".to_string(),
                         });
        }
        if let Some(gap_x) = &self.gap_x {
            classes.push(match gap_x {
                             SizableOption::None => "gap-x-0".to_string(),
                             SizableOption::Small => "gap-x-2".to_string(),
                             SizableOption::Smedium => "gap-x-3".to_string(),
                             SizableOption::Medium => "gap-x-4".to_string(),
                             SizableOption::Large => "gap-x-6".to_string(),
                             SizableOption::Xl => "gap-x-8".to_string(),
                             SizableOption::Xxl => "gap-x-12".to_string(),
                             SizableOption::Full => "gap-x-16".to_string(),
                             SizableOption::Fit => "gap-x-0".to_string(),
                         });
        }

        classes
    }

    pub fn as_class(&self, sizable_target: SizablePropsOutputTarget) -> String {
        self.to_css_classes(sizable_target).join(" ")
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
                               max_width: props.get_sizable_option_at_key("maxWidth").ok(),
                               max_height: props.get_sizable_option_at_key("maxHeight").ok(),
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
        write!(f, "{}", self.as_class(crate::lang::lib::ui::shared_props::sizable::SizablePropsOutputTarget::General))
    }
}

impl FastWritable for SizablePropsGroup {
    fn write_into<W: core::fmt::Write + ?Sized>(&self,
                                                dest: &mut W,
                                                values: &dyn askama::Values)
                                                -> askama::Result<()> {
        self.as_class(crate::lang::lib::ui::shared_props::sizable::SizablePropsOutputTarget::General)
            .as_str()
            .write_into(dest, values)
    }
}
