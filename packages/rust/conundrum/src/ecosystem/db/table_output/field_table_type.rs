/// # FieldTableType
///
/// The type of the database field associated with each key in the database.
pub enum FieldTableType {
    /// An integer or a float, a number of any type.
    Number,
    Integer,
    Float,
    String,
    Date,
    Boolean,
}
