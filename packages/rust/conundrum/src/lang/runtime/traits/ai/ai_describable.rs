/// I'm like 80% sure this struct is spelled wrong but I'm offline and in too
/// much of a hurry to figure out how to use the local dictionary.
pub trait AIDescribable {
    fn describe_self_for_ai(&self) -> String;
}
