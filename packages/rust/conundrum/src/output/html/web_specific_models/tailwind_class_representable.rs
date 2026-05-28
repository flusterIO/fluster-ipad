use crate::lang::runtime::state::conundrum_error_variant::ConundrumModalResult;

pub trait TailwindClassRepresentable {
    fn as_tailwind_class(&self) -> String;
}

pub trait TryTailwindClassRepresentable {
    fn to_tailwind_class(&self) -> ConundrumModalResult<String>;
}
