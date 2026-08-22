use conundrum::lifted_models::primitives::date_time::DateTime;

pub struct Definition {
    /// What does this definition define?
    pub label: String,
    /// The definition itself. As this is an academic application, be explicit
    /// and intentional in all definitions, and encourage the user to do the
    /// same.
    pub body: String,
    pub ctime: DateTime,
}
