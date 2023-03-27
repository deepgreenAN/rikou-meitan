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

    /// MovieClipを一つ保存
    pub async fn save(conn: &mut PgConnection, movie_clip: MovieClip) -> Result<(), InfraError> {
        sqlx::query(
            r#"
INSERT INTO movie_clips (title, "url", "start", "end", id, "like", create_date)
VALUES ($1, $2, $3, $4,  $5, $6, $7)
            "#,
        )
        .bind(movie_clip.title().to_string())
        .bind(movie_clip.url().to_string())
        .bind(movie_clip.range().start().to_u32() as i32)
        .bind(movie_clip.range().end().to_u32() as i32)
        .bind(movie_clip.id().to_uuid())
        .bind(movie_clip.like() as i32)
        .bind(movie_clip.create_date().to_chrono()?)
        .execute(conn)
        .await
        .map_err(|_| InfraError::ConflictError)?;

        Ok(())
    }

    /// MovieClipを一つ編集
    pub async fn edit(conn: &mut PgConnection, movie_clip: MovieClip) -> Result<(), InfraError> {
        sqlx::query(
            r#"
UPDATE movie_clips SET title = $1, "url" = $2, "start" = $3, "end" = $4
WHERE id = $5 RETURNING *
            "#,
        )
        .bind(movie_clip.title().to_string())
        .bind(movie_clip.url().to_string())
        .bind(movie_clip.range().start().to_u32() as i32)
        .bind(movie_clip.range().end().to_u32() as i32)
        .bind(movie_clip.id().to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::NoRecordError)?;

        Ok(())
    }
    /// `id`を持つMovieClipのLikeを一つ増やす
    pub async fn increment_like(
        conn: &mut PgConnection,
        id: MovieClipId,
    ) -> Result<(), InfraError> {
        sqlx::query(
            r#"
UPDATE movie_clips SET "like" = "like" + 1 WHERE id = $1 RETURNING *
        "#,
        )
        .bind(id.to_uuid())
        .fetch_one(conn)
        .await
        .map_err(|_| InfraError::NoRecordError)?;

        Ok(())
    }

    /// 全てのMovieClipを取得．順番は保証されない．
    pub async fn all(conn: &mut PgConnection) -> Result<Vec<MovieClip>, InfraError> {
        let all_clips = sqlx::query_as::<Postgres, MovieClip>(r#"SELECT * FROM movie_clips"#)
            .fetch_all(conn)
            .await?;

        Ok(all_clips)
    }

    /// Likeを降順に`length`分のMovieClipを取得．Likeが同じ場合はidで昇順で並べる
    pub async fn order_by_like(
        conn: &mut PgConnection,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let ordered_clips = sqlx::query_as::<Postgres, MovieClip>(
            r#"
SELECT * FROM movie_clips ORDER BY "like" DESC, id ASC LIMIT $1
                "#,
        )
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_clips)
    }

    /// Likeを降順・さらにidを昇順として`reference`以降のMovieClipを`length`分取得．
    pub async fn order_by_like_later(
        conn: &mut PgConnection,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let ordered_clips = sqlx::query_as::<Postgres, MovieClip>(
            r#"
SELECT * FROM movie_clips WHERE $1 > "like" OR ($1 = "like" AND $2 < id) ORDER BY "like" DESC, id ASC LIMIT $3         
            "#,
        )
        .bind(reference.like() as i32)
        .bind(reference.id().to_uuid())
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_clips)
    }

    /// create_dateを降順として指定した範囲分のMovieClipを`length`分取得．create_dateが同じ場合の順番は保証されない．
    pub async fn order_by_create_date_range(
        conn: &mut PgConnection,
        start: Date,
        end: Date,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let ordered_clips = sqlx::query_as::<Postgres, MovieClip>(
            r#"
SELECT * FROM movie_clips WHERE $1 <= create_date AND create_date < $2 ORDER BY create_date ASC
            "#,
        )
        .bind(start.to_chrono()?)
        .bind(end.to_chrono()?)
        .fetch_all(conn)
        .await?;

        Ok(ordered_clips)
    }

    /// create_dateを降順・さらにidを昇順として`length`分のMovieClipを取得．
    pub async fn order_by_create_date(
        conn: &mut PgConnection,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let ordered_clips = sqlx::query_as::<Postgres, MovieClip>(
            r#"
SELECT * FROM movie_clips ORDER BY create_date DESC, id ASC LIMIT $1
            "#,
        )
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_clips)
    }

    /// create_dateを降順・さらにidを昇順として`reference`以降のMovieClipを`length`分取得．
    pub async fn order_by_create_date_later(
        conn: &mut PgConnection,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let ordered_clips = sqlx::query_as::<Postgres, MovieClip>(
            r#"
SELECT * FROM movie_clips WHERE $1 > create_date OR ($1 = create_date AND $2 < id) ORDER BY create_date DESC, id ASC LIMIT $3
            "#,
        )
        .bind(reference.create_date().to_chrono()?)
        .bind(reference.id().to_uuid())
        .bind(length as i32)
        .fetch_all(conn)
        .await?;

        Ok(ordered_clips)
    }

    /// `id`を持つMovieClipを削除．
    pub async fn remove(conn: &mut PgConnection, id: MovieClipId) -> Result<(), InfraError> {
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
    async fn increment_like(&self, id: MovieClipId) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        movie_clip_sql_runner::increment_like(&mut conn, id).await?;
        Ok(())
    }
    async fn all(&self) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips = movie_clip_sql_runner::all(&mut conn).await?;
        Ok(movie_clips)
    }
    async fn order_by_like(&self, length: usize) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips = movie_clip_sql_runner::order_by_like(&mut conn, length).await?;
        Ok(movie_clips)
    }
    async fn order_by_like_later(
        &self,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips =
            movie_clip_sql_runner::order_by_like_later(&mut conn, reference, length).await?;
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
    async fn order_by_create_date(&self, length: usize) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips = movie_clip_sql_runner::order_by_create_date(&mut conn, length).await?;
        Ok(movie_clips)
    }
    async fn order_by_create_date_later(
        &self,
        reference: &MovieClip,
        length: usize,
    ) -> Result<Vec<MovieClip>, InfraError> {
        let mut conn = self.pool.acquire().await?;
        let movie_clips =
            movie_clip_sql_runner::order_by_create_date_later(&mut conn, reference, length).await?;
        Ok(movie_clips)
    }
    async fn remove(&self, id: MovieClipId) -> Result<(), InfraError> {
        let mut conn = self.pool.acquire().await?;
        movie_clip_sql_runner::remove(&mut conn, id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::movie_clip_sql_runner;
    use crate::InfraError;
    use domain::movie_clip::{MovieClip, MovieClipId};
    use domain::Date;

    use fake::{Fake, Faker};
    use pretty_assertions::assert_eq;
    use rand::{distributions::Distribution, seq::SliceRandom};
    use rstest::{fixture, rstest};
    use sqlx::postgres::{PgPool, PgPoolOptions};
    use std::cmp::Ordering;
    use std::time::Duration;

    #[fixture]
    fn movie_clips() -> Result<Vec<MovieClip>, InfraError> {
        Ok((0..100)
            .map(|_| Faker.fake::<MovieClip>())
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
    async fn test_movie_clip_save_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        let mut clips_res = movie_clip_sql_runner::all(&mut transaction).await?;
        clips_res.sort_by_key(|movie_clip| movie_clip.id());

        clips.sort_by_key(|movie_clip| movie_clip.id());

        assert_eq!(clips_res, clips);

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
        let mut clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        // clipsの一部を編集
        for _ in 0..(clips.len() / 2) {
            let edited_clip = clips.choose_mut(&mut rand::thread_rng()).unwrap();
            edited_clip.assign(Faker.fake());

            movie_clip_sql_runner::edit(&mut transaction, edited_clip.clone()).await?;
        }

        let mut clips_res = movie_clip_sql_runner::all(&mut transaction).await?;
        // データベースの取得結果をidでソート
        clips_res.sort_by_key(|clip| clip.id());
        // 参照元をidでソート
        clips.sort_by_key(|clip| clip.id());

        assert_eq!(clips_res, clips);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_increment_like_and_all(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        // clipsの一部をincrement_like
        for _ in 0..(clips.len() / 2) {
            let incremented_clip = clips.choose_mut(&mut rand::thread_rng()).unwrap();
            incremented_clip.increment_like();
            movie_clip_sql_runner::increment_like(&mut transaction, incremented_clip.id()).await?;
        }

        let mut clips_res = movie_clip_sql_runner::all(&mut transaction).await?;
        clips_res.sort_by_key(|clip| clip.id());

        clips.sort_by_key(|clip| clip.id());

        assert_eq!(clips, clips_res);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_like(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        let length = clips.len() / 2;

        // 参照元をlike(降順), idの順でソート．length分フィルタリング．
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();

        let clips_res = movie_clip_sql_runner::order_by_like(&mut transaction, length).await?;

        assert_eq!(clips, clips_res);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_like_later(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;

        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        let length = clips.len() / 2;

        // referenceとなるclipを取得
        let reference = {
            let reference_index =
                rand::distributions::Uniform::from(0..length).sample(&mut rand::thread_rng());
            clips[reference_index].clone()
        };

        // 参照元をlike(降順), idの順でソート・フィルタリング
        clips.sort_by(|x, y| y.like().cmp(&x.like()).then_with(|| x.id().cmp(&y.id())));
        let clips = clips
            .into_iter()
            .filter(|clip| {
                reference.like() > clip.like()
                    || (reference.like() == clip.like() && reference.id() < clip.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        let clips_res =
            movie_clip_sql_runner::order_by_like_later(&mut transaction, &reference, length)
                .await?;

        assert_eq!(clips, clips_res);

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
        let mut clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        let start = Faker.fake::<Date>();
        let end = Faker.fake::<Date>();

        // 参照元をcreate_date, idでソート・範囲をフィルタリング
        clips.sort_by(|x, y| {
            x.create_date()
                .cmp(&y.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        clips.retain(|clip| start <= clip.create_date() && clip.create_date() < end);

        // 得られた結果をcreate_dataが同じ場合のみidでソート
        let mut clips_res =
            movie_clip_sql_runner::order_by_create_date_range(&mut transaction, start, end).await?;
        clips_res.sort_by(|x, y| {
            if let Ordering::Equal = x.create_date().cmp(&y.create_date()) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        assert_eq!(clips, clips_res);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_date(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        let length = clips.len() / 2;

        // 参照元をcreate_date(降順), idでソート・範囲をフィルタリング
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        let clips = clips.into_iter().take(length).collect::<Vec<_>>();

        let clips_res =
            movie_clip_sql_runner::order_by_create_date(&mut transaction, length).await?;

        assert_eq!(clips, clips_res);

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_save_and_order_by_date_later(
        movie_clips: Result<Vec<MovieClip>, InfraError>,
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let mut clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        let length = clips.len() / 2;

        // referenceとなるclipを取得
        let reference = {
            let reference_index =
                rand::distributions::Uniform::from(0..length).sample(&mut rand::thread_rng());
            clips[reference_index].clone()
        };

        // 参照元をcreate_date(降順), idでソート・範囲をフィルタリング
        clips.sort_by(|x, y| {
            y.create_date()
                .cmp(&x.create_date())
                .then_with(|| x.id().cmp(&y.id()))
        });
        let clips = clips
            .into_iter()
            .filter(|clip| {
                clip.create_date() < reference.create_date()
                    || (clip.create_date() == reference.create_date() && clip.id() > reference.id())
            })
            .take(length)
            .collect::<Vec<_>>();

        let clips_res =
            movie_clip_sql_runner::order_by_create_date_later(&mut transaction, &reference, length)
                .await?;

        assert_eq!(clips, clips_res);

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
        let mut clips = movie_clips?;
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;

        for clip in clips.iter().cloned() {
            movie_clip_sql_runner::save(&mut transaction, clip).await?;
        }

        // clipsの一部を削除
        let mut clips_len = clips.len();
        let remove_number = clips_len / 10;
        for _ in 0..remove_number {
            let remove_index = rand::distributions::Uniform::from(0_usize..clips_len)
                .sample(&mut rand::thread_rng());
            let removed_clip = clips.remove(remove_index);
            movie_clip_sql_runner::remove(&mut transaction, removed_clip.id()).await?;

            // clips_lenを一つ減らす
            clips_len -= 1;
        }

        let mut rest_clips = movie_clip_sql_runner::all(&mut transaction).await?;
        rest_clips.sort_by_key(|movie_clip| movie_clip.id());

        clips.sort_by_key(|clip| clip.id());

        assert_eq!(clips, rest_clips);

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
        let clip = Faker.fake::<MovieClip>();

        let res = movie_clip_sql_runner::edit(&mut transaction, clip).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }

    #[ignore]
    #[rstest]
    #[tokio::test]
    async fn test_movie_clip_increment_like_no_exists(
        #[future] pool: Result<PgPool, InfraError>,
    ) -> Result<(), InfraError> {
        let pool = pool.await?;

        // トランザクションの開始
        let mut transaction = pool.begin().await?;
        let clip = Faker.fake::<MovieClip>();

        let res = movie_clip_sql_runner::increment_like(&mut transaction, clip.id()).await;
        assert!(matches!(res, Err(InfraError::NoRecordError)));

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

        let res = movie_clip_sql_runner::remove(&mut transaction, MovieClipId::generate()).await;

        assert!(matches!(res, Err(InfraError::NoRecordError)));

        // ロールバック
        transaction.rollback().await?;

        Ok(())
    }
}
