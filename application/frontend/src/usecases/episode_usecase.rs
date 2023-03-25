#[cfg(not(feature = "fake"))]
pub use self::product::*;

#[cfg(feature = "fake")]
pub use self::fake::*;

#[cfg(not(feature = "fake"))]
mod product {
    use crate::commands::episode_commands;
    use crate::{
        utils::{deserialize_response, deserialize_response_null},
        AppFrontError,
    };
    use crate::{API_BASE_URL, CORS_MODE};
    use domain::episode::Episode;

    use gloo_net::http::Request;

    /// エピソードを保存
    pub async fn save_episode<'a>(
        cmd: episode_commands::SaveEpisodeCommand<'_>,
    ) -> Result<(), AppFrontError> {
        let response = Request::put(&format!("{}{}", API_BASE_URL, "/episode"))
            .mode(CORS_MODE)
            .json(&cmd.episode)?
            .send()
            .await?;

        deserialize_response_null(response).await
    }

    /// エピソードを編集
    pub async fn edit_episode<'a>(
        cmd: episode_commands::EditEpisodeCommand<'_>,
    ) -> Result<(), AppFrontError> {
        let response = Request::patch(&format!("{}{}", API_BASE_URL, "/episode"))
            .mode(CORS_MODE)
            .json(&cmd.episode)?
            .send()
            .await?;

        deserialize_response_null(response).await
    }

    /// 全てのエピソードを取得。
    pub async fn all_episodes(
        _cmd: episode_commands::AllEpisodesCommand,
    ) -> Result<Vec<Episode>, AppFrontError> {
        let response = Request::get(&format!("{}{}", API_BASE_URL, "/episode"))
            .mode(CORS_MODE)
            .send()
            .await?;

        deserialize_response(response).await
    }

    /// Dateで降順・idで昇順で並べたエピソードを`length`分取得。
    pub async fn order_by_date_range_episodes(
        cmd: episode_commands::OrderByDateRangeEpisodesCommand,
    ) -> Result<Vec<Episode>, AppFrontError> {
        let query_string = format!("?sort_type=date&start={}&end={}", cmd.start, cmd.end);
        let response = Request::get(&format!(
            "{}{}{}",
            API_BASE_URL, "/episode/query", query_string
        ))
        .mode(CORS_MODE)
        .send()
        .await?;

        deserialize_response(response).await
    }

    /// エピソードを削除。
    pub async fn remove_episode(
        cmd: episode_commands::RemoveEpisodeCommand,
    ) -> Result<(), AppFrontError> {
        let response = Request::delete(&format!("{}{}{}", API_BASE_URL, "/episode/", cmd.id))
            .mode(CORS_MODE)
            .send()
            .await?;

        deserialize_response_null(response).await
    }
}

#[cfg(feature = "fake")]
mod fake {
    use crate::commands::episode_commands;
    use crate::AppFrontError;
    use domain::episode::Episode;
    use fake::{Fake, Faker};

    /// エピソードを保存(フェイク)
    pub async fn save_episode<'a>(
        _cmd: episode_commands::SaveEpisodeCommand<'_>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    /// エピソードを編集(フェイク)
    pub async fn edit_episode<'a>(
        _cmd: episode_commands::EditEpisodeCommand<'_>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    /// 全てのエピソードを取得。(フェイク)
    pub async fn all_episodes(
        _cmd: episode_commands::AllEpisodesCommand,
    ) -> Result<Vec<Episode>, AppFrontError> {
        Ok((0..100)
            .map(|_| Faker.fake::<Episode>())
            .collect::<Vec<_>>())
    }
    /// Dateで降順・idで昇順で並べたエピソードを`length`分取得。(フェイク)
    pub async fn order_by_date_range_episodes(
        _cmd: episode_commands::OrderByDateRangeEpisodesCommand,
    ) -> Result<Vec<Episode>, AppFrontError> {
        Ok((0..50).map(|_| Faker.fake::<Episode>()).collect::<Vec<_>>())
    }
    /// エピソードを削除。(フェイク)
    pub async fn remove_episode(
        _cmd: episode_commands::RemoveEpisodeCommand,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }
}

#[cfg(all(test, not(feature = "integration_test")))]
mod test {
    use crate::commands::episode_commands;
    use crate::AppFrontError;
    use domain::episode::{Episode, EpisodeId};
    use domain::Date;
    use fake::{Fake, Faker};
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_save_episode() {
        let episode = Faker.fake::<Episode>();
        let cmd = episode_commands::SaveEpisodeCommand::new(&episode);

        let _res: Result<(), AppFrontError> = super::save_episode(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_edit_episode() {
        let episode = Faker.fake::<Episode>();
        let cmd = episode_commands::EditEpisodeCommand::new(&episode);

        let _res: Result<(), AppFrontError> = super::edit_episode(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_all_episodes() {
        let cmd = episode_commands::AllEpisodesCommand;
        let _res_vec: Result<Vec<Episode>, AppFrontError> = super::all_episodes(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_order_by_date_episodes() {
        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        let cmd = episode_commands::OrderByDateRangeEpisodesCommand::new(start, end);
        let _res_vec: Result<Vec<Episode>, AppFrontError> =
            super::order_by_date_range_episodes(cmd).await;
    }

    #[wasm_bindgen_test]
    async fn test_remove_by_id_episode() {
        let id = EpisodeId::generate();
        let cmd = episode_commands::RemoveEpisodeCommand::new(id);
        let _res: Result<(), AppFrontError> = super::remove_episode(cmd).await;
    }
}
