/// ドメインに関するエラー
#[derive(thiserror::Error, Debug, Clone)]
pub enum DomainError {
    /// コンストラクト時などでドメインロジックと入力が矛盾する場合のエラー．
    #[error("DomainError::DomainLogicError: {0}")]
    DomainLogicError(String),
    /// プリミティブな型などからドメイン固有型へのパースの際のロジックのエラー．serdeのデシリアライズなどで起こる
    #[error("DomainError::DomainParseError: {0}")]
    DomainParseError(String),
    /// 外部クレートの対応する型との変換のエラー．
    #[error("DomainError::DomainConvertExternalError: {0}")]
    DomainConvertExternalError(String),
}

/// ジェネリックなパースに関するエラー
#[derive(thiserror::Error, Debug, Clone)]

pub enum GenericParseError {
    /// UUIDのパースに関するエラー
    #[error("GenericParseError::ParseUuidError: {0}")]
    ParseUuidError(#[from] uuid::Error),
    /// NaiveDateのパースに関するエラー
    #[error("GenericParseError::ParseDateError: {0}")]
    ParseDateError(#[from] chrono::ParseError),
}

impl From<GenericParseError> for DomainError {
    fn from(value: GenericParseError) -> Self {
        DomainError::DomainParseError(value.to_string())
    }
}

#[cfg(feature = "server")]
impl From<DomainError> for sqlx::Error {
    fn from(domain_error: DomainError) -> Self {
        sqlx::Error::Decode(Box::new(domain_error))
    }
}
