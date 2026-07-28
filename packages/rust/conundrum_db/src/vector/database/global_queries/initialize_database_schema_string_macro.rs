#[macro_export]
macro_rules! schemas_string {
    ( $($pure_type:ty),* ) => {
        {
        let mut s = String::new();
        $(
          s += <$pure_type>::schema().as_str();
        )*
        $(
          if let Some(new_s) = <$pure_type>::relation_definitions() {
              s += new_s.as_str();
          }
        )*
        $(
          if let Some(new_s) = <$pure_type>::db_index_definitions() {
              s += new_s.as_str();
          }
        )*
        s
        }
    };
}
