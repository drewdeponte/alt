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
    return glob("**/*").map(|paths| paths.flat_map(identity).filter(|path| path.is_file()).collect())
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

fn main() {
    let mut options = Options {
        debug: false,
        path: "".to_string(),
        file: None
    };

    { // block limits of borrows by refer() method calls
        let mut ap = ArgumentParser::new();
        ap.add_option(&["-v", "--version"], Print(env!("CARGO_PKG_VERSION").to_string()), "show version");
        ap.refer(&mut options.debug).add_option(&["-d", "--debug"], StoreTrue, "enable debug output");
        ap.refer(&mut options.file).add_option(&["-f", "--file"], StoreOption, "possible alternates file, - for stdin");
        ap.refer(&mut options.path).add_argument("PATH", Store, "path to find alternate for").required();
        ap.parse_args_or_exit();
    }

    println!("debug: {}", options.debug);
    println!("required path: {}", options.path);

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

                let cleansed_path = cleanse_path(&options.path);
                let filename = get_filename_minus_extension(&options.path);
                if is_test_file(&cleansed_path) {
                    let filename = strip_test_words(&filename);
                    let reduced_paths = paths.iter()
                        .map(|path| cleanse_path(&path.to_str().unwrap().to_string()))
                        .filter(|path| path.contains(filename.as_str()))  // filter to paths that contain the filename
                        .filter(|path| !is_test_file(&path)); // filter to paths that aren't test files
                    for path in reduced_paths {
                        println!("{:?}", path);
                    }
                } else {
                    let reduced_paths = paths.iter()
                        .map(|path| cleanse_path(&path.to_str().unwrap().to_string()))
                        .filter(|path| path.contains(filename.as_str())) // filter to paths that contain the filename
                        .filter(|path| is_test_file(&path)); // filter to paths that ARE test files
                    for path in reduced_paths {
                        println!("{:?}", path);
                    }
                }
            },
            Err(e) => println!("{:?}", e)
        }
    }

}
