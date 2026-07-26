pub enum AcademicResultMetric {
    /// A 100 based percentage, similar to score on a test.
    Percent(f32),
    /// An absolute value indicating the error percentage.
    PercentError(u32),
    /// A result on a test, like 64/70 when you don't want to convert it to a
    /// percentage.
    RationalScore {
        numerator: u32,
        denominator: u32,
    },
    /// Standard Deviation
    StandardDeviation(f32),
    /// A custom metric, with the value and label.
    Custom {
        value: f32,
        label: String,
    },
}
