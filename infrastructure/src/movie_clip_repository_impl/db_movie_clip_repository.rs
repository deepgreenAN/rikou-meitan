use crate::InfraError;
use async_trait::async_trait;
use domain::movie_clip::{MovieClip, MovieClipId};
use domain::Date;
use domain::MovieClipRepository;
use sqlx::PgPool;

// -------------------------------------------------------------------------------------------------
// movie_clip_sql_runner

/// MovieClipに関するSQLのランナーモジュール
mod movie_clip_sql_runner {
    use crate::InfraError;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;
    use sqlx::{PgConnection, Postgres};

    pub async fn save(conn: &mut PgConnection, movie_clip: MovieClip) -> Result<(), InfraError> {
        sqlx::query(
            r#"
INSERT INTO movie_clips (title, "url", "start", "end", id, "like", create_date)
VALUES ($1, $2, $3, $4,  $5, $6, $7)
            "#,
        )
        .bind(movie_clip.title().to_string())
        .bind(movie_clip.url().to_string())
        .bind(movie_clip.start().to_u32() as i32)
        .bind(movie_clip.end().to_u32() as i32)
        .bind(movie_clip.id().to_uuid())
        .bind(movie_clip.like() as i32)
        .bind(movie_clip.create_date().to_chrono()?)
        .execute(conn)
        .await
        .map_err(|_| InfraError::ConflictError)?;

        Ok(())
    }

    pub async fn edit(conn: &mut PgConnection, movie_clip: MovieClip) -> Result<(), InfraError> {
        sqlx::query(
            r#"
UPDATE movie_clips SET title = $1, "start" = $2, "end" = $3, "like" = $4
WHERE id = $5 RETURNING *
            "#,
        )
        .bind(movie_clip.title().to_string())
        .bind(movie_clip.start().to_u32() as i32)
        .bind(movie_clip.end().to_u32() as i32)
        .bind(movie_clip.like() as i32)
        .bind(movie_clip.id().to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::NoRecordError)?;

        Ok(())
    }

    pub async fn all(conn: &mut PgConnection) -> Result<Vec<MovieClip>, InfraError> {
        let all_movie_clips = sqlx::query_as::<Postgres, MovieClip>(r#"SELECT * FROM movie_clips"#)
            .fetch_all(conn)
            .await?;

        Ok(all_movie_clips)
    }

    pub async fn order_by_like_limit(
        conn: &mut PgConnection,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let ordered_by_limited_movie_clips = sqlx::query_as::<Postgres, MovieClip>(
            r#"
SELECT * FROM movie_clips ORDER BY "like" DESC LIMIT $1
                "#,
        )
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_by_limited_movie_clips)
    }

    pub async fn order_by_create_date_range(
        conn: &mut PgConnection,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let ordered_by_create_data_and_range = sqlx::query_as::<Postgres, MovieClip>(
            r#"
SELECT * FROM movie_clips WHERE $1 <= create_date AND create_date < $2 ORDER BY create_date ASC
            "#,
        )
        .bind(start.to_chrono()?)
        .bind(end.to_chrono()?)
        .fetch_all(conn)
        .await?;

        Ok(ordered_by_create_data_and_range)
    }

    pub async fn remove_by_id(conn: &mut PgConnection, id: MovieClipId) -> Result<(), InfraError> {
        sqlx::query(
            r#"
DELETE FROM movie_clips WHERE id = $1 RETURNING *
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
// MovieClipPgDBRepository

/// MovieClipのPostgresqlのリポジトリ
pub struct MovieClipPgDBRepository {
    pool: PgPool,
}

impl MovieClipPgDBRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MovieClipRepository for MovieClipPgDBRepository {
    type Error = InfraError;
    async fn save(&self, movie_clip: MovieClip) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        movie_clip_sql_runner::save(&mut conn, movie_clip).await?;
        Ok(())
    }

    async fn edit(&self, movie_clip: MovieClip) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        movie_clip_sql_runner::edit(&mut conn, movie_clip).await?;
        Ok(())
    }

    async fn all(&self) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips = movie_clip_sql_runner::all(&mut conn).await?;
        Ok(movie_clips)
    }

    async fn order_by_like_limit(&self, length: usize) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips = movie_clip_sql_runner::order_by_like_limit(&mut conn, length).await?;
        Ok(movie_clips)
    }

    async fn order_by_create_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips =
            movie_clip_sql_runner::order_by_create_date_range(&mut conn, start, end).await?;
        Ok(movie_clips)
    }

    async fn remove_by_id(&self, id: MovieClipId) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        movie_clip_sql_runner::remove_by_id(&mut conn, id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::movie_clip_sql_runner;
    use crate::InfraError;
    use assert_matches::assert_matches;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::time::Duration;

    #[fixture]
    fn movie_clips() -> Result<Vec<MovieClip>, InfraError> {
        Ok(vec![
            MovieClip::new(
                "MovieClip 1".to_string(),
                "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
                100,
                200,
                (2022, 11, 21),
            )?,
            MovieClip::new(
                "MovieClip 2".to_string(),
                "https://www.youtube.com/watch?v=NHpILI4NpCI".to_string(),
                200,
                300,
                (2022, 11, 22),
            )?,
            MovieClip::new(
                "MovieClip 3".to_string(),
                "https://www.youtube.com/watch?v=6LAn0lbMpZ8".to_string(),
                400,
                500,
                (2022, 11, 19),
            )?,
        ])
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
    async fn test_movie_clip_save_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for movie_clip in movie_clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, movie_clip).await?;
        }

        let mut movie_clips_res = movie_clip_sql_runner::all(&mut transaction).await?;
        movie_clips_res.sort_by_key(|movie_clip| movie_clip.id());

        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips_res, movie_clips);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_edit_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for movie_clip in movie_clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, movie_clip).await?;
        }

        // 編集
        let mut edited_movie_clip = movie_clips[1].clone(); // 二番目を編集
        edited_movie_clip.edit_title("Another Movie Clip".to_string())?;
        edited_movie_clip.edit_start_and_end(1200.into(), 1300.into())?;
        movie_clips[1] = edited_movie_clip.clone();

        movie_clip_sql_runner::edit(&mut transaction, edited_movie_clip).await?;

        let mut movie_clips_res = movie_clip_sql_runner::all(&mut transaction).await?;
        movie_clips_res.sort_by_key(|movie_clip| movie_clip.id());

        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips_res, movie_clips);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_like_limit(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let movie_clips = movie_clips?;
        let movie_clips_length = movie_clips.len();
        let mut movie_clips = movie_clips
            .into_iter()
            .enumerate()
            .map(|(i, mut movie_clip)| {
                for _ in 0..(movie_clips_length - i) {
                    movie_clip.like_increment(); // likeをインクリメント
                }
                movie_clip
            })
            .collect::<Vec<_>>();

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for movie_clip in movie_clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, movie_clip).await?;
        }

        let length = 2_usize;

        let ordered_by_like_movie_clips =
            movie_clip_sql_runner::order_by_like_limit(&mut transaction, length).await?;

        movie_clips.sort_by_key(|movie_clip| u32::MAX - movie_clip.like()); // 降順なため
        let movie_clips = movie_clips.into_iter().take(length).collect::<Vec<_>>();

        assert_eq!(movie_clips, ordered_by_like_movie_clips);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_date_range(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for movie_clip in movie_clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, movie_clip).await?;
        }

        let start = Date::from_ymd(2022, 11, 19)?;
        let end = Date::from_ymd(2022, 11, 22)?;

        let ordered_by_date_range =
            movie_clip_sql_runner::order_by_create_date_range(&mut transaction, start, end).await?;

        movie_clips.sort_by_key(|movie_clip| movie_clip.create_date());
        let movie_clips = movie_clips
            .into_iter()
            .filter(|movie_clip| {
                start <= movie_clip.create_date() && movie_clip.create_date() < end
            })
            .collect::<Vec<_>>();

        assert_eq!(movie_clips, ordered_by_date_range);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_remove_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut movie_clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for movie_clip in movie_clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, movie_clip).await?;
        }

        let removed_movie_clip = movie_clips.remove(1); // 二番目のデータ
        movie_clip_sql_runner::remove_by_id(&mut transaction, removed_movie_clip.id()).await?;

        let mut rest_movie_clips = movie_clip_sql_runner::all(&mut transaction).await?;
        rest_movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        movie_clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(movie_clips, rest_movie_clips);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_edit_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;
        let movie_clip = MovieClip::new(
            "Another Title".to_string(),
            "https://www.youtube.com/watch?v=lwSEI1ATLWQ".to_string(),
            1000,
            1500,
            (2022, 11, 23),
        )?;

        let res = movie_clip_sql_runner::edit(&mut transaction, movie_clip).await;
        assert_matches!(res, Err(InfraError::NoRecordError));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_remove_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;
        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        let res =
            movie_clip_sql_runner::remove_by_id(&mut transaction, MovieClipId::generate()).await;

        assert_matches!(res, Err(InfraError::NoRecordError));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }
}
