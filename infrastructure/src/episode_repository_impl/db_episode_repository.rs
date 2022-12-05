use crate::InfraError;
use async_trait::async_trait;
use domain::episode::{Episode, EpisodeId};
use domain::{Date, EpisodeRepository};
use sqlx::PgPool;

// -------------------------------------------------------------------------------------------------
// # episode_sql_runner

mod episode_sql_runner {
    use crate::InfraError;
    use domain::episode::{Episode, EpisodeId};
    use domain::Date;
    use sqlx::postgres::{PgConnection, Postgres};

    pub async fn save(conn: &mut PgConnection, episode: Episode) -> Result<(), InfraError> {
        sqlx::query(
            r#"
INSERT INTO episodes ("date", content, id) VALUES ($1, $2, $3)
        "#,
        )
        .bind(episode.date().to_chrono()?)
        .bind(episode.content())
        .bind(episode.id().to_uuid())
        .execute(conn)
        .await
        .map_err(|_| InfraError::ConflictError)?;
        Ok(())
    }
    pub async fn edit(conn: &mut PgConnection, episode: Episode) -> Result<(), InfraError> {
        sqlx::query(
            r#"
UPDATE episodes SET "date" = $1, content = $2 WHERE id = $3 RETURNING *
        "#,
        )
        .bind(episode.date().to_chrono()?)
        .bind(episode.content())
        .bind(episode.id().to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::RemovedRecordError)?;
        Ok(())
    }
    pub async fn all(conn: &mut PgConnection) -> Result<Vec<Episode>, InfraError> {
        let episodes = sqlx::query_as::<Postgres, Episode>(r#"SELECT * FROM episodes"#)
            .fetch_all(conn)
            .await?;
        Ok(episodes)
    }
    pub async fn order_by_date_range(
        conn: &mut PgConnection,
        start: Date,
        end: Date,
    ) -> Result<Vec<Episode>, InfraError> {
        let ordered_by_date_range = sqlx::query_as::<Postgres, Episode>(
            r#"SELECT * FROM episodes WHERE $1 <= "date" AND "date" < $2 ORDER BY "date""#,
        )
        .bind(start.to_chrono()?)
        .bind(end.to_chrono()?)
        .fetch_all(conn)
        .await?;

        Ok(ordered_by_date_range)
    }
    pub async fn remove_by_id(conn: &mut PgConnection, id: EpisodeId) -> Result<(), InfraError> {
        sqlx::query(r#"DELETE FROM episodes WHERE id = $1 RETURNING *"#)
            .bind(id.to_uuid())
            .fetch_one(conn)
            .await
            .map_err(|_| InfraError::RemovedRecordError)?;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// #EpisodePgDBRepository

/// EpisodeのPostgresqlリポジトリ
pub struct EpisodePgDBRepository {
    pool: PgPool,
}

impl EpisodePgDBRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EpisodeRepository for EpisodePgDBRepository {
    type Error = InfraError;
    async fn save(&self, episode: Episode) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        episode_sql_runner::save(&mut conn, episode).await?;
        Ok(())
    }
    async fn edit(&self, episode: Episode) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        episode_sql_runner::edit(&mut conn, episode).await?;
        Ok(())
    }
    async fn all(&self) -> Result<Vec<Episode>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let episodes = episode_sql_runner::all(&mut conn).await?;
        Ok(episodes)
    }
    async fn order_by_date_range(
        &self,
        start: Date,
        end: Date,
    ) -> Result<Vec<Episode>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let ordered_by_date_range =
            episode_sql_runner::order_by_date_range(&mut conn, start, end).await?;
        Ok(ordered_by_date_range)
    }
    async fn remove_by_id(&self, id: EpisodeId) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        episode_sql_runner::remove_by_id(&mut conn, id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::episode_sql_runner;
    use crate::InfraError;
    use assert_matches::assert_matches;
    use domain::episode::Episode;
    use domain::Date;
    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::time::Duration;

    #[fixture]
    fn episodes() -> Result<Vec<Episode>, InfraError> {
        Ok(vec![
            Episode::new((2022, 11, 21), "Some Episode Content1".to_string())?,
            Episode::new((2022, 11, 19), "Some Episode Content2".to_string())?,
            Episode::new((2022, 11, 22), "Some Episode Content3".to_string())?,
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
    async fn test_episode_save_and_all(
        episodes: Result<Vec<Episode>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for episode in episodes.iter().cloned() {
            episode_sql_runner::save(&mut transaction, episode).await?;
        }

        let mut episodes_res = episode_sql_runner::all(&mut transaction).await?;
        episodes_res.sort_by_key(|episode| episode.id());

        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, episodes_res);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_edit_and_all(
        episodes: Result<Vec<Episode>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for episode in episodes.iter().cloned() {
            episode_sql_runner::save(&mut transaction, episode).await?;
        }

        let mut edited_episode = episodes[1].clone();
        edited_episode.edit_date(Date::from_ymd(2022, 11, 23)?)?;
        edited_episode.edit_content("Another Episode Content".to_string())?;
        episodes[1] = edited_episode.clone();

        episode_sql_runner::edit(&mut transaction, edited_episode).await?;

        let mut episodes_res = episode_sql_runner::all(&mut transaction).await?;
        episodes_res.sort_by_key(|episode| episode.id());

        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, episodes_res);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_order_by_date_range(
        episodes: Result<Vec<Episode>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for episode in episodes.iter().cloned() {
            episode_sql_runner::save(&mut transaction, episode).await?;
        }

        let start = Date::from_ymd(2022, 11, 19)?;
        let end = Date::from_ymd(2022, 11, 22)?;

        let ordered_by_date_range =
            episode_sql_runner::order_by_date_range(&mut transaction, start, end).await?;

        episodes.sort_by_key(|episode| episode.date());
        let episodes = episodes
            .into_iter()
            .filter(|episode| start <= episode.date() && episode.date() < end)
            .collect::<Vec<_>>();

        assert_eq!(episodes, ordered_by_date_range);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_remove_by_id(
        episodes: Result<Vec<Episode>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for episode in episodes.iter().cloned() {
            episode_sql_runner::save(&mut transaction, episode).await?;
        }

        let removed_episode = episodes.remove(1); // 二番目の要素を削除

        episode_sql_runner::remove_by_id(&mut transaction, removed_episode.id()).await?;

        let mut rest_episodes = episode_sql_runner::all(&mut transaction).await?;
        rest_episodes.sort_by_key(|episode| episode.id());

        episodes.sort_by_key(|episode| episode.id());

        assert_eq!(episodes, rest_episodes);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_episode_edit_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        let episode = Episode::new((2022, 11, 23), "Another Contents".to_string())?;
        let res = episode_sql_runner::edit(&mut transaction, episode).await;
        assert_matches!(res, Err(InfraError::RemovedRecordError));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_episode_remove_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        let episode = Episode::new((2022, 11, 23), "Another Contents".to_string())?;
        let res = episode_sql_runner::remove_by_id(&mut transaction, episode.id()).await;
        assert_matches!(res, Err(InfraError::RemovedRecordError));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }
}
