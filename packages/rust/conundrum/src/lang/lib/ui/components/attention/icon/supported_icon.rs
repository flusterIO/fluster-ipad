use derive_more::derive;
use lucide_icons::Icon;
use serde::Serialize;
use winnow::error::ErrMode;

use crate::{
    lang::{
        lib::ui::ui_traits::jsx_prop_representable::FromJsxPropsOptional,
        runtime::state::{
            conundrum_error::ConundrumError,
            conundrum_error_variant::{ConundrumErrorVariant, ConundrumModalResult},
        },
    },
    parsers::conundrum::logic::object::object::ConundrumObject,
};

#[derive(Serialize, Clone, Debug)]
pub struct SupportedIcon(pub Icon);

impl FromJsxPropsOptional for SupportedIcon {
    fn from_jsx_props(props: &ConundrumObject, key: &str) -> ConundrumModalResult<Self>
        where Self: Sized {
        let x = props.get_string(key, None).map_err(ErrMode::Backtrack)?;
        let icon = Icon::try_from(x.0.as_str()).map_err(|_| {
ErrMode::Cut(
ConundrumErrorVariant::InternalParserError(ConundrumError::from_msg_and_details("Icon error", "Conundrum was looking for a valid icon name. Unfortunately the icon documentation is still a work in progress, but all **Lucide** icon names are supported."))
)
        })?;
        Ok(SupportedIcon(icon))
    }
}
