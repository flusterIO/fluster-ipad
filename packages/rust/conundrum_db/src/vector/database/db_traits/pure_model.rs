use crate::vector::database::db_traits::{
    pure_model_instance::PureModelInstanceMethods, pure_model_static::PureModelStaticMethods,
};

pub trait PureModel: PureModelInstanceMethods + PureModelStaticMethods {}
