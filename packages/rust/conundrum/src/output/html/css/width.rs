use tw_merge::{AsTailwindClass, TwVariant};

#[derive(TwVariant)]
pub enum ComponentWidthClass {
    #[tw(default, class = "max-w-full w-full")]
    Default,
}
