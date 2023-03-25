use crate::commands::video_commands;
use common::{AppCommonError, QueryInfo};
use domain::video::{Kirinuki, Video, VideoId};

// video_usecaseのモック化
#[cfg(not(test))]
use crate::usecases::video_usecases;

#[cfg(test)]
use crate::usecases::mock_video_usecases as video_usecases;

// AppStateのモック化
#[cfg(all(not(test), feature = "inmemory"))]
use crate::app_state::InMemoryAppState as AppState;

#[cfg(all(not(test), not(feature = "inmemory")))]
use crate::app_state::AppState;

#[cfg(test)]
use crate::app_state::MockAppState as AppState;

use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    extract::{Json, Path, Query, State},
};
use serde::Deserialize;
use std::str::FromStr;

pub async fn save_kirinuki(
    State(app_state): State<AppState>,
    video_res: Result<Json<Video<Kirinuki>>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let video = video_res?.0;
    let cmd = video_commands::SaveVideoCommand::new(video);
    video_usecases::save_video(app_state.kirinuki_repo, cmd).await?;
    Ok(())
}

pub async fn edit_kirinuki(
    State(app_state): State<AppState>,
    video_res: Result<Json<Video<Kirinuki>>, JsonRejection>,
) -> Result<(), AppCommonError> {
    let video = video_res?.0;
    let cmd = video_commands::EditVideoCommand::new(video);
    video_usecases::edit_video(app_state.kirinuki_repo, cmd).await?;
    Ok(())
}

pub async fn increment_like_kirinuki(
    id: Result<Path<VideoId>, PathRejection>,
    State(app_state): State<AppState>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = video_commands::IncrementLikeVideoCommand::new(id);
    video_usecases::increment_like_video(app_state.kirinuki_repo, cmd).await?;
    Ok(())
}

pub async fn all_kirinukis(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Video<Kirinuki>>>, AppCommonError> {
    let cmd = video_commands::AllVideosCommand;
    let videos = video_usecases::all_videos(app_state.kirinuki_repo, cmd).await?;
    Ok(Json(videos))
}

#[derive(Deserialize, strum_macros::EnumString)]
#[serde(try_from = "String")]
#[strum(serialize_all = "snake_case")]
pub enum SortType {
    Date,
    Like,
}

impl TryFrom<String> for SortType {
    type Error = <SortType as FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Deserialize)]
pub struct KirinukiQuery {
    sort_type: SortType,
    length: Option<usize>,
}

pub async fn get_kirinukis_with_query(
    path_query_res: Result<Query<KirinukiQuery>, QueryRejection>,
    State(app_state): State<AppState>,
    query_info_res: Result<Json<QueryInfo<Video<Kirinuki>>>, JsonRejection>,
) -> Result<Json<Vec<Video<Kirinuki>>>, AppCommonError> {
    let path_query = path_query_res?.0;
    let reference_video = match query_info_res {
        // リクエストに正しいjsonが与えられた場合
        Ok(query_info) => query_info.0.reference,
        // Jsonが与えら無かった場合
        Err(JsonRejection::MissingJsonContentType(_)) => None,
        // その他のJsonRejection
        Err(e) => return Err(e.into()),
    };

    match (path_query.sort_type, path_query.length) {
        // ソートタイプがLikeの場合
        (SortType::Like, Some(length)) => {
            match reference_video {
                // QueryInfoにreference_originalが与えられた場合
                Some(reference_video) => {
                    let cmd =
                        video_commands::OrderByLikeLaterVideosCommand::new(reference_video, length);
                    let videos =
                        video_usecases::order_by_like_later_videos(app_state.kirinuki_repo, cmd)
                            .await?;
                    Ok(Json(videos))
                }
                // QueryInfoにreference_originalが与えられなかった場合
                None => {
                    let cmd = video_commands::OrderByLikeVideosCommand::new(length);
                    let videos =
                        video_usecases::order_by_like_videos(app_state.kirinuki_repo, cmd).await?;
                    Ok(Json(videos))
                }
            }
        }
        // ソートタイプがDateの場合
        (SortType::Date, Some(length)) => {
            match reference_video {
                // QueryInfoにreference_originalが与えられた場合
                Some(reference_video) => {
                    let cmd =
                        video_commands::OrderByDateLaterVideosCommand::new(reference_video, length);
                    let videos =
                        video_usecases::order_by_date_later_videos(app_state.kirinuki_repo, cmd)
                            .await?;
                    Ok(Json(videos))
                }
                // QueryInfoにreference_originalが与えられなかった場合
                None => {
                    let cmd = video_commands::OrderByDateVideosCommand::new(length);
                    let videos =
                        video_usecases::order_by_date_videos(app_state.kirinuki_repo, cmd).await?;
                    Ok(Json(videos))
                }
            }
        }
        // 無効なクエリの場合
        _ => Err(AppCommonError::QueryStringRejectionError(
            "Invalid query parameter combination.".to_string(),
        )),
    }
}

pub async fn remove_kirinuki(
    id: Result<Path<VideoId>, PathRejection>,
    State(app_state): State<AppState>,
) -> Result<(), AppCommonError> {
    let id = id?.0;
    let cmd = video_commands::RemoveVideoCommand::new(id);
    video_usecases::remove_video(app_state.kirinuki_repo, cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // asyncなため重複するモックを利用できない
}
