#[derive(uniffi::Error)]
pub enum FlusterError {
    CanaryError,
    // IndexOutOfBounds { index: u32, size: u32 },
    // tuple-enums work.
    // Generic(String),
}

pub type FlusterResult<T> = Result<T, FlusterError>;
