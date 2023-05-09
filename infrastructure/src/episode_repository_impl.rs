mod db_episode_repository;
mod inmemory_episode_repository;
mod mock_episode_repository;

pub use db_episode_repository::EpisodePgDBRepository;
pub use inmemory_episode_repository::InMemoryEpisodeRepository;
pub use mock_episode_repository::MockEpisodeRepository;

#[cfg(test)]
pub(crate) mod episode_assert {
    use domain::episode::Episode;
    use pretty_assertions::assert_eq;
    use std::cmp::Ordering;

    /// idで昇順にソートして比較
    pub(crate) fn episodes_assert_eq(actual: &mut Vec<Episode>, expected: &mut Vec<Episode>) {
        actual.sort_by_key(|episode| episode.id());
        expected.sort_by_key(|episode| episode.id());

        assert_eq!(actual, expected);
    }

    /// expectedを`sort_f`を基にソート(同じ場合はidて昇順に)して`filter_f`を基にフィルタリングして比較．
    pub(crate) fn episodes_assert_eq_with_sort_by_key_and_filter<SF, FF>(
        actual: &mut Vec<Episode>,
        expected: &mut Vec<Episode>,
        mut sort_f: SF,
        filter_f: FF,
    ) where
        SF: FnMut(&Episode, &Episode) -> Ordering,
        FF: FnMut(&Episode) -> bool,
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
        expected.retain(filter_f);
    }
}
