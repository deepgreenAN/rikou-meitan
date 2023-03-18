#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub mod video_usecases {
    use crate::commands::video_commands;
    use common::AppCommonError;
    use domain::{
        video::{Video, VideoId, VideoType},
        VideoRepository,
    };
    use infrastructure::InfraError;
    use std::sync::Arc;

    pub(crate) async fn save_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::SaveVideoCommand<V>,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.save(cmd.video).await?;
        Ok(())
    }

    pub(crate) async fn edit_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::EditVideoCommand<V>,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.edit(cmd.video).await?;
        Ok(())
    }

    pub(crate) async fn increment_like_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::IncrementLikeVideoCommand,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.increment_like(cmd.id).await?;
        Ok(())
    }

    pub(crate) async fn order_by_like_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByLikeVideoCommand,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_like(cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn order_by_like_later_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByLikeLaterVideoCommand<V>,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_like_later(&cmd.reference, cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn order_by_date_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByDateVideoCommand,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_date(cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn order_by_date_later_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::OrderByDateLaterVideoCommand<V>,
    ) -> Result<Vec<Video<V>>, AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        let videos = repo.order_by_date_later(&cmd.reference, cmd.length).await?;
        Ok(videos)
    }

    pub(crate) async fn remove_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::RemoveVideoCommand,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: VideoType + 'static,
    {
        repo.remove(cmd.id).await?;
        Ok(())
    }
}
