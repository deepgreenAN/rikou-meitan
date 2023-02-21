/// applicationのfront側に関するエラー
#[derive(thiserror::Error, Debug)]
pub enum AppFrontError {
    /// Fetch-APIに関するエラー
    #[error("FetchError: {0}")]
    FetchError(String),
    /// Serdeのシリアライズ・デシリアライズに関するエラー
    #[error("SerdeError: {0}")]
    SerdeError(String),
    /// サーバー側から送られてくるエラー
    #[error(transparent)]
    CommonError(#[from] common::AppCommonError),
}

impl From<gloo_net::Error> for AppFrontError {
    fn from(value: gloo_net::Error) -> Self {
        match value {
            e @ gloo_net::Error::JsError(_) | e @ gloo_net::Error::GlooError(_) => {
                Self::FetchError(format!("{e}"))
            }
            e @ gloo_net::Error::SerdeError(_) => Self::SerdeError(format!("{e}")),
        }
    }
}
