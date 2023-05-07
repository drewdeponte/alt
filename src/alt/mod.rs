use alt::path::scoring::{score_paths, ScoredPath};
use std::cmp::Ordering;
use std::thread;

pub mod path;

pub fn find_alt(
    cleansed_path: &str,
    paths: Vec<String>,
    truncate_len: usize,
    filename_weight: f32,
    path_weight: f32,
) -> Vec<ScoredPath> {
    let mut possible_paths_with_scores: Vec<ScoredPath> =
        score_paths(paths, cleansed_path, filename_weight, path_weight);

    possible_paths_with_scores.sort_by(order_scored_paths);

    truncate_scored_paths(&mut possible_paths_with_scores, truncate_len);

    possible_paths_with_scores
}

#[derive(Debug)]
pub enum FindAltWithThreadsError {
    NoAvailableParallelism,
}

pub fn find_alt_with_threads(
    cleansed_path: &str,
    paths: Vec<String>,
    truncate_len: usize,
    filename_weight: f32,
    path_weight: f32,
) -> Result<Vec<ScoredPath>, FindAltWithThreadsError> {
    // get the parallel potential
    let parallel_est = thread::available_parallelism()
        .map_err(|_| FindAltWithThreadsError::NoAvailableParallelism)?;

    // split the paths vec into that many groups
    let chunk_size = (paths.len() / parallel_est) + (paths.len() % parallel_est);
    let mut thread_handles: Vec<std::thread::JoinHandle<Vec<ScoredPath>>> = Vec::new();

    // spin up thread for each group to score the paths
    for chunk in paths.chunks(chunk_size) {
        let threads_paths: Vec<String> = chunk.to_vec();
        let threads_cleansed_path: String = cleansed_path.to_owned();
        let threads_filename_weight: f32 = filename_weight;
        let threads_path_weight: f32 = path_weight;
        let thread_handle = std::thread::spawn(move || {
            score_paths(
                threads_paths,
                &threads_cleansed_path,
                threads_filename_weight,
                threads_path_weight,
            )
        });
        thread_handles.push(thread_handle);
    }

    // join on all the threads and get back the scored paths
    // combine them all back into a single vec
    let collections_of_scored_paths: Vec<Vec<ScoredPath>> = thread_handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    let mut scored_paths = collections_of_scored_paths.concat();

    scored_paths.sort_by(order_scored_paths);

    truncate_scored_paths(&mut scored_paths, truncate_len);

    Ok(scored_paths)
}

fn order_scored_paths(scored_path_a: &ScoredPath, scored_path_b: &ScoredPath) -> Ordering {
    if scored_path_a.0 > scored_path_b.0 {
        Ordering::Less
    } else if scored_path_a.0 < scored_path_b.0 {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn truncate_scored_paths(scored_paths: &mut Vec<ScoredPath>, len: usize) {
    match len {
        0 => (),
        _ => scored_paths.truncate(len),
    }
}

#[cfg(test)]
mod tests {
    use super::{find_alt, order_scored_paths, truncate_scored_paths, ScoredPath};

    #[test]
    fn truncate_scored_paths_with_zero_len() {
        let mut scored_paths: Vec<ScoredPath> = vec![
            (0.8, "some/path/to/a/file.ts".to_owned()),
            (0.4, "some/path/to/another/foo.ts".to_owned()),
            (0.2, "some/other_path/to/a/bar.ts".to_owned()),
            (0.1, "some/short/path/zoo.ts".to_owned()),
            (0.023, "some/blue/fortytwo/sports_ball.ts".to_owned()),
        ];

        truncate_scored_paths(&mut scored_paths, 0);

        assert_eq!(scored_paths.len(), 5);
    }

    #[test]
    fn truncate_scored_paths_with_non_zero_len() {
        let mut scored_paths: Vec<ScoredPath> = vec![
            (0.8, "some/path/to/a/file.ts".to_owned()),
            (0.4, "some/path/to/another/foo.ts".to_owned()),
            (0.2, "some/other_path/to/a/bar.ts".to_owned()),
            (0.1, "some/short/path/zoo.ts".to_owned()),
            (0.023, "some/blue/fortytwo/sports_ball.ts".to_owned()),
        ];

        truncate_scored_paths(&mut scored_paths, 3);

        assert_eq!(scored_paths.len(), 3);
    }

    #[test]
    fn order_scored_paths_with_a_larger() {
        let ordering = order_scored_paths(
            &(0.3, "some/path/to/a/file.ts".to_owned()),
            &(0.2, "some/other/path/bar.ts".to_owned()),
        );

        assert_eq!(ordering, std::cmp::Ordering::Less);
    }

    #[test]
    fn order_scored_paths_with_a_smaller() {
        let ordering = order_scored_paths(
            &(0.2, "some/path/to/a/file.ts".to_owned()),
            &(0.3, "some/other/path/bar.ts".to_owned()),
        );

        assert_eq!(ordering, std::cmp::Ordering::Greater);
    }

    #[test]
    fn order_scored_paths_with_a_and_b_equal() {
        let ordering = order_scored_paths(
            &(0.3, "some/path/to/a/file.ts".to_owned()),
            &(0.3, "some/other/path/bar.ts".to_owned()),
        );

        assert_eq!(ordering, std::cmp::Ordering::Equal);
    }

    #[test]
    fn find_alt_scores_paths_and_sorts_them_by_score() {
        let paths: Vec<String> = vec![
            "src/database/nft-wallet/nft-wallet.repository.spec.ts".to_owned(),
            "src/models/mocks/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/mockes/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/nft-wallet.ts".to_owned(),
            "src/database/nft-wallet/nft-wallet.repository.ts".to_owned(),
        ];
        let scored_paths: Vec<ScoredPath> =
            find_alt("src/models/nft-wallet.ts", paths, 0, 10.0, 1.0);
        assert_eq!(scored_paths.len(), 5);
        assert!(scored_paths[0].0 > scored_paths[1].0);
        assert!(scored_paths[1].0 > scored_paths[2].0);
        assert!(scored_paths[2].0 > scored_paths[3].0);
        assert!(scored_paths[3].0 > scored_paths[4].0);

        let stripped_scored_paths: Vec<String> = scored_paths.into_iter().map(|s| s.1).collect();
        assert_eq!(
            stripped_scored_paths,
            vec![
                "src/concerns/nft/models/nft-wallet.ts",
                "src/models/mocks/nft-wallet.mocks.ts",
                "src/concerns/nft/models/mockes/nft-wallet.mocks.ts",
                "src/database/nft-wallet/nft-wallet.repository.ts",
                "src/database/nft-wallet/nft-wallet.repository.spec.ts"
            ]
        )
    }

    #[test]
    fn find_alt_scores_paths_and_drops_full_matches() {
        let paths: Vec<String> = vec![
            "src/models/nft-wallet.ts".to_owned(), // should be dropped
            "src/database/nft-wallet/nft-wallet.repository.spec.ts".to_owned(),
            "src/models/mocks/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/mockes/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/nft-wallet.ts".to_owned(),
            "src/database/nft-wallet/nft-wallet.repository.ts".to_owned(),
        ];
        let scored_paths: Vec<ScoredPath> =
            find_alt("src/models/nft-wallet.ts", paths, 0, 10.0, 1.0);
        assert_eq!(scored_paths.len(), 5);

        let stripped_scored_paths: Vec<String> = scored_paths.into_iter().map(|s| s.1).collect();
        assert_eq!(
            stripped_scored_paths,
            vec![
                "src/concerns/nft/models/nft-wallet.ts",
                "src/models/mocks/nft-wallet.mocks.ts",
                "src/concerns/nft/models/mockes/nft-wallet.mocks.ts",
                "src/database/nft-wallet/nft-wallet.repository.ts",
                "src/database/nft-wallet/nft-wallet.repository.spec.ts"
            ]
        )
    }

    #[test]
    fn find_alt_truncates_when_truncate_len_is_greater_than_zero() {
        let paths: Vec<String> = vec![
            "src/database/nft-wallet/nft-wallet.repository.spec.ts".to_owned(),
            "src/models/mocks/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/mockes/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/nft-wallet.ts".to_owned(),
            "src/database/nft-wallet/nft-wallet.repository.ts".to_owned(),
        ];
        let scored_paths: Vec<ScoredPath> =
            find_alt("src/models/nft-wallet.ts", paths, 3, 10.0, 1.0);
        assert_eq!(scored_paths.len(), 3);
        assert!(scored_paths[0].0 > scored_paths[1].0);
        assert!(scored_paths[1].0 > scored_paths[2].0);

        let stripped_scored_paths: Vec<String> = scored_paths.into_iter().map(|s| s.1).collect();
        assert_eq!(
            stripped_scored_paths,
            vec![
                "src/concerns/nft/models/nft-wallet.ts",
                "src/models/mocks/nft-wallet.mocks.ts",
                "src/concerns/nft/models/mockes/nft-wallet.mocks.ts",
            ]
        )
    }

    #[test]
    fn find_alt_does_not_truncate_results_when_truncate_len_is_zero() {
        let paths: Vec<String> = vec![
            "src/database/nft-wallet/nft-wallet.repository.spec.ts".to_owned(),
            "src/models/mocks/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/mockes/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/nft-wallet.ts".to_owned(),
            "src/database/nft-wallet/nft-wallet.repository.ts".to_owned(),
        ];
        let scored_paths: Vec<ScoredPath> =
            find_alt("src/models/nft-wallet.ts", paths, 0, 10.0, 1.0);
        assert_eq!(scored_paths.len(), 5);
        assert!(scored_paths[0].0 > scored_paths[1].0);
        assert!(scored_paths[1].0 > scored_paths[2].0);
        assert!(scored_paths[2].0 > scored_paths[3].0);
        assert!(scored_paths[3].0 > scored_paths[4].0);

        let stripped_scored_paths: Vec<String> = scored_paths.into_iter().map(|s| s.1).collect();
        assert_eq!(
            stripped_scored_paths,
            vec![
                "src/concerns/nft/models/nft-wallet.ts",
                "src/models/mocks/nft-wallet.mocks.ts",
                "src/concerns/nft/models/mockes/nft-wallet.mocks.ts",
                "src/database/nft-wallet/nft-wallet.repository.ts",
                "src/database/nft-wallet/nft-wallet.repository.spec.ts"
            ]
        )
    }

    #[test]
    fn find_alt_with_larger_path_weight() {
        let paths: Vec<String> = vec![
            "src/database/nft-wallet/nft-wallet.repository.spec.ts".to_owned(),
            "src/models/mocks/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/mockes/nft-wallet.mocks.ts".to_owned(),
            "src/concerns/nft/models/nft-wallet.ts".to_owned(),
            "src/database/nft-wallet/nft-wallet.repository.ts".to_owned(),
        ];
        let scored_paths: Vec<ScoredPath> =
            find_alt("src/models/nft-wallet.ts", paths, 0, 1.0, 10.0);
        assert_eq!(scored_paths.len(), 5);

        let stripped_scored_paths: Vec<String> = scored_paths.into_iter().map(|s| s.1).collect();
        assert_eq!(
            stripped_scored_paths,
            vec![
                "src/models/mocks/nft-wallet.mocks.ts",
                "src/concerns/nft/models/nft-wallet.ts",
                "src/concerns/nft/models/mockes/nft-wallet.mocks.ts",
                "src/database/nft-wallet/nft-wallet.repository.ts",
                "src/database/nft-wallet/nft-wallet.repository.spec.ts"
            ]
        )
    }
}
