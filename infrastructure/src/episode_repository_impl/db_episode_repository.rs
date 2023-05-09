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
        .bind(episode.content().to_string())
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
        .bind(episode.content().to_string())
        .bind(episode.id().to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::NoRecordError)?;
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
    pub async fn remove(conn: &mut PgConnection, id: EpisodeId) -> Result<(), InfraError> {
        sqlx::query(r#"DELETE FROM episodes WHERE id = $1 RETURNING *"#)
            .bind(id.to_uuid())
            .fetch_one(conn)
            .await
            .map_err(|_| InfraError::NoRecordError)?;
        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------
// #EpisodePgDBRepository

/// EpisodeのPostgresqlリポジトリ

#[derive(Debug, Clone)]
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
    async fn remove(&self, id: EpisodeId) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        episode_sql_runner::remove(&mut conn, id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::episode_sql_runner;
    use crate::episode_repository_impl::episode_assert::{
        episodes_assert_eq, episodes_assert_eq_with_sort_by_key_and_filter,
    };
    use crate::InfraError;
    use domain::{episode::Episode, Date};

    use fake::{Fake, Faker};
    use rand::seq::SliceRandom;
    use rstest::{fixture, rstest};
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::time::Duration;

    #[fixture]
    fn episodes() -> Result<Vec<Episode>, InfraError> {
        Ok((0..100)
            .map(|_| Faker.fake::<Episode>())
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
    async fn test_episode_save_and_all(
        episodes: Result<Vec<Episode>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut episodes = episodes?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        // データベースへ保存
        for episode in episodes.iter().cloned() {
            episode_sql_runner::save(&mut transaction, episode).await?;
        }

        let mut episodes_res = episode_sql_runner::all(&mut transaction).await?;
        episodes_assert_eq(&mut episodes_res, &mut episodes);

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

        // データベースへ保存
        for episode in episodes.iter().cloned() {
            episode_sql_runner::save(&mut transaction, episode).await?;
        }

        // episodesの一部を編集
        for _ in 0..(episodes.len() / 2_usize) {
            let edited_episode = episodes.choose_mut(&mut rand::thread_rng()).unwrap();
            let new_episode = Faker.fake::<Episode>();
            edited_episode.assign(new_episode);

            episode_sql_runner::edit(&mut transaction, edited_episode.clone()).await?;
        }

        let mut episodes_res = episode_sql_runner::all(&mut transaction).await?;
        episodes_assert_eq(&mut episodes_res, &mut episodes);

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

        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        // データベースから得られた結果をDateが同じ場合のみidでソート
        let mut episodes_res =
            episode_sql_runner::order_by_date_range(&mut transaction, start, end).await?;

        episodes_assert_eq_with_sort_by_key_and_filter(
            &mut episodes_res,
            &mut episodes,
            |x, y| x.date().cmp(&y.date()),
            |episode| start <= episode.date() && episode.date() < end,
        );

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_episode_save_and_remove(
        episodes: Result<Vec<Episode>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let episodes = episodes?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for episode in episodes.iter().cloned() {
            episode_sql_runner::save(&mut transaction, episode).await?;
        }

        // episodesの一部を削除
        let remove_indices = (0..episodes.len()).collect::<Vec<usize>>();
        let remove_indices = remove_indices.into_iter().take(20).collect::<Vec<_>>();

        let removed_episodes = episodes
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, episode)| remove_indices.contains(&i).then_some(episode))
            .collect::<Vec<_>>();
        let mut rest_episodes = episodes
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, episode)| (!remove_indices.contains(&i)).then_some(episode))
            .collect::<Vec<_>>();

        for episode in removed_episodes.into_iter() {
            episode_sql_runner::remove(&mut transaction, episode.id()).await?;
        }

        let mut episodes_res = episode_sql_runner::all(&mut transaction).await?;
        episodes_assert_eq(&mut episodes_res, &mut rest_episodes);

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

        let episode = Faker.fake::<Episode>();
        let res = episode_sql_runner::edit(&mut transaction, episode).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

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

        let episode = Faker.fake::<Episode>();
        let res = episode_sql_runner::remove(&mut transaction, episode.id()).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }
}
