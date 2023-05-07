use super::utils::cleanse_path;
use std::path::Path;

pub type ScoredPath = (f32, String);

fn find_longest_common_substring_length(s1: &str, s2: &str) -> i32 {
    // Currently this is implemented using a dynamic programming solution similar
    // to http://www.geeksforgeeks.org/longest-common-substring/. This is O(N*M)
    // where N is the length of one string and M is the length of the other
    // string.
    //
    // Another option would of course be to explore using something like a
    // suffix tree to solve this problem, something like, the following.
    // http://www.geeksforgeeks.org/suffix-tree-application-5-longest-common-substring-2/
    // This is O(M+N) to build a Generalized Suffix Tree and O(M+N) to find the
    // the longest common substring via depth first search.
    //
    // Beyond that we would have to explore not caring about longest substring
    // and moving to a similarity ranking algorithm that maybe cares about
    // subsequences rather that substrings, etc.
    if s1.is_empty() || s2.is_empty() {
        return 0;
    }

    let mut m: Vec<Vec<i32>> = Vec::with_capacity(s1.len());
    for _ in 0..s1.len() {
        let v: Vec<i32> = vec![0; s2.len()];
        m.push(v);
    }

    let mut longest_length = 0;

    let s1_bytes = s1.as_bytes();
    let s2_bytes = s2.as_bytes();

    for i in 0..s1.len() {
        for j in 0..s2.len() {
            if s1_bytes[i] == s2_bytes[j] {
                m[i][j] = 1;
                if i > 0 && j > 0 {
                    m[i][j] += m[i - 1][j - 1];
                }
                if m[i][j] > longest_length {
                    longest_length = m[i][j];
                }
            }
        }
    }

    longest_length
}

fn similarity_ratio(s1: &str, s2: &str) -> f32 {
    let longest_common_substring_len = find_longest_common_substring_length(s1, s2) as f32;

    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    (longest_common_substring_len / s1.len() as f32)
        * (longest_common_substring_len / s2.len() as f32)
}

fn score(s1: &str, s2: &str, filename_weight: f32, path_weight: f32) -> f32 {
    let path1 = Path::new(s1);
    let path2 = Path::new(s2);

    match (
        path1.file_stem().and_then(|f| f.to_str()),
        path2.file_stem().and_then(|f| f.to_str()),
    ) {
        (Some(path1_filename), Some(path2_filename)) => {
            let filename_score = similarity_ratio(path1_filename, path2_filename);

            let path_score = match (
                path1.parent().and_then(|f| f.to_str()),
                path2.parent().and_then(|f| f.to_str()),
            ) {
                (Some(path1_parent), Some(path2_parent)) => {
                    similarity_ratio(path1_parent, path2_parent)
                }
                (None, None) => 1.0, // both have no path, thats a perfect match
                _ => 0.0,            // one has path but other doesn't, can't be any similarity
            };

            // filename_score + path_score
            (filename_weight * filename_score) + (path_weight * path_score)
        }
        _ => 0.0,
    }
}

pub fn score_paths(
    paths: Vec<String>,
    cleansed_path: &str,
    filename_weight: f32,
    path_weight: f32,
) -> Vec<ScoredPath> {
    paths
        .iter()
        .map(|path| cleanse_path(path))
        .filter(|path| path != cleansed_path)
        .map(|path| {
            (
                score(cleansed_path, &path, filename_weight, path_weight),
                path,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{score, score_paths, similarity_ratio};

    #[test]
    fn score_paths_with_same_path_it_should_filter_same_path() {
        let paths: Vec<String> = vec![
            "foo/bar/car.ts".to_owned(),
            "hoopty/doopty/foopty.ts".to_owned(),
            "home/away/lets_play.ts".to_owned(),
        ];

        let scored_paths = score_paths(paths, "hoopty/doopty/foopty.ts", 10.0, 1.0);

        assert_eq!(scored_paths.len(), 2);
        assert_eq!(scored_paths[0].1, "foo/bar/car.ts".to_owned());
        assert_eq!(scored_paths[1].1, "home/away/lets_play.ts".to_owned());
        assert!(scored_paths[0].0 > 0.0);
        assert!(scored_paths[1].0 > 0.0);
    }

    #[test]
    fn score_paths_without_same_path_it_should_not_filter_same_path() {
        let paths: Vec<String> = vec![
            "foo/bar/car.ts".to_owned(),
            "hoopty/doopty/foopty.ts".to_owned(),
            "home/away/lets_play.ts".to_owned(),
        ];

        let scored_paths = score_paths(paths, "person/place/thing.ts", 10.0, 1.0);

        assert_eq!(scored_paths.len(), 3);
        assert_eq!(scored_paths[0].1, "foo/bar/car.ts".to_owned());
        assert_eq!(scored_paths[1].1, "hoopty/doopty/foopty.ts".to_owned());
        assert_eq!(scored_paths[2].1, "home/away/lets_play.ts".to_owned());
        assert!(scored_paths[0].0 > 0.0);
        assert!(scored_paths[1].0 > 0.0);
        assert!(scored_paths[2].0 > 0.0);
    }

    #[test]
    fn score_paths_that_have_no_similarity_as_zero() {
        let val = score("abc/d", "xyz/e", 10.0, 1.0);
        assert_eq!(val, 0.0);
    }

    #[test]
    fn score_paths_where_filenames_only_but_no_similarity_as_zero() {
        let val = score("foo", "bar", 10.0, 1.0);
        assert_eq!(val, 0.0);
    }

    #[test]
    fn score_paths_where_doesnot_have_file_name_as_zero() {
        let val = score("/", "foo/bar/zar", 10.0, 1.0);
        assert_eq!(val, 0.0);
        let val = score("/..", "foo/bar/zar", 10.0, 1.0);
        assert_eq!(val, 0.0);
        let val = score("", "foo/bar/zar", 10.0, 1.0);
        assert_eq!(val, 0.0);
        let val = score("..", "foo/bar/zar", 10.0, 1.0);
        assert_eq!(val, 0.0);
        let val = score("foo/bar/zar", "/", 10.0, 1.0);
        assert_eq!(val, 0.0);
        let val = score("foo/bar/zar", "/..", 10.0, 1.0);
        assert_eq!(val, 0.0);
        let val = score("foo/bar/zar", "", 10.0, 1.0);
        assert_eq!(val, 0.0);
        let val = score("foo/bar/zar", "..", 10.0, 1.0);
        assert_eq!(val, 0.0);
    }

    #[test]
    fn score_paths_that_have_similar_files_over_similar_dirs() {
        let val_a = score("foo/bar/car.ts", "aaa/ddd/car.ts", 10.0, 1.0);
        let val_b = score("aaa/ddd/hoopty.ts", "aaa/ddd/car.ts", 10.0, 1.0);
        assert!(val_a > val_b);
    }

    #[test]
    fn score_paths_that_have_similar_dirs_over_ones_that_dont_when_files_match() {
        let val_a = score("foo/bar/car.ts", "aaa/ddd/car.ts", 10.0, 1.0);
        let val_b = score("ppp/ddd/car.ts", "aaa/ddd/car.ts", 10.0, 1.0);
        assert!(val_b > val_a);
    }

    #[test]
    fn score_paths_based_on_similarity_with_filename_having_presedence() {
        let val_a = score("foo/bar/car.ts", "aaa/ddd/car.ts", 10.0, 1.0);
        let val_b = score("ppp/ddd/car.ts", "aaa/ddd/car.ts", 10.0, 1.0);
        let val_c = score("aaa/ddd/car.ts", "aaa/ddd/car.ts", 10.0, 1.0);
        assert!(val_c > val_b);
        assert!(val_b > val_a);
    }

    #[test]
    fn similarity_ratio_is_zero_when_one_or_more_strings_is_empty() {
        assert_eq!(similarity_ratio("", "foo/bar/car.ts"), 0.0);
        assert_eq!(similarity_ratio("foo/bar/car.ts", ""), 0.0);
        assert_eq!(similarity_ratio("", ""), 0.0);
    }

    #[test]
    fn similarity_ratio_is_not_impacted_by_size_of_match_alone() {
        let val_a = similarity_ratio("foobarcar", "bar");
        let val_b = similarity_ratio("abc", "b");
        assert_eq!(val_a, val_b);
    }
}
