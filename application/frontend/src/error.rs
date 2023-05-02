/// applicationのfront側に関するエラー
#[derive(thiserror::Error, Debug)]
pub enum AppFrontError {
    /// Fetch-APIに関するエラー
    #[error("AppFrontError::FetchError: {0}")]
    FetchError(String),
    /// Serdeのシリアライズ・デシリアライズに関するエラー
    #[error("AppFrontError::SerdeError: {0}")]
    SerdeError(String),
    /// サーバー側から送られてくるエラー
    #[error("AppFrontError::CommonError: {0}")]
    CommonError(#[from] common::AppCommonError),
}

impl From<reqwest::Error> for AppFrontError {
    fn from(value: reqwest::Error) -> Self {
        // jsonのデコードに関するエラー
        if value.is_decode() {
            AppFrontError::SerdeError(format!("{value}"))
        } else {
            // その他のエラー
            AppFrontError::FetchError(format!("{value}"))
        }
    }
}
