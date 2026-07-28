/// The `model` that's been rejoined into something useful, not necessarily the
/// way it is represented in the database. This is the lowest level that most
/// applications will want to interect with their data as the `PureModel` trait
/// methods are called in patterns by the `ComposedModel` that are required to
/// keep the database stable.
pub trait ComposedModel<PureModelType> {
    fn to_pure_model(&self) -> PureModelType;
}

pub trait ComposedModelOptionalField<PureModelType> {
    fn to_optional_pure_model(&self) -> Option<PureModelType>;
}
