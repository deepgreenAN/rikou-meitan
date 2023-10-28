#[cfg(not(feature = "fake"))]
pub use self::product::*;

#[cfg(feature = "fake")]
pub use self::fake::*;

/// APIをチェックするためのbehavior
#[cfg(test)]
mod _behavior {
    use crate::commands::video_commands;
    use crate::AppFrontError;
    use domain::video::{Video, VideoType};

    #[cfg_attr(not(feature = "fake"), behavior::behavior(modules(super::product)))]
    // #[cfg_attr(feature = "fake", behavior::behavior(modules(super::fake)))]
    #[async_trait::async_trait]
    trait Behavior {
        /// Videoを保存
        async fn save_video<'a, T: VideoType>(
            cmd: video_commands::SaveVideoCommand<'a, T>,
        ) -> Result<(), AppFrontError>;
        /// Videoを編集
        async fn edit_video<'a, T: VideoType>(
            cmd: video_commands::EditVideoCommand<'a, T>,
        ) -> Result<(), AppFrontError>;
        /// `id`を持つVideoのLikeを一つ増やす
        async fn increment_like_video<T: VideoType>(
            cmd: video_commands::IncrementLikeVideoCommand,
        ) -> Result<(), AppFrontError>;
        /// 全てのVideoを取得する
        async fn all_videos<T: VideoType>(
            cmd: video_commands::AllVideosCommand,
        ) -> Result<Vec<Video<T>>, AppFrontError>;
        /// Likeを降順・idを昇順に並べたVideoを`length`分取得
        async fn order_by_like_videos<T: VideoType>(
            cmd: video_commands::OrderByLikeVideosCommand,
        ) -> Result<Vec<Video<T>>, AppFrontError>;
        /// Likeを降順・idを昇順に並べた`reference`以降のVideoを`length`分取得
        async fn order_by_like_later_videos<'a, T: VideoType>(
            cmd: video_commands::OrderByLikeLaterVideosCommand<'_, T>,
        ) -> Result<Vec<Video<T>>, AppFrontError>;
        /// dateを降順・idを昇順に並べたVideoを`length`分取得
        async fn order_by_date_videos<T: VideoType>(
            cmd: video_commands::OrderByDateVideosCommand,
        ) -> Result<Vec<Video<T>>, AppFrontError>;
        /// dateを降順・idをしょうじゅんに並べた`reference`以降のVideoを`length`分取得
        async fn order_by_date_later_videos<'a, T: VideoType>(
            cmd: video_commands::OrderByDateLaterVideosCommand<'_, T>,
        ) -> Result<Vec<Video<T>>, AppFrontError>;
        /// `id`を持つVideoを削除
        async fn remove_video<T: VideoType>(
            cmd: video_commands::RemoveVideoCommand,
        ) -> Result<(), AppFrontError>;
    }
}

#[cfg(not(feature = "fake"))]
mod product {
    pub(crate) mod product_inner {
        /// テストするためにurlを引数とする関数を定義するモジュール
        use crate::commands::video_commands;
        use crate::{
            utils::{deserialize_response, deserialize_response_null},
            AppFrontError,
        };
        use common::QueryInfoRef;
        use domain::video::{Video, VideoType};

        use reqwest::Client;
        use std::borrow::Cow::Borrowed;

        /// Videoを保存
        pub async fn save_video<'a, T: VideoType>(
            url: &str,
            cmd: video_commands::SaveVideoCommand<'_, T>,
        ) -> Result<(), AppFrontError> {
            let request = Client::new()
                .put(&format!("{}/{}", url, T::snake_case()))
                .json(&cmd.video);

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        /// Videoを編集
        pub async fn edit_video<'a, T: VideoType>(
            url: &str,
            cmd: video_commands::EditVideoCommand<'_, T>,
        ) -> Result<(), AppFrontError> {
            let request = Client::new()
                .patch(&format!("{}/{}", url, T::snake_case()))
                .json(&cmd.video);

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        /// `id`を持つVideoのLikeを一つ増やす
        pub async fn increment_like_video<T: VideoType>(
            url: &str,
            cmd: video_commands::IncrementLikeVideoCommand,
        ) -> Result<(), AppFrontError> {
            let request = Client::new().patch(&format!(
                "{}/{}/increment_like/{}",
                url,
                T::snake_case(),
                cmd.id
            ));

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        /// 全てのVideoを取得する
        pub async fn all_videos<T: VideoType>(
            url: &str,
            _cmd: video_commands::AllVideosCommand,
        ) -> Result<Vec<Video<T>>, AppFrontError> {
            let request = Client::new().get(&format!("{}/{}", url, T::snake_case()));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        /// Likeを降順・idを昇順に並べたVideoを`length`分取得
        pub async fn order_by_like_videos<T: VideoType>(
            url: &str,
            cmd: video_commands::OrderByLikeVideosCommand,
        ) -> Result<Vec<Video<T>>, AppFrontError> {
            let query_string = format!("?sort_type=like&length={}", cmd.length);
            let request = Client::new().get(&format!(
                "{}/{}/query{}",
                url,
                T::snake_case(),
                query_string
            ));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        /// Likeを降順・idを昇順に並べた`reference`以降のVideoを`length`分取得
        pub async fn order_by_like_later_videos<'a, T: VideoType>(
            url: &str,
            cmd: video_commands::OrderByLikeLaterVideosCommand<'_, T>,
        ) -> Result<Vec<Video<T>>, AppFrontError> {
            let query_string = format!("?sort_type=like&length={}", cmd.length);
            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(cmd.reference))
                .build();

            let request = Client::new()
                .post(&format!(
                    "{}/{}/query{}",
                    url,
                    T::snake_case(),
                    query_string
                ))
                .json(&query_info);

            let response = request.send().await?;

            deserialize_response(response).await
        }

        /// dateを降順・idを昇順に並べたVideoを`length`分取得
        pub async fn order_by_date_videos<T: VideoType>(
            url: &str,
            cmd: video_commands::OrderByDateVideosCommand,
        ) -> Result<Vec<Video<T>>, AppFrontError> {
            let query_string = format!("?sort_type=date&length={}", cmd.length);

            let request = Client::new().get(&format!(
                "{}/{}/query{}",
                url,
                T::snake_case(),
                query_string
            ));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        /// dateを降順・idをしょうじゅんに並べた`reference`以降のVideoを`length`分取得
        pub async fn order_by_date_later_videos<'a, T: VideoType>(
            url: &str,
            cmd: video_commands::OrderByDateLaterVideosCommand<'_, T>,
        ) -> Result<Vec<Video<T>>, AppFrontError> {
            let query_string = format!("?sort_type=date&length={}", cmd.length);
            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(cmd.reference))
                .build();

            let request = Client::new()
                .post(&format!(
                    "{}/{}/query{}",
                    url,
                    T::snake_case(),
                    query_string
                ))
                .json(&query_info);

            let response = request.send().await?;

            deserialize_response(response).await
        }

        /// `id`を持つVideoを削除
        pub async fn remove_video<T: VideoType>(
            url: &str,
            cmd: video_commands::RemoveVideoCommand,
        ) -> Result<(), AppFrontError> {
            let request = Client::new().delete(&format!("{}/{}/{}", url, T::snake_case(), cmd.id));

            let response = request.send().await?;

            deserialize_response_null(response).await
        }
    }

    use crate::commands::video_commands;
    use crate::AppFrontError;
    use crate::{api_base_url, API_BASE_URL};
    use domain::video::{Video, VideoType};

    /// Videoを保存
    pub async fn save_video<'a, T: VideoType>(
        cmd: video_commands::SaveVideoCommand<'_, T>,
    ) -> Result<(), AppFrontError> {
        product_inner::save_video(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// Videoを編集
    pub async fn edit_video<'a, T: VideoType>(
        cmd: video_commands::EditVideoCommand<'_, T>,
    ) -> Result<(), AppFrontError> {
        product_inner::edit_video(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// `id`を持つVideoのLikeを一つ増やす
    pub async fn increment_like_video<T: VideoType>(
        cmd: video_commands::IncrementLikeVideoCommand,
    ) -> Result<(), AppFrontError> {
        product_inner::increment_like_video::<T>(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// 全てのVideoを取得する
    pub async fn all_videos<T: VideoType>(
        cmd: video_commands::AllVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        product_inner::all_videos(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// Likeを降順・idを昇順に並べたVideoを`length`分取得
    pub async fn order_by_like_videos<T: VideoType>(
        cmd: video_commands::OrderByLikeVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        product_inner::order_by_like_videos(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// Likeを降順・idを昇順に並べた`reference`以降のVideoを`length`分取得
    pub async fn order_by_like_later_videos<'a, T: VideoType>(
        cmd: video_commands::OrderByLikeLaterVideosCommand<'_, T>,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        product_inner::order_by_like_later_videos(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// dateを降順・idを昇順に並べたVideoを`length`分取得
    pub async fn order_by_date_videos<T: VideoType>(
        cmd: video_commands::OrderByDateVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        product_inner::order_by_date_videos(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// dateを降順・idをしょうじゅんに並べた`reference`以降のVideoを`length`分取得
    pub async fn order_by_date_later_videos<'a, T: VideoType>(
        cmd: video_commands::OrderByDateLaterVideosCommand<'_, T>,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        product_inner::order_by_date_later_videos(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    /// `id`を持つVideoを削除
    pub async fn remove_video<T: VideoType>(
        cmd: video_commands::RemoveVideoCommand,
    ) -> Result<(), AppFrontError> {
        product_inner::remove_video::<T>(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }
}

#[cfg(feature = "fake")]
mod fake {
    use crate::commands::video_commands;
    use crate::AppFrontError;
    use domain::video::{Video, VideoType};

    use fake::{Fake, Faker};

    /// Videoを保存(フェイク)
    pub async fn save_video<'a, T: VideoType>(
        _cmd: video_commands::SaveVideoCommand<'_, T>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    /// Videoを編集(フェイク)
    pub async fn edit_video<'a, T: VideoType>(
        _cmd: video_commands::EditVideoCommand<'_, T>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    /// `id`を持つVideoのLikeを一つ増やす(フェイク)
    pub async fn increment_like_video<T: VideoType>(
        _cmd: video_commands::IncrementLikeVideoCommand,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    /// 全てのVideoを取得する(フェイク)
    pub async fn all_videos<T: VideoType>(
        _cmd: video_commands::AllVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        Ok((0..100)
            .map(|_| Faker.fake::<Video<T>>())
            .collect::<Vec<_>>())
    }

    /// Likeを降順・idを昇順に並べたVideoを`length`分取得(フェイク)
    pub async fn order_by_like_videos<T: VideoType>(
        cmd: video_commands::OrderByLikeVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<Video<T>>())
            .collect::<Vec<_>>())
    }

    /// Likeを降順・idを昇順に並べた`reference`以降のVideoを`length`分取得(フェイク)
    pub async fn order_by_like_later_videos<'a, T: VideoType>(
        cmd: video_commands::OrderByLikeLaterVideosCommand<'_, T>,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<Video<T>>())
            .collect::<Vec<_>>())
    }

    /// dateを降順・idを昇順に並べたVideoを`length`分取得(フェイク)
    pub async fn order_by_date_videos<T: VideoType>(
        cmd: video_commands::OrderByDateVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<Video<T>>())
            .collect::<Vec<_>>())
    }

    /// dateを降順・idをしょうじゅんに並べた`reference`以降のVideoを`length`分取得(フェイク)
    pub async fn order_by_date_later_videos<'a, T: VideoType>(
        cmd: video_commands::OrderByDateLaterVideosCommand<'_, T>,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<Video<T>>())
            .collect::<Vec<_>>())
    }

    /// `id`を持つVideoを削除(フェイク)
    pub async fn remove_video<T: VideoType>(
        _cmd: video_commands::RemoveVideoCommand,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[cfg(not(feature = "fake"))]
    mod product_test {
        use super::super::product::product_inner;
        use crate::commands::video_commands;
        use crate::AppFrontError;
        use common::{AppCommonError, QueryInfoRef};
        use domain::video::{Original, Video, VideoId};

        use fake::{Fake, Faker};
        use pretty_assertions::assert_eq;
        use std::borrow::Cow::Borrowed;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        #[tokio::test]
        async fn test_save_video() {
            let video = Faker.fake::<Video<Original>>();

            {
                // 成功した場合
                let video = video.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PUT"))
                    .and(path("/original"))
                    .and(body_json(video.clone()))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::SaveVideoCommand::new(&video);
                    product_inner::save_video(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let video = video.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PUT"))
                    .and(path("/original"))
                    .and(body_json(video.clone()))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::ConflictError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::SaveVideoCommand::new(&video);
                    product_inner::save_video(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_edit_video() {
            let video = Faker.fake::<Video<Original>>();

            {
                // 成功した場合
                let video = video.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path("/original"))
                    .and(body_json(video.clone()))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::EditVideoCommand::new(&video);
                    product_inner::edit_video(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let video = video.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path("/original"))
                    .and(body_json(video.clone()))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::ConflictError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::EditVideoCommand::new(&video);
                    product_inner::edit_video(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_increment_like_video() {
            let id = VideoId::generate();

            {
                // 成功した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path(format!("/original/increment_like/{}", id)))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::IncrementLikeVideoCommand::new(id);
                    product_inner::increment_like_video::<Original>(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path(format!("/original/increment_like/{}", id)))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::NoRecordError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::IncrementLikeVideoCommand::new(id);
                    product_inner::increment_like_video::<Original>(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_all_videos() {
            let videos = (0..100)
                .map(|_| Faker.fake::<Video<Original>>())
                .collect::<Vec<_>>();

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/original"))
                .respond_with(ResponseTemplate::new(200).set_body_json(videos.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = video_commands::AllVideosCommand;
                product_inner::all_videos(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), videos);
        }

        #[tokio::test]
        async fn test_order_by_like_videos() {
            let videos = (0..100)
                .map(|_| Faker.fake::<Video<Original>>())
                .collect::<Vec<_>>();

            let length = 10_usize;

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/original/query"))
                .and(query_param("sort_type", "like"))
                .and(query_param("length", length.to_string()))
                .respond_with(ResponseTemplate::new(200).set_body_json(videos.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = video_commands::OrderByLikeVideosCommand::new(length);
                product_inner::order_by_like_videos::<Original>(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), videos);
        }

        #[tokio::test]
        async fn test_order_by_like_later_videos() {
            let videos = (0..100)
                .map(|_| Faker.fake::<Video<Original>>())
                .collect::<Vec<_>>();

            let length = 10_usize;
            let reference = Faker.fake::<Video<Original>>();

            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(&reference))
                .build();

            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/original/query"))
                .and(query_param("sort_type", "like"))
                .and(query_param("length", length.to_string()))
                .and(body_json(query_info))
                .respond_with(ResponseTemplate::new(200).set_body_json(videos.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = video_commands::OrderByLikeLaterVideosCommand::new(&reference, length);
                product_inner::order_by_like_later_videos(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), videos);
        }

        #[tokio::test]
        async fn test_order_by_date_videos() {
            let videos = (0..100)
                .map(|_| Faker.fake::<Video<Original>>())
                .collect::<Vec<_>>();

            let length = 10_usize;

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/original/query"))
                .and(query_param("sort_type", "date"))
                .and(query_param("length", length.to_string()))
                .respond_with(ResponseTemplate::new(200).set_body_json(videos.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = video_commands::OrderByDateVideosCommand::new(length);
                product_inner::order_by_date_videos::<Original>(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), videos);
        }

        #[tokio::test]
        async fn test_order_by_date_later_videos() {
            let videos = (0..100)
                .map(|_| Faker.fake::<Video<Original>>())
                .collect::<Vec<_>>();

            let length = 10_usize;
            let reference = Faker.fake::<Video<Original>>();

            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(&reference))
                .build();

            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/original/query"))
                .and(query_param("sort_type", "date"))
                .and(query_param("length", length.to_string()))
                .and(body_json(query_info))
                .respond_with(ResponseTemplate::new(200).set_body_json(videos.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = video_commands::OrderByDateLaterVideosCommand::new(&reference, length);
                product_inner::order_by_date_later_videos(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), videos);
        }

        #[tokio::test]
        async fn test_remove_video() {
            let id = VideoId::generate();

            {
                // 成功した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("DELETE"))
                    .and(path(format!("/original/{}", id)))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::RemoveVideoCommand::new(id);
                    product_inner::remove_video::<Original>(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("DELETE"))
                    .and(path(format!("/original/{}", id)))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::NoRecordError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = video_commands::RemoveVideoCommand::new(id);
                    product_inner::remove_video::<Original>(&mock_server.uri(), cmd).await
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
