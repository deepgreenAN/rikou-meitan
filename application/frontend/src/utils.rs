use crate::AppFrontError;
use common::AppCommonError;
use gloo_net::http::Response;
use serde::de::DeserializeOwned;

/// レスポンスのResultを特定の型とエラーにデシリアライズ
pub async fn deserialize_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, AppFrontError> {
    match response.ok() {
        true => {
            let data = response.json::<T>().await?;
            Ok(data)
        }
        false => {
            let err = response.json::<AppCommonError>().await?;
            Err(err.into())
        }
    }
}

/// レスポンスのResultを()型とエラーにデシリアライズ
pub async fn deserialize_response_null(response: Response) -> Result<(), AppFrontError> {
    match response.ok() {
        true => Ok(()),
        false => {
            let err = response.json::<AppCommonError>().await?;
            Err(err.into())
        }
    }
}
