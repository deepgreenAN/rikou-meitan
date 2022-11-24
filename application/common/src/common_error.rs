use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use infrastructure::InfraError;

#[cfg(feature = "server")]
use domain::DomainError;

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum AppCommonError {
    #[error("{0}")]
    DomainError(String),

    #[error("{0}")]
    DBConnectionError(String),

    #[error("{0}")]
    OtherSQLXError(String),

    #[error("RemovedRecordError: Removed row accessed")]
    RemovedRecordError,
}

// -------------------------------------------------------------------------------------------------
// Fromトレイトはfeature = serverのときのみ

#[cfg(feature = "server")]
impl From<InfraError> for AppCommonError {
    fn from(infra_error: InfraError) -> Self {
        match infra_error {
            InfraError::DomainError(err) => {
                AppCommonError::DomainError(format!("{}", InfraError::DomainError(err)))
            }
            InfraError::DBConnectionError(err) => {
                AppCommonError::DBConnectionError(format!("{}", InfraError::DBConnectionError(err)))
            }
            InfraError::OtherSQLXError(err) => {
                AppCommonError::OtherSQLXError(format!("{}", InfraError::OtherSQLXError(err)))
            }
            InfraError::RemovedRecordError => AppCommonError::RemovedRecordError,
        }
    }
}

#[cfg(feature = "server")]
impl From<DomainError> for AppCommonError {
    fn from(domain_error: DomainError) -> Self {
        AppCommonError::DomainError(format!("{}", domain_error))
    }
}
