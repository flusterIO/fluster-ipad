#[derive(Debug, thiserror::Error, Clone)]
pub enum DocGenError {
    #[error("I'll fill this out later... it's just docgen stuff.")]
    GeneralError,
    #[error("Conundrum could not successfully visit all children.")]
    FailToVisitChildren,
    #[error("The {0} template failed to render.")]
    TemplateRenderError(String),
}

pub type DocGenResult<T> = Result<T, DocGenError>;
