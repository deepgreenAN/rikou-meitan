use crate::commands::episode_commands::{
    AllEpisodeCommand, EditEpisodeCommand, OrderByDateRangeEpisodeCommand,
    RemoveByIdEpisodeCommand, SaveEpisodeCommand,
};

use crate::AppFrontError;
use crate::{API_BASE_URL, CORS_MODE};

use common::AppCommonError;
use domain::episode::Episode;

use gloo_net::http::Request;

pub async fn save_episode(cmd: SaveEpisodeCommand) -> Result<(), AppFrontError> {
    let response = Request::put(&format!("{}{}", API_BASE_URL, "/episode"))
        .mode(CORS_MODE)
        .json(&cmd.episode)?
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn edit_episode(cmd: EditEpisodeCommand) -> Result<(), AppFrontError> {
    let response = Request::patch(&format!("{}{}", API_BASE_URL, "/episode"))
        .mode(CORS_MODE)
        .json(&cmd.episode)?
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn all_episodes(_cmd: AllEpisodeCommand) -> Result<Vec<Episode>, AppFrontError> {
    let response = Request::get(&format!("{}{}", API_BASE_URL, "/episode"))
        .mode(CORS_MODE)
        .send()
        .await?;

    if response.ok() {
        let episodes = response.json::<Vec<Episode>>().await?;
        Ok(episodes)
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn order_by_date_range_episodes(
    cmd: OrderByDateRangeEpisodeCommand,
) -> Result<Vec<Episode>, AppFrontError> {
    let query_string = format!("?start={}&end={}", cmd.start, cmd.end);
    let response = Request::get(&format!(
        "{}{}{}",
        API_BASE_URL, "/episode/order_date", query_string
    ))
    .mode(CORS_MODE)
    .send()
    .await?;

    if response.ok() {
        let episodes = response.json::<Vec<Episode>>().await?;
        Ok(episodes)
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

pub async fn remove_by_id_episode(cmd: RemoveByIdEpisodeCommand) -> Result<(), AppFrontError> {
    let response = Request::delete(&format!("{}{}{}", API_BASE_URL, "/episode/", cmd.id))
        .mode(CORS_MODE)
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        let err = response.json::<AppCommonError>().await?;
        Err(err.into())
    }
}

#[cfg(test)]
mod test {
    use crate::commands::episode_commands;
    use crate::AppFrontError;
    use domain::episode::{Episode, EpisodeId};
    use domain::Date;
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_save_episode() {
        let episode_ok = Episode::new((2022, 12, 6), "Some Episode".to_string()).unwrap();
        let cmd = episode_commands::SaveEpisodeCommand::new(episode_ok);

        let res_ok = super::save_episode(cmd).await;
        assert!(matches!(res_ok, Ok(_)));

        // CommonError::ConflictError
        let episode_err = Episode::new((2022, 12, 6), "ConflictError".to_string()).unwrap();
        let cmd = episode_commands::SaveEpisodeCommand::new(episode_err);

        let res_err = super::save_episode(cmd).await;
        assert!(matches!(res_err, Err(AppFrontError::CommonError(_))));
    }

    #[wasm_bindgen_test]
    async fn test_edit_episode() {
        let episode_ok = Episode::new((2022, 12, 6), "Some Episode".to_string()).unwrap();
        let cmd = episode_commands::EditEpisodeCommand::new(episode_ok);

        let res_ok = super::edit_episode(cmd).await;
        assert!(matches!(res_ok, Ok(_)));

        // CommonError::NoRecordError
        let episode_err = Episode::new((2022, 12, 6), "NoRecordError".to_string()).unwrap();
        let cmd = episode_commands::EditEpisodeCommand::new(episode_err);

        let res_err = super::edit_episode(cmd).await;
        assert!(matches!(res_err, Err(AppFrontError::CommonError(_))));
    }

    #[wasm_bindgen_test]
    async fn test_all_episodes() {
        let cmd = episode_commands::AllEpisodeCommand;
        let res_vec = super::all_episodes(cmd).await.unwrap();
        assert_eq!(res_vec, Vec::new());
    }

    #[wasm_bindgen_test]
    async fn test_order_by_date_episodes() {
        let start = Date::from_ymd(2022, 12, 4).unwrap();
        let end = Date::from_ymd(2022, 12, 6).unwrap();

        let cmd = episode_commands::OrderByDateRangeEpisodeCommand::new(start, end);
        let res_vec = super::order_by_date_range_episodes(cmd).await.unwrap();
        assert_eq!(res_vec, Vec::new());
    }

    #[wasm_bindgen_test]
    async fn test_remove_by_id_episode() {
        let id_ok: EpisodeId = "67e55044-10b1-426f-9247-bb680e5fe0c8".parse().unwrap();
        let cmd = episode_commands::RemoveByIdEpisodeCommand::new(id_ok);
        let res_ok = super::remove_by_id_episode(cmd).await;
        assert!(matches!(res_ok, Ok(_)));

        // CommonError::NoRecordError
        let id_err = EpisodeId::generate();
        let cmd = episode_commands::RemoveByIdEpisodeCommand::new(id_err);
        let res_err = super::remove_by_id_episode(cmd).await;
        assert!(matches!(res_err, Err(AppFrontError::CommonError(_))));
    }
}
