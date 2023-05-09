mod db_video_repository;
mod inmemory_video_repository;
mod mock_video_repository;

pub use db_video_repository::VideoPgDbRepository;
pub use inmemory_video_repository::InMemoryVideoRepository;
pub use mock_video_repository::{MockVideoKirinukiRepository, MockVideoOriginalRepository};

#[cfg(test)]
mod assert_video {
    use domain::video::{Video, VideoType};
    use pretty_assertions::assert_eq;
    use std::cmp::Ordering;

    /// idで昇順にソートして比較
    pub(crate) fn videos_assert_eq<V: VideoType>(
        actual: &mut Vec<Video<V>>,
        expected: &mut Vec<Video<V>>,
    ) {
        actual.sort_by_key(|episode| episode.id());
        expected.sort_by_key(|episode| episode.id());

        assert_eq!(actual, expected);
    }

    /// expectedを`sort_f`を基にソート(同じ場合はidて昇順に)して`filter_f`を基にフィルタリングして比較．
    pub(crate) fn videos_assert_eq_with_sort_by_key_and_filter<V, SF, FF>(
        actual: &mut Vec<Video<V>>,
        expected: &mut Vec<Video<V>>,
        mut sort_f: SF,
        filter_f: Option<FF>,
        n: Option<usize>,
    ) where
        V: VideoType,
        SF: FnMut(&Video<V>, &Video<V>) -> Ordering,
        FF: FnMut(&Video<V>) -> bool,
    {
        // actualはsort_fが同じ場合のみidで昇順にソート
        actual.sort_by(|x, y| {
            if let Ordering::Equal = sort_f(x, y) {
                x.id().cmp(&y.id())
            } else {
                Ordering::Equal
            }
        });

        // expectをsort_f・idで昇順にソート，そしてフィルタリング．
        expected.sort_by(|x, y| sort_f(x, y).then(x.id().cmp(&y.id())));
        if let Some(filter_f) = filter_f {
            expected.retain(filter_f);
        }

        if let Some(n) = n {
            expected.truncate(n);
        }
    }
}
