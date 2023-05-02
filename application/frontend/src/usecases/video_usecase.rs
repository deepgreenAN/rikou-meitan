#[cfg(not(feature = "fake"))]
pub use self::product::*;

#[cfg(feature = "fake")]
pub use self::fake::*;

#[cfg(not(feature = "fake"))]
mod product {
    use crate::commands::video_commands;
    use crate::{api_base_url, API_BASE_URL};
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
        cmd: video_commands::SaveVideoCommand<'_, T>,
    ) -> Result<(), AppFrontError> {
        let request = Client::new()
            .put(&format!(
                "{}/{}",
                API_BASE_URL.get_or_init(api_base_url),
                T::snake_case()
            ))
            .json(&cmd.video);

        let response = request.send().await?;

        deserialize_response_null(response).await
    }

    /// Videoを編集
    pub async fn edit_video<'a, T: VideoType>(
        cmd: video_commands::EditVideoCommand<'_, T>,
    ) -> Result<(), AppFrontError> {
        let request = Client::new()
            .patch(&format!(
                "{}/{}",
                API_BASE_URL.get_or_init(api_base_url),
                T::snake_case()
            ))
            .json(&cmd.video);

        let response = request.send().await?;

        deserialize_response_null(response).await
    }

    /// `id`を持つVideoのLikeを一つ増やす
    pub async fn increment_like<T: VideoType>(
        cmd: video_commands::IncrementLikeVideoCommand,
    ) -> Result<(), AppFrontError> {
        let request = Client::new().patch(&format!(
            "{}/{}/increment_like/{}",
            API_BASE_URL.get_or_init(api_base_url),
            T::snake_case(),
            cmd.id
        ));

        let response = request.send().await?;

        deserialize_response_null(response).await
    }

    /// 全てのVideoを取得する
    pub async fn all_videos<T: VideoType>(
        _cmd: video_commands::AllVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        let request = Client::new().get(&format!(
            "{}/{}",
            API_BASE_URL.get_or_init(api_base_url),
            T::snake_case()
        ));

        let response = request.send().await?;

        deserialize_response(response).await
    }

    /// Likeを降順・idを昇順に並べたVideoを`length`分取得
    pub async fn order_by_like_videos<T: VideoType>(
        cmd: video_commands::OrderByLikeVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        let query_string = format!("?sort_type=like&length={}", cmd.length);
        let request = Client::new().get(&format!(
            "{}/{}/query{}",
            API_BASE_URL.get_or_init(api_base_url),
            T::snake_case(),
            query_string
        ));

        let response = request.send().await?;

        deserialize_response(response).await
    }

    /// Likeを降順・idを昇順に並べた`reference`以降のVideoを`length`分取得
    pub async fn order_by_like_later_videos<'a, T: VideoType>(
        cmd: video_commands::OrderByLikeLaterVideosCommand<'_, T>,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        let query_string = format!("?sort_type=like&length={}", cmd.length);
        let query_info = QueryInfoRef::builder()
            .reference(Borrowed(cmd.reference))
            .build();

        let request = Client::new()
            .post(&format!(
                "{}/{}/query{}",
                API_BASE_URL.get_or_init(api_base_url),
                T::snake_case(),
                query_string
            ))
            .json(&query_info);

        let response = request.send().await?;

        deserialize_response(response).await
    }

    /// dateを降順・idを昇順に並べたVideoを`length`分取得
    pub async fn order_by_date_videos<T: VideoType>(
        cmd: video_commands::OrderByDateVideosCommand,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        let query_string = format!("?sort_type=date&length={}", cmd.length);

        let request = Client::new().get(&format!(
            "{}/{}/query{}",
            API_BASE_URL.get_or_init(api_base_url),
            T::snake_case(),
            query_string
        ));

        let response = request.send().await?;

        deserialize_response(response).await
    }

    /// dateを降順・idをしょうじゅんに並べた`reference`以降のVideoを`length`分取得
    pub async fn order_by_date_later_videos<'a, T: VideoType>(
        cmd: video_commands::OrderByDateLaterVideosCommand<'_, T>,
    ) -> Result<Vec<Video<T>>, AppFrontError> {
        let query_string = format!("?sort_type=date&length={}", cmd.length);
        let query_info = QueryInfoRef::builder()
            .reference(Borrowed(cmd.reference))
            .build();

        let request = Client::new()
            .post(&format!(
                "{}/{}/query{}",
                API_BASE_URL.get_or_init(api_base_url),
                T::snake_case(),
                query_string
            ))
            .json(&query_info);

        let response = request.send().await?;

        deserialize_response(response).await
    }

    /// `id`を持つVideoを削除
    pub async fn remove_video<T: VideoType>(
        cmd: video_commands::RemoveVideoCommand,
    ) -> Result<(), AppFrontError> {
        let request = Client::new().delete(&format!(
            "{}/{}/{}",
            API_BASE_URL.get_or_init(api_base_url),
            T::snake_case(),
            cmd.id
        ));

        let response = request.send().await?;

        deserialize_response_null(response).await
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
    pub async fn increment_like<T: VideoType>(
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

#[cfg(all(test, not(feature = "integration_test")))]
mod test {
    use crate::commands::video_commands;
    use crate::AppFrontError;
    use domain::video::{Original, Video, VideoId};
    use fake::{Fake, Faker};

    #[allow(dead_code)]
    async fn test_save_video() {
        let video = Faker.fake::<Video<Original>>();

        let cmd = video_commands::SaveVideoCommand::new(&video);
        let _res: Result<(), AppFrontError> = super::save_video(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_edit_video() {
        let video = Faker.fake::<Video<Original>>();

        let cmd = video_commands::EditVideoCommand::new(&video);
        let _res: Result<(), AppFrontError> = super::edit_video(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_increment_like_video() {
        let id = VideoId::generate();

        let cmd = video_commands::IncrementLikeVideoCommand::new(id);
        let _res: Result<(), AppFrontError> = super::increment_like::<Original>(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_all_videos() {
        let cmd = video_commands::AllVideosCommand;
        let _res: Result<Vec<Video<Original>>, AppFrontError> =
            super::all_videos::<Original>(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_order_by_like_videos() {
        let length = 100_usize;
        let cmd = video_commands::OrderByLikeVideosCommand::new(length);
        let _res: Result<Vec<Video<Original>>, AppFrontError> =
            super::order_by_like_videos(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_order_by_like_later_videos() {
        let length = 100_usize;
        let reference = Faker.fake::<Video<Original>>();
        let cmd = video_commands::OrderByLikeLaterVideosCommand::new(&reference, length);
        let _res: Result<Vec<Video<Original>>, AppFrontError> =
            super::order_by_like_later_videos(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_order_by_date_videos() {
        let length = 100_usize;
        let cmd = video_commands::OrderByDateVideosCommand::new(length);
        let _res: Result<Vec<Video<Original>>, AppFrontError> =
            super::order_by_date_videos(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_order_by_date_later_videos() {
        let length = 100_usize;
        let reference = Faker.fake::<Video<Original>>();
        let cmd = video_commands::OrderByDateLaterVideosCommand::new(&reference, length);
        let _res: Result<Vec<Video<Original>>, AppFrontError> =
            super::order_by_date_later_videos(cmd).await;
    }

    #[allow(dead_code)]
    async fn test_remove_video() {
        let id = VideoId::generate();

        let cmd = video_commands::RemoveVideoCommand::new(id);
        let _res: Result<(), AppFrontError> = super::remove_video::<Original>(cmd).await;
    }
}
