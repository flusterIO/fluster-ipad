use crate::vector::database::db_traits::pure_model_instance::PureModelInstanceMethods;

/// The `model` that's been rejoined into something useful, not necessarily the
/// way it is represented in the database. This is the lowest level that most
/// applications will want to interect with their data as the `PureModel` trait
/// methods are called in patterns by the `UnifiedModel` that are required to
/// keep the database stable.
pub trait UnifiedModel {
    fn to_pure_models() -> Vec<Box<dyn PureModelInstanceMethods>>;
}
