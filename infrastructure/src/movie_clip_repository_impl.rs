mod db_movie_clip_repository;
mod inmemory_movie_clip_repository;
mod mock_movie_clip_repository;

pub use db_movie_clip_repository::MovieClipPgDBRepository;
pub use inmemory_movie_clip_repository::InMemoryMovieClipRepository;
pub use mock_movie_clip_repository::MockMovieClipRepository;

#[cfg(test)]
mod assert_movie_clip {
    use domain::movie_clip::MovieClip;
    use pretty_assertions::assert_eq;
    use std::cmp::Ordering;

    /// idで昇順にソートして比較
    pub(crate) fn clips_assert_eq(actual: &mut Vec<MovieClip>, expected: &mut Vec<MovieClip>) {
        actual.sort_by_key(|episode| episode.id());
        expected.sort_by_key(|episode| episode.id());

        assert_eq!(actual, expected);
    }

    /// expectedを`sort_f`を基にソート(同じ場合はidて昇順に)して`filter_f`を基にフィルタリングして比較．
    pub(crate) fn clips_assert_eq_with_sort_by_key_and_filter<SF, FF>(
        actual: &mut Vec<MovieClip>,
        expected: &mut Vec<MovieClip>,
        mut sort_f: SF,
        filter_f: Option<FF>,
        n: Option<usize>,
    ) where
        SF: FnMut(&MovieClip, &MovieClip) -> Ordering,
        FF: FnMut(&MovieClip) -> bool,
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
