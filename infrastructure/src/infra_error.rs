use domain::DomainError;

/// インフラに関するエラー
#[derive(thiserror::Error, Debug, Clone)]
pub enum InfraError {
    /// ドメインエラーから生成されたエラー
    #[error("InfraError::DomainError: {0}")]
    DomainError(#[from] DomainError),
    /// データベースのコネクションに関するエラー．
    #[error("InfraError::DBConnectionError: {0}")]
    DBConnectionError(String),
    /// その他のsqlxに関するエラー
    #[error("InfraError::OtherSQLXError: {0}")]
    OtherSQLXError(String),
    /// データベースのドメイン固有型へのデコードエラー．
    #[error("InfraError::DecodeError: {0}")]
    DBDecodeError(String),
    /// データの保存時に既にテータが存在する場合のエラー．テストでのみ起こると想定
    #[error("InfraError::ConflictError: duplicated row inserted")]
    ConflictError,
    /// 編集・削除を行うときにデータが存在しない場合のエラー．テストでのみ起こると想定
    #[error("InfraError::NoRecordError: Removed row accessed")]
    NoRecordError,
}

impl From<sqlx::Error> for InfraError {
    fn from(sqlx_error: sqlx::Error) -> Self {
        match sqlx_error {
            e @ sqlx::Error::Io(_)
            | e @ sqlx::Error::PoolClosed
            | e @ sqlx::Error::PoolTimedOut
            | e @ sqlx::Error::Tls(_)
            | e @ sqlx::Error::Protocol(_)
            | e @ sqlx::Error::Database(_) => InfraError::DBConnectionError(format!("{e}")),

            e @ sqlx::Error::Configuration(_)
            | e @ sqlx::Error::RowNotFound
            | e @ sqlx::Error::TypeNotFound { type_name: _ }
            | e @ sqlx::Error::ColumnIndexOutOfBounds { index: _, len: _ }
            | e @ sqlx::Error::ColumnNotFound(_)
            | e @ sqlx::Error::ColumnDecode {
                index: _,
                source: _,
            }
            | e @ sqlx::Error::WorkerCrashed
            | e @ sqlx::Error::Migrate(_) => InfraError::OtherSQLXError(format!("{e}")),

            e @ sqlx::Error::Decode(_) => InfraError::DBDecodeError(format!("{e}")),
            e @ _ => InfraError::OtherSQLXError(format!("Undefined Error: {e}")),
        }
    }
}
