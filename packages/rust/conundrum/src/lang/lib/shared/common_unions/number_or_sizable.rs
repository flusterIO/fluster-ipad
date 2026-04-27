use serde::Serialize;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::{
            shared_props::sizable_option::SizableOption, ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        },
        runtime::state::{conundrum_error::ConundrumError, conundrum_error_variant::ConundrumErrorVariant},
    },
    parsers::conundrum::logic::number::conundrum_number::ConundrumNumber,
};

#[typeshare::typeshare]
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "tag", content = "content")]
pub enum NumberOrSizable {
    Sizable(SizableOption),
    Number(ConundrumNumber),
}

impl FromJsxPropsOptional for NumberOrSizable {
    fn from_jsx_props(props: &crate::parsers::conundrum::logic::object::object::ConundrumObject,
                      key: &str)
                      -> crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult<Self>
        where Self: Sized {
        if let Ok(from_sizable) = SizableOption::from_jsx_props(&props, &key) {
            Ok(NumberOrSizable::Sizable(from_sizable))
        } else if let Ok(from_number) = ConundrumNumber::from_jsx_props(&props, &key) {
            Ok(NumberOrSizable::Number(from_number))
        } else {
            Err(
                    ErrMode::Backtrack(
                        ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Invalid property", format!("Conundrum was looking for a number at the `{}` property but could not find one.", key).as_str()))
                    )
                )
        }
    }
}
