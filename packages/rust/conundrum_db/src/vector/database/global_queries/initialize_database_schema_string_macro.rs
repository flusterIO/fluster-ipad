use quote::quote;

#[macro_export]
macro_rules! schemas_string {
    ( $($pure_type:ty),* ) => {
        {
        let mut s = String::new();
        $(
          s += <$pure_type>::schema().as_str();
        )*
        s
        }
    };
}
