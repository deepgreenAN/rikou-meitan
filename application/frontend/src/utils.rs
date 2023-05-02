use crate::AppFrontError;
use common::AppCommonError;
use reqwest::Response;
use serde::de::DeserializeOwned;

/// レスポンスのResultを特定の型とエラーにデシリアライズ
pub async fn deserialize_response<T: DeserializeOwned>(
    response: Response,
) -> Result<T, AppFrontError> {
    match response.status().is_success() {
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
    match response.status().is_success() {
        true => Ok(()),
        false => {
            let err = response.json::<AppCommonError>().await?;
            Err(err.into())
        }
    }
}
