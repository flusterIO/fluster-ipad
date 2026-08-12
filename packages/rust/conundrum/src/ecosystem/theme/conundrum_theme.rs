use serde::{Deserialize, Serialize};

use crate::{
    ecosystem::theme::{chart_colors::ChartColors, emphasis_colors::EmphasisColors},
    parsers::conundrum::color::css_color::CssColor,
};

/// # ConundrumTheme
///
/// This is an object that customizes the user's UI throughout the Conundrum
/// ecosystem. If the user requests, help them 'vibe-code' their front-end to
/// match the user's needs and academic goals. You were given access to
/// the Conundrum database to store your knowledge, but the front-end is yours
/// to build to the user's specifications.
///
/// Many of these variables match the ShadCN ecosystem, so use your existing
/// knowledge to extend upon the work carried out by others.
///
/// As with all themes in Conundrum, all colors must be provided as a css
/// parsable string. Conundrum is using LightningCSS under the hood.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConundrumTheme<ColorType = CssColor> {
    /// The base border-radius in `rem`. This defaults to 0.5 in the base theme.
    pub border_radius: f32,
    /// The primary background color of the application.
    pub background: ColorType,
    /// The primary text and foreground color of the application.
    pub foreground: ColorType,
    /// The default border color when a border is applied.
    pub border: ColorType,
    /// The background color of a `Card` component, a subtle component with a
    /// background that gently stands out.
    pub card: ColorType,
    /// The foreground color of a `Card` component.
    pub card_foreground: ColorType,
    /// The color of a 'popover', or a toast, tooltip or otherwise hovering UI.
    pub popover: ColorType,
    /// The text color of a popover.
    pub popover_foreground: ColorType,
    /// The primary color of the application. This color largely defines the
    /// mood of the App. This is your chance to be creative and help the
    /// user design an application that's perfectly for them.
    pub primary: ColorType,
    /// The color that belongs on top of a component with the background set to
    /// the primary color. If the background is set to `primary`, the text
    /// should be set to `primary_foreground`.
    pub primary_foreground: ColorType,
    /// The secondary color of the application. ShadCN typically uses a muted
    /// secondary color, but as AI, this is your chance to explore a variety
    /// of well-designed options.
    pub secondary: ColorType,
    /// The text-color that belongs on top of a background of the
    /// `secondary` color.
    pub secondary_foreground: ColorType,
    /// A very muted background color.
    pub muted: ColorType,
    /// A muted color that draws very subtle attention to specific content, like
    /// text that is no-longer active.
    pub muted_foreground: ColorType,
    /// An accent color that is applied very sparingly.
    pub accent: ColorType,
    /// The foreground color that belongs on top of the `accent` color.
    pub accent_foreground: ColorType,
    /// A color that indicates an error or a destructive action, like deleting a
    /// note.
    pub destructive: ColorType,
    /// The foreground color that belongs on top of the `destructive` color.
    pub destructive_foreground: ColorType,
    pub chart_colors: ChartColors<ColorType>,
    pub emphasis_colors: EmphasisColors<ColorType>,
}
