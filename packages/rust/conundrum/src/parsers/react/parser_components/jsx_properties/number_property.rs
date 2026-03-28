use serde::{Deserialize, Serialize};
use winnow::{ModalResult, Parser, stream::Stream};

use crate::{
    lang::runtime::traits::conundrum_input::ConundrumInput,
    parsers::react::parser_components::jsx_properties::jsx_curly_bracket_wrapped_property::jsx_curly_bracket_wrapped_property,
};

#[typeshare::typeshare]
#[derive(Debug, Serialize, Deserialize)]
pub struct JsxNumberPropertyResult {
    pub key: String,
    pub value: f64,
}

pub fn jsx_number_property(input: &mut ConundrumInput) -> ModalResult<()> {
    let start = input.input.checkpoint();
    let (key, bracketed_content) = jsx_curly_bracket_wrapped_property.parse_next(input).inspect_err(|_| {
                                                                                            input.input.reset(&start);
                                                                                        })?;

    Ok(())
}
