#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub mod video_usecases {
    use crate::commands::video_commands;
    use common::AppCommonError;
    use domain::{video::VideoId, VideoRepository};
    use infrastructure::InfraError;
    use std::sync::Arc;

    pub(crate) fn save_video<T, V>(
        repo: Arc<T>,
        cmd: video_commands::SaveVideoCommand<V>,
    ) -> Result<(), AppCommonError>
    where
        T: VideoRepository<V, Error = InfraError> + 'static,
        V: Default
            + ToString
            + TryFrom<String, Error = domain::DomainError>
            + Send
            + Sync
            + Unpin
            + 'static,
    {
        todo!()
    }
}
