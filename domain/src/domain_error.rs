#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("DomainError: Invalid Domain Value:{0}")]
    DomainLogicError(String),

    #[error("DomainError: Parse Error:{0}")]
    DomainParseError(String),

    #[error(transparent)]
    GenericParseError(#[from] GenericParseError),
}

#[derive(thiserror::Error, Debug)]
pub enum GenericParseError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    UuidParseError(#[from] uuid::Error),
}
