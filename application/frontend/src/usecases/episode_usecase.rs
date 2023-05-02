#[cfg(not(feature = "fake"))]
pub use self::product::*;

#[cfg(feature = "fake")]
pub use self::fake::*;

#[cfg(not(feature = "fake"))]
mod product {
    /// テストのためにuriを引数とした関数にするためのモジュール
    pub(crate) mod product_inner {
        use crate::commands::episode_commands;
        use crate::{
            utils::{deserialize_response, deserialize_response_null},
            AppFrontError,
        };
        use domain::episode::Episode;

        use reqwest::Client;

        /// エピソードを保存
        pub async fn save_episode<'a>(
            url: &str,
            cmd: episode_commands::SaveEpisodeCommand<'_>,
        ) -> Result<(), AppFrontError> {
            let request = Client::new()
                .put(&format!("{}{}", url, "/episode"))
                .json(&cmd.episode);

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        /// エピソードを編集
        pub async fn edit_episode<'a>(
            url: &str,
            cmd: episode_commands::EditEpisodeCommand<'_>,
        ) -> Result<(), AppFrontError> {
            let request = Client::new()
                .patch(&format!("{}{}", url, "/episode"))
                .json(&cmd.episode);

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        /// 全てのエピソードを取得。
        pub async fn all_episodes(
            url: &str,
            _cmd: episode_commands::AllEpisodesCommand,
        ) -> Result<Vec<Episode>, AppFrontError> {
            let request = Client::new().get(&format!("{}{}", url, "/episode"));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        /// Dateで降順・idで昇順で並べたエピソードを`length`分取得。
        pub async fn order_by_date_range_episodes(
            url: &str,
            cmd: episode_commands::OrderByDateRangeEpisodesCommand,
        ) -> Result<Vec<Episode>, AppFrontError> {
            let query_string = format!("?sort_type=date&start={}&end={}", cmd.start, cmd.end);
            let request =
                Client::new().get(&format!("{}{}{}", url, "/episode/query", query_string));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        /// エピソードを削除。
        pub async fn remove_episode(
            url: &str,
            cmd: episode_commands::RemoveEpisodeCommand,
        ) -> Result<(), AppFrontError> {
            let request = Client::new().delete(&format!("{}{}{}", url, "/episode/", cmd.id));

            let response = request.send().await?;

            deserialize_response_null(response).await
        }
    }

    use crate::commands::episode_commands;
    use crate::AppFrontError;
    use crate::{api_base_url, API_BASE_URL};
    use domain::episode::Episode;

    /// エピソードを保存
    pub async fn save_episode<'a>(
        cmd: episode_commands::SaveEpisodeCommand<'_>,
    ) -> Result<(), AppFrontError> {
        product_inner::save_episode(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// エピソードを編集
    pub async fn edit_episode<'a>(
        cmd: episode_commands::EditEpisodeCommand<'_>,
    ) -> Result<(), AppFrontError> {
        product_inner::edit_episode(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// 全てのエピソードを取得。
    pub async fn all_episodes(
        cmd: episode_commands::AllEpisodesCommand,
    ) -> Result<Vec<Episode>, AppFrontError> {
        product_inner::all_episodes(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// Dateで降順・idで昇順で並べたエピソードを`length`分取得。
    pub async fn order_by_date_range_episodes(
        cmd: episode_commands::OrderByDateRangeEpisodesCommand,
    ) -> Result<Vec<Episode>, AppFrontError> {
        product_inner::order_by_date_range_episodes(API_BASE_URL.get_or_init(api_base_url), cmd)
            .await
    }

    /// エピソードを削除。
    pub async fn remove_episode(
        cmd: episode_commands::RemoveEpisodeCommand,
    ) -> Result<(), AppFrontError> {
        product_inner::remove_episode(API_BASE_URL.get_or_init(api_base_url), cmd).await
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

#[cfg(test)]
mod test {
    /// fakeとそうでないときにシグネチャを一致させるためのコンパイル
    mod compile {
        use crate::commands::episode_commands;
        use crate::AppFrontError;
        use domain::episode::{Episode, EpisodeId};
        use domain::Date;
        use fake::{Fake, Faker};

        #[allow(dead_code)]
        async fn test_save_episode() {
            let episode = Faker.fake::<Episode>();
            let cmd = episode_commands::SaveEpisodeCommand::new(&episode);

            let _res: Result<(), AppFrontError> = super::super::save_episode(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_edit_episode() {
            let episode = Faker.fake::<Episode>();
            let cmd = episode_commands::EditEpisodeCommand::new(&episode);

            let _res: Result<(), AppFrontError> = super::super::edit_episode(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_all_episodes() {
            let cmd = episode_commands::AllEpisodesCommand;
            let _res_vec: Result<Vec<Episode>, AppFrontError> =
                super::super::all_episodes(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_order_by_date_episodes() {
            let start = Faker.fake::<Date>();
            let end = Faker.fake::<Date>();

            let cmd = episode_commands::OrderByDateRangeEpisodesCommand::new(start, end);
            let _res_vec: Result<Vec<Episode>, AppFrontError> =
                super::super::order_by_date_range_episodes(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_remove_by_id_episode() {
            let id = EpisodeId::generate();
            let cmd = episode_commands::RemoveEpisodeCommand::new(id);
            let _res: Result<(), AppFrontError> = super::super::remove_episode(cmd).await;
        }
    }

    #[cfg(not(feature = "fake"))]
    mod product_test {
        use super::super::product::product_inner;
        use crate::commands::episode_commands;
        use crate::AppFrontError;
        use common::AppCommonError;
        use domain::episode::{Episode, EpisodeId};
        use domain::Date;
        
        use fake::{Fake, Faker};
        use pretty_assertions::assert_eq;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        #[tokio::test]
        async fn test_save_episode() {
            let episode = Faker.fake::<Episode>();

            {
                // 成功した場合
                let episode = episode.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PUT"))
                    .and(path("/episode"))
                    .and(body_json(episode.clone()))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = episode_commands::SaveEpisodeCommand::new(&episode);
                    product_inner::save_episode(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let episode = episode.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PUT"))
                    .and(path("/episode"))
                    .and(body_json(episode.clone()))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::ConflictError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = episode_commands::SaveEpisodeCommand::new(&episode);
                    product_inner::save_episode(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_edit_episode() {
            let episode = Faker.fake::<Episode>();

            {
                // 成功した場合
                let episode = episode.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path("/episode"))
                    .and(body_json(episode.clone()))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = episode_commands::EditEpisodeCommand::new(&episode);
                    product_inner::edit_episode(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let episode = episode.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path("/episode"))
                    .and(body_json(episode.clone()))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::NoRecordError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = episode_commands::EditEpisodeCommand::new(&episode);
                    product_inner::edit_episode(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_all_episodes() {
            let episodes = (0..100)
                .map(|_| Faker.fake::<Episode>())
                .collect::<Vec<_>>();
            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/episode"))
                .respond_with(ResponseTemplate::new(200).set_body_json(episodes.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = episode_commands::AllEpisodesCommand;
                product_inner::all_episodes(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), episodes);
        }

        #[tokio::test]
        async fn test_order_by_date_episodes() {
            let episodes = (0..100)
                .map(|_| Faker.fake::<Episode>())
                .collect::<Vec<_>>();

            let start = Faker.fake::<Date>();
            let end = Faker.fake::<Date>();

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/episode/query"))
                .and(query_param("sort_type", "date"))
                .and(query_param("start", start))
                .and(query_param("end", end))
                .respond_with(ResponseTemplate::new(200).set_body_json(episodes.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = episode_commands::OrderByDateRangeEpisodesCommand::new(start, end);
                product_inner::order_by_date_range_episodes(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), episodes);
        }

        #[tokio::test]
        async fn test_remove_episode() {
            let id = EpisodeId::generate();

            {
                // 成功する場合
                let mock_server = MockServer::start().await;

                Mock::given(method("DELETE"))
                    .and(path(format!("/episode/{}", id)))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = episode_commands::RemoveEpisodeCommand::new(id);
                    product_inner::remove_episode(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }

            {
                // 成功する場合
                let mock_server = MockServer::start().await;

                Mock::given(method("DELETE"))
                    .and(path(format!("/episode/{}", id)))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::NoRecordError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = episode_commands::RemoveEpisodeCommand::new(id);
                    product_inner::remove_episode(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }
    }
}
