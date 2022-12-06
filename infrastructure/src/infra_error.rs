use domain::DomainError;

/// インフラに関するエラー
#[derive(thiserror::Error, Debug, Clone)]
pub enum InfraError {
    #[error(transparent)]
    DomainError(#[from] DomainError),

    #[error("DBConnectionError:{0}")]
    DBConnectionError(String),

    #[error("OtherSQLXError:{0}")]
    OtherSQLXError(String),

    #[error("DecodeError:{0}")]
    DBDecodeError(String),

    #[error("ConflictError: duplicated row inserted")]
    ConflictError,

    #[error("NoRecordError: Removed row accessed")]
    NoRecordError,
}

impl From<sqlx::Error> for InfraError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error {
            sqlx::Error::Io(err) => {
                InfraError::DBConnectionError(format!("{}", sqlx::Error::Io(err)))
            }
            sqlx::Error::PoolClosed => {
                InfraError::DBConnectionError(format!("{}", sqlx::Error::PoolClosed))
            }
            sqlx::Error::PoolTimedOut => {
                InfraError::DBConnectionError(format!("{}", sqlx::Error::PoolTimedOut))
            }
            sqlx::Error::Tls(err) => {
                InfraError::DBConnectionError(format!("{}", sqlx::Error::Tls(err)))
            }
            sqlx::Error::Protocol(err) => {
                InfraError::DBConnectionError(format!("{}", sqlx::Error::Protocol(err)))
            }
            sqlx::Error::Database(err) => {
                InfraError::DBConnectionError(format!("{}", sqlx::Error::Database(err)))
            }
            sqlx::Error::Configuration(err) => {
                InfraError::OtherSQLXError(format!("{}", sqlx::Error::Configuration(err)))
            }
            sqlx::Error::RowNotFound => {
                InfraError::OtherSQLXError(format!("{}", sqlx::Error::RowNotFound))
            }
            sqlx::Error::TypeNotFound { type_name } => {
                InfraError::OtherSQLXError(format!("{}", sqlx::Error::TypeNotFound { type_name }))
            }
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => InfraError::OtherSQLXError(
                format!("{}", sqlx::Error::ColumnIndexOutOfBounds { index, len }),
            ),
            sqlx::Error::ColumnNotFound(err) => {
                InfraError::OtherSQLXError(format!("{}", sqlx::Error::ColumnNotFound(err)))
            }
            sqlx::Error::ColumnDecode { index, source } => InfraError::OtherSQLXError(format!(
                "{}",
                sqlx::Error::ColumnDecode { index, source }
            )),
            sqlx::Error::Decode(err) => {
                InfraError::DBDecodeError(format!("{}", sqlx::Error::Decode(err)))
            }
            sqlx::Error::WorkerCrashed => {
                InfraError::OtherSQLXError(format!("{}", sqlx::Error::WorkerCrashed))
            }
            sqlx::Error::Migrate(err) => {
                InfraError::OtherSQLXError(format!("{}", sqlx::Error::Migrate(err)))
            }
            _ => InfraError::OtherSQLXError("Undefined Error".to_string()),
        }
    }
}
