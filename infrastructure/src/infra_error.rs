use domain::DomainError;

/// インフラに関するエラー
#[derive(thiserror::Error, Debug)]
pub enum InfraError {
    #[error("InfraError: DatabaseError:{0}")]
    DatabaseError(String),

    #[error(transparent)]
    DomainError(#[from] DomainError),

    #[error(transparent)]
    SQLXError(#[from] sqlx::Error),
}
