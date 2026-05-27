#[derive(Debug, thiserror::Error, Clone)]
pub enum DocgenError {
    #[error("I'll fill this out later... it's just docgen stuff.")]
    GeneralError,
    #[error("Conundrum could not successfully visit all children.")]
    FailToVisitChildren,
}

pub type DocgenResult<T> = Result<T, DocgenError>;
