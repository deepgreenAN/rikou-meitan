#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("DomainError: Invalid Domain Value:{0}")]
    DomainLogicError(String),

    #[error("DomainError: Parse Error:")]
    DomainParseError(String),

    #[error(transparent)]
    OtherParseError(#[from] anyhow::Error),
}
