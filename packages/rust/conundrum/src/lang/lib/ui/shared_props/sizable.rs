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
