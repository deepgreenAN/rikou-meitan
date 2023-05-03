#[cfg(not(feature = "fake"))]
pub use self::product::*;

#[cfg(feature = "fake")]
pub use self::fake::*;

#[cfg(not(feature = "fake"))]
mod product {

    pub(crate) mod product_inner {
        use crate::commands::movie_clip_commands;
        use crate::{
            utils::{deserialize_response, deserialize_response_null},
            AppFrontError,
        };
        use common::QueryInfoRef;
        use domain::movie_clip::MovieClip;

        use reqwest::Client;
        use std::borrow::Cow::Borrowed;

        pub async fn save_movie_clip<'a>(
            url: &str,
            cmd: movie_clip_commands::SaveMovieClipCommand<'_>,
        ) -> Result<(), AppFrontError> {
            let request = Client::new()
                .put(&format!("{}{}", url, "/movie_clip"))
                .json(&cmd.movie_clip);

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        pub async fn edit_movie_clip<'a>(
            url: &str,
            cmd: movie_clip_commands::EditMovieClipCommand<'_>,
        ) -> Result<(), AppFrontError> {
            let request = Client::new()
                .patch(&format!("{}{}", url, "/movie_clip"))
                .json(&cmd.movie_clip);

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        pub async fn increment_like_movie_clip(
            url: &str,
            cmd: movie_clip_commands::IncrementLikeMovieClipCommand,
        ) -> Result<(), AppFrontError> {
            let request = Client::new().patch(&format!(
                "{}{}{}",
                url, "/movie_clip/increment_like/", cmd.id
            ));

            let response = request.send().await?;

            deserialize_response_null(response).await
        }

        pub async fn all_movie_clips(
            url: &str,
            _cmd: movie_clip_commands::AllMovieClipsCommand,
        ) -> Result<Vec<MovieClip>, AppFrontError> {
            let request = Client::new().get(&format!("{}{}", url, "/movie_clip"));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        pub async fn order_by_like_movie_clips(
            url: &str,
            cmd: movie_clip_commands::OrderByLikeMovieClipsCommand,
        ) -> Result<Vec<MovieClip>, AppFrontError> {
            let query_string = format!("?sort_type=like&length={}", cmd.length);
            let request =
                Client::new().get(&format!("{}{}{}", url, "/movie_clip/query", query_string));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        pub async fn order_by_like_later_movie_clips<'a>(
            url: &str,
            cmd: movie_clip_commands::OrderByLikeLaterMovieClipsCommand<'_>,
        ) -> Result<Vec<MovieClip>, AppFrontError> {
            let query_string = format!("?sort_type=like&length={}", cmd.length);
            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(cmd.reference))
                .build();

            let request = Client::new()
                .post(&format!("{}{}{}", url, "/movie_clip/query", query_string))
                .json(&query_info);

            let response = request.send().await?;

            deserialize_response(response).await
        }

        pub async fn order_by_create_date_range_movie_clips(
            url: &str,
            cmd: movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand,
        ) -> Result<Vec<MovieClip>, AppFrontError> {
            let query_string =
                format!("?sort_type=create_date&start={}&end={}", cmd.start, cmd.end);
            let request =
                Client::new().get(&format!("{}{}{}", url, "/movie_clip/query", query_string));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        pub async fn order_by_create_date_movie_clips(
            url: &str,
            cmd: movie_clip_commands::OrderByCreateDateMovieClipsCommand,
        ) -> Result<Vec<MovieClip>, AppFrontError> {
            let query_string = format!("?sort_type=create_date&length={}", cmd.length);
            let request =
                Client::new().get(&format!("{}{}{}", url, "/movie_clip/query", query_string));

            let response = request.send().await?;

            deserialize_response(response).await
        }

        pub async fn order_by_create_date_later_movie_clips<'a>(
            url: &str,
            cmd: movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand<'_>,
        ) -> Result<Vec<MovieClip>, AppFrontError> {
            let query_string = format!("?sort_type=create_date&length={}", cmd.length);
            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(cmd.reference))
                .build();

            let request = Client::new()
                .post(&format!("{}{}{}", url, "/movie_clip/query", query_string))
                .json(&query_info);

            let response = request.send().await?;

            deserialize_response(response).await
        }

        pub async fn remove_movie_clip(
            url: &str,
            cmd: movie_clip_commands::RemoveMovieClipCommand,
        ) -> Result<(), AppFrontError> {
            let request = Client::new().delete(&format!("{}{}{}", url, "/movie_clip/", cmd.id));

            let response = request.send().await?;

            deserialize_response_null(response).await
        }
    }

    use crate::commands::movie_clip_commands;
    use crate::AppFrontError;
    use crate::{api_base_url, API_BASE_URL};
    use domain::movie_clip::MovieClip;

    pub async fn save_movie_clip<'a>(
        cmd: movie_clip_commands::SaveMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        product_inner::save_movie_clip(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    pub async fn edit_movie_clip<'a>(
        cmd: movie_clip_commands::EditMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        product_inner::edit_movie_clip(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    pub async fn increment_like_movie_clip(
        cmd: movie_clip_commands::IncrementLikeMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        product_inner::increment_like_movie_clip(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    pub async fn all_movie_clips(
        cmd: movie_clip_commands::AllMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        product_inner::all_movie_clips(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    pub async fn order_by_like_movie_clips(
        cmd: movie_clip_commands::OrderByLikeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        product_inner::order_by_like_movie_clips(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }

    pub async fn order_by_like_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByLikeLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        product_inner::order_by_like_later_movie_clips(API_BASE_URL.get_or_init(api_base_url), cmd)
            .await
    }

    pub async fn order_by_create_date_range_movie_clips(
        cmd: movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        product_inner::order_by_create_date_range_movie_clips(
            API_BASE_URL.get_or_init(api_base_url),
            cmd,
        )
        .await
    }

    pub async fn order_by_create_date_movie_clips(
        cmd: movie_clip_commands::OrderByCreateDateMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        product_inner::order_by_create_date_movie_clips(API_BASE_URL.get_or_init(api_base_url), cmd)
            .await
    }

    pub async fn order_by_create_date_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        product_inner::order_by_create_date_later_movie_clips(
            API_BASE_URL.get_or_init(api_base_url),
            cmd,
        )
        .await
    }

    pub async fn remove_movie_clip(
        cmd: movie_clip_commands::RemoveMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        product_inner::remove_movie_clip(API_BASE_URL.get_or_init(api_base_url), cmd).await
    }
}

#[cfg(feature = "fake")]
mod fake {
    use crate::commands::movie_clip_commands;
    use crate::AppFrontError;
    use domain::movie_clip::MovieClip;

    use fake::{Fake, Faker};

    pub async fn save_movie_clip<'a>(
        _cmd: movie_clip_commands::SaveMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    pub async fn edit_movie_clip<'a>(
        _cmd: movie_clip_commands::EditMovieClipCommand<'_>,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    pub async fn increment_like_movie_clip(
        _cmd: movie_clip_commands::IncrementLikeMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }

    pub async fn all_movie_clips(
        _cmd: movie_clip_commands::AllMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..100)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_like_movie_clips(
        cmd: movie_clip_commands::OrderByLikeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_like_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByLikeLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_create_date_range_movie_clips(
        _cmd: movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..50)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_create_date_movie_clips(
        cmd: movie_clip_commands::OrderByCreateDateMovieClipsCommand,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn order_by_create_date_later_movie_clips<'a>(
        cmd: movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand<'_>,
    ) -> Result<Vec<MovieClip>, AppFrontError> {
        Ok((0..cmd.length)
            .map(|_| Faker.fake::<MovieClip>())
            .collect::<Vec<_>>())
    }

    pub async fn remove_movie_clip(
        _cmd: movie_clip_commands::RemoveMovieClipCommand,
    ) -> Result<(), AppFrontError> {
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    /// fakeのときのそうでないときにシグネチャを一緒にするためのコンパイル
    mod compile {
        use crate::commands::movie_clip_commands;
        use crate::AppFrontError;
        use domain::movie_clip::{MovieClip, MovieClipId};
        use domain::Date;
        use fake::{Fake, Faker};

        #[allow(dead_code)]
        async fn test_save_movie_clip() {
            let movie_clip = Faker.fake::<MovieClip>();

            let cmd = movie_clip_commands::SaveMovieClipCommand::new(&movie_clip);
            let _res: Result<(), AppFrontError> = super::super::save_movie_clip(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_edit_movie_clip() {
            let movie_clip = Faker.fake::<MovieClip>();

            let cmd = movie_clip_commands::EditMovieClipCommand::new(&movie_clip);
            let _res: Result<(), AppFrontError> = super::super::edit_movie_clip(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_increment_like_movie_clip() {
            let movie_clip = Faker.fake::<MovieClip>();

            let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(movie_clip.id());
            let _res: Result<(), AppFrontError> =
                super::super::increment_like_movie_clip(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_all_movie_clips() {
            let cmd = movie_clip_commands::AllMovieClipsCommand;
            let _res: Result<Vec<MovieClip>, AppFrontError> =
                super::super::all_movie_clips(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_order_by_like_movie_clips() {
            let length = 0;
            let cmd = movie_clip_commands::OrderByLikeMovieClipsCommand::new(length);
            let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
                super::super::order_by_like_movie_clips(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_order_by_like_later_movie_clips() {
            let length = 0;
            let reference = Faker.fake::<MovieClip>();
            let cmd =
                movie_clip_commands::OrderByLikeLaterMovieClipsCommand::new(&reference, length);
            let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
                super::super::order_by_like_later_movie_clips(cmd).await;
        }
        #[allow(dead_code)]
        async fn test_order_by_create_date_range_movie_clips() {
            let start = Faker.fake::<Date>();
            let end = Faker.fake::<Date>();

            let cmd = movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand::new(start, end);
            let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
                super::super::order_by_create_date_range_movie_clips(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_order_by_create_date_movie_clips() {
            let length = 0;
            let cmd = movie_clip_commands::OrderByCreateDateMovieClipsCommand::new(length);
            let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
                super::super::order_by_create_date_movie_clips(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_order_by_create_date_later_movie_clips() {
            let length = 0;
            let reference = Faker.fake::<MovieClip>();
            let cmd = movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand::new(
                &reference, length,
            );
            let _res_vec: Result<Vec<MovieClip>, AppFrontError> =
                super::super::order_by_create_date_later_movie_clips(cmd).await;
        }

        #[allow(dead_code)]
        async fn test_remove_by_id_movie_clip() {
            let id = MovieClipId::generate();
            let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id);
            let _res: Result<(), AppFrontError> = super::super::remove_movie_clip(cmd).await;
        }
    }

    #[cfg(not(feature = "fake"))]
    mod product_test {
        use super::super::product::product_inner;
        use crate::commands::movie_clip_commands;
        use crate::AppFrontError;
        use common::{AppCommonError, QueryInfoRef};
        use domain::movie_clip::{MovieClip, MovieClipId};
        use domain::Date;

        use fake::{Fake, Faker};
        use pretty_assertions::assert_eq;
        use std::borrow::Cow::Borrowed;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        #[tokio::test]
        async fn test_save_movie_clip() {
            let clip = Faker.fake::<MovieClip>();

            {
                // 成功した場合
                let clip = clip.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PUT"))
                    .and(path("/movie_clip"))
                    .and(body_json(clip.clone()))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::SaveMovieClipCommand::new(&clip);
                    product_inner::save_movie_clip(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let clip = clip.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PUT"))
                    .and(path("/movie_clip"))
                    .and(body_json(clip.clone()))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::ConflictError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::SaveMovieClipCommand::new(&clip);
                    product_inner::save_movie_clip(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_edit_movie_clip() {
            let clip = Faker.fake::<MovieClip>();

            {
                // 成功した場合
                let clip = clip.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path("/movie_clip"))
                    .and(body_json(clip.clone()))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::EditMovieClipCommand::new(&clip);
                    product_inner::edit_movie_clip(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let clip = clip.clone();
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path("/movie_clip"))
                    .and(body_json(clip.clone()))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::NoRecordError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::EditMovieClipCommand::new(&clip);
                    product_inner::edit_movie_clip(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_increment_like_movie_clip() {
            let id = MovieClipId::generate();

            {
                // 成功した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path(format!("/movie_clip/increment_like/{}", id)))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(id);
                    product_inner::increment_like_movie_clip(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("PATCH"))
                    .and(path(format!("/movie_clip/increment_like/{}", id)))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::NoRecordError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::IncrementLikeMovieClipCommand::new(id);
                    product_inner::increment_like_movie_clip(&mock_server.uri(), cmd).await
                };

                assert!(
                    matches!(res, Err(AppFrontError::CommonError(_))),
                    "{:?}",
                    res.unwrap_err()
                );
            }
        }

        #[tokio::test]
        async fn test_all_movie_clips() {
            let clips = (0..100)
                .map(|_| Faker.fake::<MovieClip>())
                .collect::<Vec<_>>();

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/movie_clip"))
                .respond_with(ResponseTemplate::new(200).set_body_json(clips.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = movie_clip_commands::AllMovieClipsCommand;
                product_inner::all_movie_clips(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), clips);
        }

        #[tokio::test]
        async fn test_order_by_like_movie_clips() {
            let clips = (0..100)
                .map(|_| Faker.fake::<MovieClip>())
                .collect::<Vec<_>>();

            let length = 10_usize;

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/movie_clip/query"))
                .and(query_param("sort_type", "like"))
                .and(query_param("length", length.to_string()))
                .respond_with(ResponseTemplate::new(200).set_body_json(clips.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = movie_clip_commands::OrderByLikeMovieClipsCommand::new(length);
                product_inner::order_by_like_movie_clips(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), clips);
        }

        #[tokio::test]
        async fn test_order_by_like_later_movie_clips() {
            let clips = (0..100)
                .map(|_| Faker.fake::<MovieClip>())
                .collect::<Vec<_>>();

            let length = 10_usize;
            let reference = Faker.fake::<MovieClip>();

            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(&reference))
                .build();

            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/movie_clip/query"))
                .and(query_param("sort_type", "like"))
                .and(query_param("length", length.to_string()))
                .and(body_json(query_info))
                .respond_with(ResponseTemplate::new(200).set_body_json(clips.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd =
                    movie_clip_commands::OrderByLikeLaterMovieClipsCommand::new(&reference, length);
                product_inner::order_by_like_later_movie_clips(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), clips);
        }

        #[tokio::test]
        async fn test_order_by_create_date_range_movie_clips() {
            let clips = (0..100)
                .map(|_| Faker.fake::<MovieClip>())
                .collect::<Vec<_>>();

            let start = Faker.fake::<Date>();
            let end = Faker.fake::<Date>();

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/movie_clip/query"))
                .and(query_param("sort_type", "create_date"))
                .and(query_param("start", start.to_string()))
                .and(query_param("end", end.to_string()))
                .respond_with(ResponseTemplate::new(200).set_body_json(clips.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd =
                    movie_clip_commands::OrderByCreateDateRangeMovieClipsCommand::new(start, end);
                product_inner::order_by_create_date_range_movie_clips(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), clips);
        }

        #[tokio::test]
        async fn test_order_by_create_date_movie_clips() {
            let clips = (0..100)
                .map(|_| Faker.fake::<MovieClip>())
                .collect::<Vec<_>>();

            let length = 10_usize;

            let mock_server = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/movie_clip/query"))
                .and(query_param("sort_type", "create_date"))
                .and(query_param("length", length.to_string()))
                .respond_with(ResponseTemplate::new(200).set_body_json(clips.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = movie_clip_commands::OrderByCreateDateMovieClipsCommand::new(length);
                product_inner::order_by_create_date_movie_clips(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), clips);
        }

        #[tokio::test]
        async fn test_order_by_create_date_later_movie_clips() {
            let clips = (0..100)
                .map(|_| Faker.fake::<MovieClip>())
                .collect::<Vec<_>>();

            let length = 10_usize;
            let reference = Faker.fake::<MovieClip>();

            let query_info = QueryInfoRef::builder()
                .reference(Borrowed(&reference))
                .build();

            let mock_server = MockServer::start().await;

            Mock::given(method("POST"))
                .and(path("/movie_clip/query"))
                .and(query_param("sort_type", "create_date"))
                .and(query_param("length", length.to_string()))
                .and(body_json(query_info))
                .respond_with(ResponseTemplate::new(200).set_body_json(clips.clone()))
                .mount(&mock_server)
                .await;

            let res = {
                let cmd = movie_clip_commands::OrderByCreateDateLaterMovieClipsCommand::new(
                    &reference, length,
                );
                product_inner::order_by_create_date_later_movie_clips(&mock_server.uri(), cmd).await
            };

            assert_eq!(res.unwrap(), clips);
        }

        #[tokio::test]
        async fn test_remove_movie_clip() {
            let id = MovieClipId::generate();

            {
                // 成功した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("DELETE"))
                    .and(path(format!("/movie_clip/{}", id)))
                    .respond_with(ResponseTemplate::new(200))
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id);
                    product_inner::remove_movie_clip(&mock_server.uri(), cmd).await
                };

                assert!(matches!(res, Ok(_)), "{:?}", res.unwrap_err());
            }
            {
                // 失敗した場合
                let mock_server = MockServer::start().await;

                Mock::given(method("DELETE"))
                    .and(path(format!("/movie_clip/{}", id)))
                    .respond_with(
                        ResponseTemplate::new(500).set_body_json(AppCommonError::NoRecordError),
                    )
                    .mount(&mock_server)
                    .await;

                let res = {
                    let cmd = movie_clip_commands::RemoveMovieClipCommand::new(id);
                    product_inner::remove_movie_clip(&mock_server.uri(), cmd).await
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
