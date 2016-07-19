extern crate argparse;
extern crate glob;
extern crate regex;

use argparse::{ArgumentParser, StoreTrue, Store, StoreOption, Print};
use std::io::BufRead;
use glob::glob;
use regex::Regex;

fn identity<T>(x: T) -> T {x}

struct Options {
    debug: bool,
    path: String,
    file: Option<String>
}

fn get_possible_files_from_glob() -> Result<Vec<std::path::PathBuf>, glob::PatternError> {
    // Dir.glob("**/*").reject { |p| File.directory?(p) }.map { |p| Path.new(p) }
    glob("**/*").map(|paths| paths.flat_map(identity).filter(|path| path.is_file()).collect())
}

fn get_filename_minus_extension(path_str: &String) -> String {
    std::path::Path::new(path_str).file_stem().unwrap().to_str().unwrap().to_string()
}

fn is_test_file(path: &String) -> bool {
    let re = Regex::new(r"^(features/|test/|spec/|tests/)").unwrap();
    re.is_match(path.as_str())
}

fn strip_test_words(filename: &String) -> String {
    let re = Regex::new(r"(test_)?(?P<p>\w+?)(_rake_spec|_spec|_test|_steps)?(\.rb|\.exs|\.ex|\.js|\.py)?$").unwrap();
    re.replace_all(filename.as_str(), "$p")
}

fn cleanse_path(path: &String) -> String {
    let re = Regex::new(r"^(\./|\r|\n| )").unwrap();
    re.replace_all(path.as_str(), "")
}

fn find_longest_common_substring(s1: &String, s2: &String) -> String {
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
        return "".to_string()
    }

    let mut m: Vec<Vec<i32>> = Vec::new();
    for x in 0..s1.len() {
        let mut v: Vec<i32> = Vec::new();
        for y in 0..s2.len() {
            v.push(0);
        }
        m.push(v);
    }

    let mut longest_length = 0;
    let mut longest_end_pos = 0;

    for i in 0..s1.len() {
        for j in 0..s2.len() {
            if s1.as_bytes()[i] == s2.as_bytes()[j] {
                m[i][j] = 1;
                if i > 0 && j > 0 {
                    m[i][j] += m[i-1][j-1];
                }
                if m[i][j] > longest_length {
                    longest_length = m[i][j];
                    longest_end_pos = i;
                }
            }
        }
    }

    let start: usize = longest_end_pos as usize - longest_length as usize + 1;
    let end: usize = longest_end_pos as usize;
    s1[start..end].to_string()
}

fn score(s1: &String, s2: &String) -> f32 {
    // TODO: Improve performance. At the moment the find_longest_common_substring
    // method is the most costly operation in this app. This leaves us with a few
    // levers to move in terms of improving performance. Specifically, we could
    // find a more performant algorithm, or find a way to reduce the number of
    // times it has to be called, or use threading to do a scatter and gather
    // approach.
    let longest_match = find_longest_common_substring(s1, s2);
    (longest_match.len() as f32/s2.len() as f32) * (longest_match.len() as f32/s1.len() as f32)
}

fn main() {
    let mut options = Options {
        debug: false,
        path: "".to_string(),
        file: None
    };

    let mut highest_score: f32 = 0.0;
    let mut best_match: String = "".to_string();

    { // block limits of borrows by refer() method calls
        let mut ap = ArgumentParser::new();
        ap.add_option(&["-v", "--version"], Print(env!("CARGO_PKG_VERSION").to_string()), "show version");
        ap.refer(&mut options.debug).add_option(&["-d", "--debug"], StoreTrue, "enable debug output");
        ap.refer(&mut options.file).add_option(&["-f", "--file"], StoreOption, "possible alternates file, - for stdin");
        ap.refer(&mut options.path).add_argument("PATH", Store, "path to find alternate for").required();
        ap.parse_args_or_exit();
    }

    //println!("debug: {}", options.debug);
    //println!("required path: {}", options.path);
    let cleansed_path = cleanse_path(&options.path);

    if let Some(unwrapped_file) = options.file {
        println!("file: {}", unwrapped_file);
        if unwrapped_file == "-" {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
              println!("{}", line.unwrap());
            }
        } else {
            // figure out how to open specified file and read in as content
        }
    } else {
        match get_possible_files_from_glob() {
            Ok(paths) => {
                // - get collection of possible alternate file paths
                // - cleanse possible paths (stripping prefixed ./ and postfixed \r,\n,sp)
                // - if path is test file
                //      - reduce set to paths that match filename stripped of test words and extensions
                //      - reduce set to non-test files
                //   else
                //      - reduce set to paths that match filename stripped of test words and extensions
                //      - reduce set to test files
                // - score the reduced set of possible paths, tracking the highest score
                // - return the path with the highest score

                let filename = get_filename_minus_extension(&options.path);
                if is_test_file(&cleansed_path) {
                    let filename = strip_test_words(&filename);
                    let reduced_paths = paths.iter()
                        .map(|path| cleanse_path(&path.to_str().unwrap().to_string()))
                        .filter(|path| path.contains(filename.as_str()))  // filter to paths that contain the filename
                        .filter(|path| !is_test_file(&path)); // filter to paths that aren't test files
                    for path in reduced_paths {
                        let s = score(&path, &cleansed_path);
                        if s > highest_score {
                            highest_score = s;
                            best_match = path;
                        }
                    }
                } else {
                    let reduced_paths = paths.iter()
                        .map(|path| cleanse_path(&path.to_str().unwrap().to_string()))
                        .filter(|path| path.contains(filename.as_str())) // filter to paths that contain the filename
                        .filter(|path| is_test_file(&path)); // filter to paths that ARE test files
                    for path in reduced_paths {
                        let s = score(&path, &cleansed_path);
                        if s > highest_score {
                            highest_score = s;
                            best_match = path;
                        }
                    }
                }
            },
            Err(e) => println!("{:?}", e)
        }
        print!("{}", best_match);
    }

}
