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
        .await?;

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
SELECT * FROM movie_clips ORDER BY like DESC LIMIT $1
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
DELETE FROM movie_clips WHERE id == $1
            "#,
        )
        .bind(id.to_uuid())
        .execute(conn)
        .await?;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// MovieClipPgDBRepository

/// MovieClipに関するPostgresqlのリポジトリ
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
    async fn save(
        &mut self,
        movie_clip: MovieClip,
    ) -> Result<(), <Self as MovieClipRepository>::Error> {
        let mut conn = self.pool.acquire().await?;
        movie_clip_sql_runner::save(&mut conn, movie_clip).await?;
        Ok(())
    }

    async fn all(&mut self) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips = movie_clip_sql_runner::all(&mut conn).await?;
        Ok(movie_clips)
    }

    async fn order_by_like_limit(
        &mut self,
        length: usize,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips = movie_clip_sql_runner::order_by_like_limit(&mut conn, length).await?;
        Ok(movie_clips)
    }

    async fn order_by_create_date_range(
        &mut self,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, <Self as MovieClipRepository>::Error> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips =
            movie_clip_sql_runner::order_by_create_date_range(&mut conn, start, end).await?;
        Ok(movie_clips)
    }

    async fn remove_by_id(
        &mut self,
        id: MovieClipId,
    ) -> Result<(), <Self as MovieClipRepository>::Error> {
        let mut conn = self.pool.acquire().await?;
        movie_clip_sql_runner::remove_by_id(&mut conn, id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::movie_clip_sql_runner;
    use crate::InfraError;
    use domain::movie_clip::MovieClip;
    use sqlx::postgres::PgPoolOptions;

    #[ignore]
    #[tokio::test]
    async fn test_movie_clip_save_and_all() -> Result<(), InfraError> {
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new().connect(&database_url).await?;

        let mut transaction = pool.begin().await?;

        let movie_clip_1 = MovieClip::new(
            "MovieClip 1".to_string(),
            "https://www.youtube.com/watch?v=B7OPlsdBuVc".to_string(),
            100,
            200,
            (2022, 11, 21),
        )?;

        movie_clip_sql_runner::save(&mut transaction, movie_clip_1.clone()).await?;

        let movie_clip_2 = MovieClip::new(
            "MovieClip 2".to_string(),
            "https://www.youtube.com/watch?v=NHpILI4NpCI".to_string(),
            200,
            300,
            (2022, 11, 22),
        )?;

        movie_clip_sql_runner::save(&mut transaction, movie_clip_2.clone()).await?;

        let mut movie_clips_res = movie_clip_sql_runner::all(&mut transaction).await?;
        movie_clips_res.sort_by_key(|movie_clip| movie_clip.id().to_uuid());

        let mut movie_clips = vec![movie_clip_1, movie_clip_2];
        movie_clips.sort_by_key(|movie_clip| movie_clip.id().to_uuid());

        assert_eq!(movie_clips_res, movie_clips);

        transaction.rollback().await?;

        Ok(())
    }
}
