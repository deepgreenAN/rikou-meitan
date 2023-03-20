use serde::{Deserialize, Serialize};

/// サーバーサイドのエラーをフロントエンドに伝えるためのエラー．
#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
pub enum AppCommonError {
    #[error("{0}")]
    DomainError(String),

    #[error("{0}")]
    DBConnectionError(String),

    #[error("{0}")]
    OtherSQLXError(String),

    #[error("{0}")]
    DBDecodeError(String),

    #[error("ConflictError:tried to insert duplicated row")]
    ConflictError,

    #[error("NoRecordError: not existing row accessed")]
    NoRecordError,

    #[error("JsonRejectionError: {0}")]
    JsonRejectionError(String),

    #[error("QueryStringRejectionError: {0}")]
    QueryStringRejectionError(String),

    #[error("PathRejectionError: {0}")]
    PathRejectionError(String),
}

#[cfg(feature = "server")]
mod from_server_errors_into_response {
    use super::AppCommonError;
    use axum::{http::StatusCode, response::IntoResponse, Json};
    use domain::DomainError;
    use infrastructure::InfraError;

    // -------------------------------------------------------------------------------------------------
    // Fromトレイト
    impl From<InfraError> for AppCommonError {
        fn from(infra_error: InfraError) -> Self {
            match infra_error {
                e @ InfraError::DomainError(_) => AppCommonError::DomainError(format!("{e}")),
                e @ InfraError::DBConnectionError(_) => {
                    AppCommonError::DBConnectionError(format!("{e}"))
                }
                e @ InfraError::OtherSQLXError(_) => AppCommonError::OtherSQLXError(format!("{e}")),
                e @ InfraError::DBDecodeError(_) => AppCommonError::DBDecodeError(format!("{e}")),
                InfraError::ConflictError => AppCommonError::ConflictError,
                InfraError::NoRecordError => AppCommonError::NoRecordError,
            }
        }
    }

    impl From<DomainError> for AppCommonError {
        fn from(domain_error: DomainError) -> Self {
            AppCommonError::DomainError(format!("{domain_error}"))
        }
    }

    impl From<axum::extract::rejection::JsonRejection> for AppCommonError {
        fn from(json_rejection_error: axum::extract::rejection::JsonRejection) -> Self {
            AppCommonError::JsonRejectionError(format!("{json_rejection_error}"))
        }
    }

    impl From<axum::extract::rejection::QueryRejection> for AppCommonError {
        fn from(query_rejection_error: axum::extract::rejection::QueryRejection) -> Self {
            AppCommonError::QueryStringRejectionError(format!("{query_rejection_error}"))
        }
    }

    impl From<axum::extract::rejection::PathRejection> for AppCommonError {
        fn from(path_rejection_error: axum::extract::rejection::PathRejection) -> Self {
            AppCommonError::PathRejectionError(format!("{path_rejection_error}"))
        }
    }

    // -------------------------------------------------------------------------------------------------
    // IntoResponseトレイトの実装(StatusCode, AppCommonError)に変換

    impl IntoResponse for AppCommonError {
        fn into_response(self) -> axum::response::Response {
            match self {
                Self::JsonRejectionError(_) => {
                    (StatusCode::BAD_REQUEST, Json(self)).into_response()
                }
                Self::QueryStringRejectionError(_) => {
                    (StatusCode::BAD_REQUEST, Json(self)).into_response()
                }
                Self::PathRejectionError(_) => (StatusCode::NOT_FOUND, Json(self)).into_response(),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(self)).into_response(),
            }
        }
    }
}
