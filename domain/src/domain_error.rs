/// ドメインに関するエラー
#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("DomainError: Invalid Domain Value:{0}")]
    DomainLogicError(String),

    #[error("DomainError: ParseError:{0}")]
    DomainParseError(String),

    #[error("DomainError: UrlParseError:{0}")]
    UrlParseError(String),

    #[error("DomainError: NotChangedError:{0}")]
    NotChangedError(String),

    #[error(transparent)]
    GenericParseError(#[from] GenericParseError),
}

/// ジェネリックなパースに関するエラー
#[derive(thiserror::Error, Debug)]

pub enum GenericParseError {
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    ParseUuidError(#[from] uuid::Error),
}

#[cfg(feature = "server")]
impl From<DomainError> for sqlx::Error {
    fn from(domain_error: DomainError) -> Self {
        sqlx::Error::Decode(Box::new(domain_error))
    }
}
