use std::fmt::Display;

use cssparser::{
    Parser, ParserInput,
    color::{parse_hash_color, parse_named_color},
};
use lightningcss::{
    printer::PrinterOptions,
    traits::{Parse, ToCss},
    values::color::{ColorFallbackKind, CssColor as CL, RGBA},
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use winnow::error::ErrMode;

use crate::{
    lang::runtime::{
        state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
        traits::rendering::display_with_params::DisplayWithParam,
    },
    output::html::{
        web_specific_models::lightning_css_printer_options::safari_specific_lightning_css_printer_options,
        web_specific_traits::css_value_representable::{
            CSSInlineHtmlValuePairRepresentable, CSSValuePossiblyRepresentable, CSSValueRepresentable,
        },
    },
    parsers::{
        conundrum::color::{color_pair::ColorPair, conundrum_color::ConundrumColor},
        parser_trait::ConundrumParser,
    },
};

/// A simple wraper with some utility methods around the lightningcss struct of
/// the same name.
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CssColor(pub CL);

impl Display for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.0
                    .to_css_string(safari_specific_lightning_css_printer_options())
                    .or_else(|_| {
                        let fallback = self.0.get_fallback(ColorFallbackKind::RGB);
                        if let Ok((r, g, b, a)) = match fallback {
                            CL::RGBA(r) => Ok((r.red, r.green, r.blue, r.alpha)),
                            _ => Err(()),
                        } {
                            return Result::<String, ()>::Ok(format!("rgba({}, {}, {}, {})", r, g, b, a));
                        } else {
                            return Ok(fallback.to_css_string(safari_specific_lightning_css_printer_options())
                                              .unwrap_or_default());
                        }
                    })
                    .unwrap_or_default();
        write!(f, "{}", s)
    }
}

impl CssColor {
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(CL::RGBA(RGBA { red,
                             green,
                             blue,
                             alpha }))
    }
}

impl<'a> DisplayWithParam<PrinterOptions<'a>> for CssColor {
    fn display_with_param(&self, parser_opts: PrinterOptions<'a>) -> ConundrumModalResult<String> {
        if let Ok(r) = self.0.to_css_string(parser_opts).map_err(|e| {
                                                            log::warn!("CSS Error: {}", e);
                                                            ConundrumErrorVariant::InvalidColor("unknown".to_string())
                                                        })
        {
            Ok(r)
        } else {
            if let Ok(res) = self.0
                                 .get_fallback(ColorFallbackKind::RGB)
                                 .to_css_string(safari_specific_lightning_css_printer_options())
            {
                Ok(res)
            } else {
                Err(ErrMode::Cut(ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid color",
                                                                                                                 "Conundrum attempted to parse a color variable and could not. This is likely indicative of a larger problem"))))
            }
        }
    }
}

impl CSSInlineHtmlValuePairRepresentable<ConundrumColor> for CssColor {
    /// This sets the primary color as the *background*, and calculates a text
    /// color.
    fn as_inline_style_value_group(&self) -> super::color_pair::ColorPair<ConundrumColor> {
        // let x = self.0
        //             .to_css_string(safari_specific_lightning_css_printer_options())
        //             .inspect_err(|e| {
        //                 println!("Error: {}", e);
        //             })
        //             .unwrap_or("rgba(255, 0, 0, 1)".to_string());
        let x = ConundrumColor::Css(self.clone());
        ColorPair { background: x.clone(),
                    foreground: x }
    }
}

impl TryFrom<&str> for CssColor {
    type Error = ErrMode<ConundrumErrorVariant>;

    /// Should hopefully parse any valid css color as a string.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CL::parse_string(value).map_err(|_| ErrMode::Backtrack(ConundrumErrorVariant::InvalidColor(value.to_string())))
                               .map(Self)
    }
}

impl ConundrumParser<CssColor> for CssColor {
    fn parse_input_string(input: &mut crate::lang::runtime::traits::conundrum_input::ConundrumInput)
                          -> ConundrumModalResult<CssColor> {
        let mut nested_input = ParserInput::new(input.input);
        let mut parser = Parser::new(&mut nested_input);
        todo!()
    }

    fn matches_first_char(char: char) -> bool {
        true
    }
}

impl CSSValuePossiblyRepresentable for CssColor {
    fn to_css_value(&self) -> ConundrumModalResult<String> {
        let opts = safari_specific_lightning_css_printer_options();
        self.0.to_css_string(opts).map_err(|_| {
                                      ErrMode::Backtrack(
                    ConundrumErrorVariant::InvalidColor("unable to compile".to_string())
                )
                                  })
    }
}

impl CSSValueRepresentable for CssColor {
    fn as_css_value(&self) -> String {
        self.0.to_css_string(safari_specific_lightning_css_printer_options()).unwrap_or_else(|_| {
                                                                                 self.0
                                          .get_fallback(ColorFallbackKind::all())
                                          .to_css_string(safari_specific_lightning_css_printer_options())
                                          .unwrap_or_else(|_| String::from("rgb(255, 0, 0)"))
                                                                             })
    }
}
