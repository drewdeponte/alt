extern crate argparse;
extern crate glob;
extern crate regex;

use argparse::{ArgumentParser, Store, StoreOption, Print};
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;
use glob::glob;
use regex::Regex;

fn identity<T>(x: T) -> T {x}

macro_rules! printerr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

struct Options {
    path: String,
    file: Option<String>
}

fn get_possible_files_from_glob() -> Result<Vec<std::path::PathBuf>, glob::PatternError> {
    glob("**/*").map(|paths| paths.flat_map(identity).filter(|path| path.is_file()).collect())
}

fn get_filename_minus_extension(path_str: &String) -> String {
    std::path::Path::new(path_str).file_stem().unwrap().to_str().unwrap().to_string()
}

fn is_test_file(path: &String) -> bool {
    let re = Regex::new(r"^(features/|test/|spec/|tests/|\w*Tests/)").unwrap();
    re.is_match(path.as_str())
}

fn strip_test_words(filename: &String) -> String {
    let re = Regex::new(r"(test_)?(?P<p>\w+?)(_rake_spec|_spec|_test|_steps|Tests|UITests|Specs|UISpecs)?(\.rb|\.exs|\.ex|\.js|\.py|\.swift)?$").unwrap();
    re.replace_all(filename.as_str(), "$p")
}

fn cleanse_path(path: &str) -> String {
    let s = path.to_string();
    if s.len() > 1 && s[0..2].to_string() == "./" {
        s[2..].to_string()
    } else {
        s
    }
}

fn find_longest_common_substring_length(s1: &String, s2: &String) -> i32 {
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
        return 0
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
                    m[i][j] += m[i-1][j-1];
                }
                if m[i][j] > longest_length {
                    longest_length = m[i][j];
                }
            }
        }
    }

    longest_length
}

fn score(s1: &String, s2: &String) -> f32 {
    let longest_match_length: f32 = find_longest_common_substring_length(s1, s2) as f32;
    (longest_match_length/s2.len() as f32) * (longest_match_length/s1.len() as f32)
}

fn find_alt(filename: &String, cleansed_path: &String, paths: Vec<String>, test_file: bool) -> String {
    let (_, alternate) = paths.iter()
        .map(|path| cleanse_path(&path))
        .filter(|path| path.contains(filename.as_str()))  // filter to paths that contain the filename
        .filter(|path| is_test_file(&path) != test_file)
        .fold((0 as f32, "".to_string()), |result, path| {
            let (highest_score, best_match) = result;
            let s = score(&path, &cleansed_path);
            if s > highest_score {
                (s, path)
            } else {
                (highest_score, best_match)
            }
        });
    alternate
}

fn main() {
    let mut options = Options {
        path: "".to_string(),
        file: None
    };

    { // block limits of borrows by refer() method calls
        let mut ap = ArgumentParser::new();
        ap.add_option(&["-v", "--version"], Print(env!("CARGO_PKG_VERSION").to_string()), "show version");
        ap.refer(&mut options.file).add_option(&["-f", "--file"], StoreOption, "possible alternates file, - for stdin");
        ap.refer(&mut options.path).add_argument("PATH", Store, "path to find alternate for").required();
        ap.parse_args_or_exit();
    }

    let cleansed_path = cleanse_path(&options.path);
    let mut filename = get_filename_minus_extension(&options.path);
    if is_test_file(&cleansed_path) {
        filename = strip_test_words(&filename);
    }

    let best_match = if let Some(unwrapped_file) = options.file {
        if unwrapped_file == "-" {
            let stdin = std::io::stdin();
            let paths: Vec<String> = stdin.lock().lines().map(|path| path.unwrap()).collect();
            find_alt(&filename, &cleansed_path, paths, is_test_file(&cleansed_path))
        } else {
            let f = match File::open(&unwrapped_file) {
                Ok(file) => file,
                Err(e) => {
                    printerr!("Failure occurred opening file {}, {}", &unwrapped_file, e);
                    std::process::exit(1)
                }
            };
            let file = BufReader::new(&f);
            let paths: Vec<String> = file.lines().map(|path| path.unwrap()).collect();
            find_alt(&filename, &cleansed_path, paths, is_test_file(&cleansed_path))
        }
    } else {
        match get_possible_files_from_glob() {
            Ok(paths) => {
                let unwrapped_paths:Vec<String> = paths.iter().map(|path| { path.to_str().unwrap().to_string() }).collect();
                find_alt(&filename, &cleansed_path, unwrapped_paths, is_test_file(&cleansed_path))
            },
            Err(e) => {
                printerr!("Error reading paths {}", e);
                std::process::exit(1)
            }
        }
    };
    print!("{}", best_match);
}
