use katex::{KatexContext, Settings, TrustSetting, render_to_string};

use crate::lang::runtime::{
    run_conundrum::ParseConundrumOptions,
    state::{
        conundrum_error::ConundrumError,
        conundrum_error_variant::{ConundrumErrorVariant, ConundrumResult},
        parse_state::ConundrumModifier,
    },
};

/// ## Math Mode
///
///
/// Accepts the content that goes inside of the `$$`, **not** wrapped in that
/// syntax. Also, I _really_ wanted to include this as part of the main
/// function, just triggered by a flag, but that would've added a lot of
/// complexity due to the type clash at a time when I'm superrrrr busy, but I'll
/// come back to it.
///
/// ## Example
///
/// ```rs
/// let html = run_math_mode("\\delta = 2G \\frac{M}{R^3}");
/// ```
pub fn run_math_mode(opts: ParseConundrumOptions, format: katex::OutputFormat) -> ConundrumResult<String> {
    let is_inline = opts.modifiers.contains(&ConundrumModifier::MathModeInline);
    let context = KatexContext::default();
    let settings = Settings::builder().display_mode(!is_inline)
                                      .trust(TrustSetting::Bool(opts.trusted))
                                      .color_is_text_color(true)
                                      .output(format)
                                      .build();

    render_to_string(&context, &opts.content, &settings).map_err(|e| {
                ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Math Error", format!(r#"Conundrum could not compile a math block with the following content:

```tex
{}
```
                        "#, opts.content).as_str())
            )
        })
}
