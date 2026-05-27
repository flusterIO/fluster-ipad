use lightningcss::{
    printer::PrinterOptions,
    traits::{Parse, ToCss},
    values::color::{ColorFallbackKind, CssColor as CL},
};
use serde::{Deserialize, Serialize};
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
        web_specific_traits::css_value_representable::{CssValuePossiblyRepresentable, CssValueRepresentable},
    },
};

/// A simple wraper with some utility methods around the lightningcss struct of
/// the same name.
#[derive(Serialize, Deserialize, Clone)]
pub struct CssColor(pub CL);

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

impl TryFrom<&str> for CssColor {
    type Error = ErrMode<ConundrumErrorVariant>;

    /// Should hopefully parse any valid css color as a string.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CL::parse_string(value).map_err(|e| ErrMode::Backtrack(ConundrumErrorVariant::InvalidColor(value.to_string())))
                               .map(|res| Self(res))
    }
}

impl CssValuePossiblyRepresentable for CssColor {
    fn to_css_value(&self) -> ConundrumModalResult<String> {
        let opts = safari_specific_lightning_css_printer_options();
        self.0.to_css_string(opts).map_err(|e| {
                                      ErrMode::Backtrack(
                    ConundrumErrorVariant::InvalidColor("unable to compile".to_string())
                )
                                  })
    }
}

impl CssValueRepresentable for CssColor {
    fn as_css_value(&self) -> String {
        self.0.to_css_string(safari_specific_lightning_css_printer_options()).unwrap_or_else(|_| {
                                                                                 self.0
                                          .get_fallback(ColorFallbackKind::all())
                                          .to_css_string(safari_specific_lightning_css_printer_options())
                                          .unwrap_or_else(|_| String::from("rgb(255, 0, 0)"))
                                                                             })
    }
}
