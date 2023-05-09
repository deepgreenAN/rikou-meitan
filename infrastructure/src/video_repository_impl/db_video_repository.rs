use crate::InfraError;
use domain::video::{Video, VideoId, VideoType};
use domain::VideoRepository;

use async_trait::async_trait;
use sqlx::PgPool;
use std::marker::PhantomData;

// -------------------------------------------------------------------------------------------------
// video_sql_runner

/// videoに関するSQLのランナーモジュール
mod video_sql_runner {
    use crate::InfraError;
    use domain::video::{Video, VideoId, VideoType};
    use sqlx::{PgConnection, Postgres};

    /// Video<T>を一つ保存
    pub async fn save<T: VideoType>(
        conn: &mut PgConnection,
        video: Video<T>,
    ) -> Result<(), InfraError> {
        sqlx::query(
            r#"
INSERT INTO videos (title, "url", id, "date", author, "like", video_type)
VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        )
        .bind(video.title().to_string())
        .bind(video.url().to_string())
        .bind(video.id().to_uuid())
        .bind(video.date().to_chrono()?)
        .bind(video.author().to_string())
        .bind(video.like() as i32)
        .bind(T::default().to_string())
        .execute(conn)
        .await
        .map_err(|_| InfraError::ConflictError)?;

        Ok(())
    }

    /// Video<T>を一つ編集
    pub async fn edit<T: VideoType>(
        conn: &mut PgConnection,
        video: Video<T>,
    ) -> Result<(), InfraError> {
        sqlx::query(
            r#"
UPDATE videos SET title = $1, "url" = $2, "date" = $3, author = $4
WHERE video_type = $5 AND id = $6 RETURNING *
        "#,
        )
        .bind(video.title().to_string())
        .bind(video.url().to_string())
        .bind(video.date().to_chrono()?)
        .bind(video.author().to_string())
        .bind(T::default().to_string())
        .bind(video.id().to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::NoRecordError)?;

        Ok(())
    }

    /// `id`を持つVideo<T>のLikeを一つ増やす
    pub async fn increment_like<T: VideoType>(
        conn: &mut PgConnection,
        id: VideoId,
    ) -> Result<(), InfraError> {
        sqlx::query(
            r#"
UPDATE videos SET "like" = "like" + 1 WHERE video_type = $1 AND id = $2 RETURNING *
        "#,
        )
        .bind(T::default().to_string())
        .bind(id.to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::NoRecordError)?;

        Ok(())
    }

    /// 全てのVideo<T>を取得
    pub async fn all<T: VideoType>(conn: &mut PgConnection) -> Result<Vec<Video<T>>, InfraError> {
        let all_videos = sqlx::query_as::<Postgres, Video<T>>(
            r#"
SELECT * FROM videos WHERE video_type = $1
        "#,
        )
        .bind(T::default().to_string())
        .fetch_all(conn)
        .await?;

        Ok(all_videos)
    }

    /// Likeを降順に`length`分のVideo<T>を取得．Likeが同じ場合はidを昇順で並べる．
    pub async fn order_by_like<T: VideoType>(
        conn: &mut PgConnection,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let ordered_videos = sqlx::query_as::<Postgres, Video<T>>(
            r#"
SELECT * FROM videos WHERE video_type = $1 ORDER BY "like" DESC, id ASC LIMIT $2
        "#,
        )
        .bind(T::default().to_string())
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_videos)
    }

    /// Likeを降順・idを昇順に`reference`以降のVideo<T>を`length`分取得．
    pub async fn order_by_like_later<T: VideoType>(
        conn: &mut PgConnection,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let ordered_videos = sqlx::query_as::<Postgres, Video<T>>(
            r#"
SELECT * FROM videos WHERE video_type = $1 AND ($2 > "like" OR ($2 = "like" AND $3 < id)) ORDER BY "like" DESC, id ASC LIMIT $4
        "#,
        )
        .bind(T::default().to_string())
        .bind(reference.like() as i32)
        .bind(reference.id().to_uuid())
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_videos)
    }

    /// dateを降順に`length`分の`Video<T>`を取得．dateが同じ場合はidを昇順で並べる．
    pub async fn order_by_date<T: VideoType>(
        conn: &mut PgConnection,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let ordered_videos = sqlx::query_as::<Postgres, Video<T>>(
            r#"
SELECT * FROM videos WHERE video_type = $1 ORDER BY "date" DESC, id ASC LIMIT $2
        "#,
        )
        .bind(T::default().to_string())
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_videos)
    }

    /// dateを降順・idを昇順に`reference`以降の`Video<T>`を`length`分取得．
    pub async fn order_by_date_later<T: VideoType>(
        conn: &mut PgConnection,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let ordered_videos = sqlx::query_as::<Postgres, Video<T>>(
            r#"
SELECT * FROM videos WHERE video_type = $1 AND ($2 > "date" OR ($2 = "date" AND $3 < id)) ORDER BY "date" DESC, id ASC LIMIT $4
        "#,
        )
        .bind(T::default().to_string())
        .bind(reference.date().to_chrono()?)
        .bind(reference.id().to_uuid())
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_videos)
    }

    /// `id`を持つVideo<T>を削除する．
    pub async fn remove(conn: &mut PgConnection, id: VideoId) -> Result<(), InfraError> {
        sqlx::query(
            r#"
DELETE FROM videos WHERE id = $1 RETURNING *
            "#,
        )
        .bind(id.to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::NoRecordError)?;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// VideoPgDbRepository

/// VideoのPostgreSQLのリポジトリ
#[derive(Debug, Clone)]
pub struct VideoPgDbRepository<T: VideoType> {
    pool: PgPool,
    video_type: PhantomData<T>,
}

impl<T: VideoType> VideoPgDbRepository<T> {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            video_type: PhantomData,
        }
    }
}

#[async_trait]
impl<T: VideoType> VideoRepository<T> for VideoPgDbRepository<T> {
    type Error = InfraError;
    async fn save(&self, video: Video<T>) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        video_sql_runner::save(&mut conn, video).await?;
        Ok(())
    }
    async fn edit(&self, new_video: Video<T>) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        video_sql_runner::edit(&mut conn, new_video).await?;
        Ok(())
    }
    async fn increment_like(&self, id: VideoId) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        video_sql_runner::increment_like::<T>(&mut conn, id).await?;
        Ok(())
    }
    async fn all(&self) -> Result<Vec<Video<T>>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let clips = video_sql_runner::all(&mut conn).await?;
        Ok(clips)
    }
    async fn order_by_like(&self, length: usize) -> Result<Vec<Video<T>>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let clips = video_sql_runner::order_by_like(&mut conn, length).await?;
        Ok(clips)
    }
    async fn order_by_like_later(
        &self,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let clips = video_sql_runner::order_by_like_later(&mut conn, reference, length).await?;
        Ok(clips)
    }
    async fn order_by_date(&self, length: usize) -> Result<Vec<Video<T>>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let clips = video_sql_runner::order_by_date(&mut conn, length).await?;
        Ok(clips)
    }
    async fn order_by_date_later(
        &self,
        reference: &Video<T>,
        length: usize,
    ) -> Result<Vec<Video<T>>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let clips = video_sql_runner::order_by_date_later(&mut conn, reference, length).await?;
        Ok(clips)
    }
    async fn remove(&self, id: VideoId) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        video_sql_runner::remove(&mut conn, id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::video_sql_runner;
    use crate::video_repository_impl::assert_video::{
        videos_assert_eq, videos_assert_eq_with_sort_by_key_and_filter,
    };
    use crate::InfraError;
    use domain::video::{Kirinuki, Original, Video};

    use fake::{Fake, Faker};
    use rand::seq::SliceRandom;
    use rand::Rng;
    use rstest::{fixture, rstest};
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::time::Duration;

    #[fixture]
    fn original_videos() -> Result<Vec<Video<Original>>, InfraError> {
        Ok((0..100)
            .map(|_| Faker.fake::<Video<Original>>())
            .collect::<Vec<_>>())
    }

    #[fixture]
    fn kirinuki_videos() -> Result<Vec<Video<Kirinuki>>, InfraError> {
        Ok((0..100)
            .map(|_| Faker.fake::<Video<Kirinuki>>())
            .collect::<Vec<_>>())
    }

    #[fixture]
    async fn pool() -> Result<PgPool, InfraError> {
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .idle_timeout(Duration::from_secs(1))
            .connect(&database_url)
            .await?;
        Ok(pool)
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        let mut originals_res = video_sql_runner::all::<Original>(&mut transaction).await?;

        videos_assert_eq(&mut originals_res, &mut originals);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_edit_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        // originalsの一部を編集．
        for _ in 0..(originals.len() / 2) {
            let edited_original = originals.choose_mut(&mut rand::thread_rng()).unwrap();
            edited_original.assign(Faker.fake());
            video_sql_runner::edit(&mut transaction, edited_original.clone()).await?;
        }

        let mut originals_res = video_sql_runner::all::<Original>(&mut transaction).await?;
        videos_assert_eq(&mut originals_res, &mut originals);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_increment_like_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        // originalsの一部をincrement_like
        for _ in 0..(originals.len() / 2) {
            let incremented_original = originals.choose_mut(&mut rand::thread_rng()).unwrap();
            incremented_original.increment_like();
            video_sql_runner::increment_like::<Original>(
                &mut transaction,
                incremented_original.id(),
            )
            .await?;
        }

        let mut originals_res = video_sql_runner::all::<Original>(&mut transaction).await?;

        videos_assert_eq(&mut originals_res, &mut originals);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_like_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        let length = originals.len() / 2;

        let mut originals_res = video_sql_runner::order_by_like(&mut transaction, length).await?;

        // originalsをlike(降順)・idの順に並べる．length分フィルタリング．
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.like().cmp(&x.like()),
            Option::<fn(&Video<Original>) -> bool>::None,
            Some(length),
        );

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_like_later_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        let length = originals.len() / 2;

        // referenceとなるvideoを取得
        let reference = {
            let reference_index = rand::thread_rng().gen_range(0..length);
            originals[reference_index].clone()
        };

        let mut originals_res =
            video_sql_runner::order_by_like_later(&mut transaction, &reference, length).await?;

        // originalsをlike(降順)・idの順に並べ，フィルタリングして比較
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.like().cmp(&x.like()),
            Some(|original: &Video<Original>| {
                original.like() < reference.like()
                    || (original.like() == reference.like() && original.id() > reference.id())
            }),
            Some(length),
        );

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_date_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        let length = originals.len() / 2;

        let mut originals_res = video_sql_runner::order_by_date(&mut transaction, length).await?;

        // originalsをdate(降順)・idの順に並べる．length分フィルタリング．
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.date().cmp(&x.date()),
            Option::<fn(&Video<Original>) -> bool>::None,
            Some(length),
        );

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_order_by_date_later_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        let length = originals.len() / 2;

        // referenceとなるvideoを取得
        let reference = {
            let reference_index = rand::thread_rng().gen_range(0..length);
            originals[reference_index].clone()
        };

        let mut originals_res =
            video_sql_runner::order_by_date_later(&mut transaction, &reference, length).await?;

        // originalsをdate(降順)・idの順に並べ，フィルタリング
        videos_assert_eq_with_sort_by_key_and_filter(
            &mut originals_res,
            &mut originals,
            |x, y| y.date().cmp(&x.date()),
            Some(|original: &Video<Original>| {
                original.date() < reference.date()
                    || (original.date() == reference.date() && original.id() > reference.id())
            }),
            Some(length),
        );

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_save_and_remove_and_all(
        original_videos: Result<Vec<Video<Original>>, InfraError>,
        kirinuki_videos: Result<Vec<Video<Kirinuki>>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let originals = original_videos?;
        let kirinukis = kirinuki_videos?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for (original, kirinuki) in originals.iter().cloned().zip(kirinukis.iter().cloned()) {
            video_sql_runner::save(&mut transaction, original).await?;
            video_sql_runner::save(&mut transaction, kirinuki).await?;
        }

        // originalsの一部を削除
        let remove_indices = (0..originals.len()).collect::<Vec<usize>>();
        let remove_indices = remove_indices.into_iter().take(20).collect::<Vec<_>>();

        let removed_originals = originals
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, original)| remove_indices.contains(&i).then_some(original))
            .collect::<Vec<_>>();
        let mut rest_originals = originals
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, original)| (!remove_indices.contains(&i)).then_some(original))
            .collect::<Vec<_>>();

        for original in removed_originals.into_iter() {
            video_sql_runner::remove(&mut transaction, original.id()).await?;
        }

        let mut originals_res = video_sql_runner::all::<Original>(&mut transaction).await?;
        videos_assert_eq(&mut originals_res, &mut rest_originals);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_edit_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        let original = Faker.fake::<Video<Original>>();

        let res = video_sql_runner::edit(&mut transaction, original).await;

        assert!(matches!(res, Err(InfraError::NoRecordError)));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_increment_like_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        let original = Faker.fake::<Video<Original>>();

        let res =
            video_sql_runner::increment_like::<Original>(&mut transaction, original.id()).await;

        assert!(matches!(res, Err(InfraError::NoRecordError)));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_video_remove_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        let original = Faker.fake::<Video<Original>>();

        let res = video_sql_runner::remove(&mut transaction, original.id()).await;

        assert!(matches!(res, Err(InfraError::NoRecordError)));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }
}
