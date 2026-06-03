use crate::{
    lang::runtime::{state::conundrum_error_variant::ConundrumModalResult, traits::conundrum_input::ArcState},
    parsers::conundrum::color::conundrum_color::ConundrumColor,
};

pub trait ColorRepresentable {
    fn as_conundrum_color(&self) -> ConundrumColor;
}

pub trait ColorPossiblyRepresentable {
    fn as_conundrum_color(&self, state: ArcState) -> ConundrumModalResult<ConundrumColor>;
}
