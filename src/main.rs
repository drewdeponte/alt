extern crate argparse;
extern crate ignore;

use alt::path::scoring::ScoredPath;
use alt::path::utils::cleanse_path;
use alt::{find_alt, find_alt_with_threads};
use argparse::{ArgumentParser, Print, Store, StoreOption, StoreTrue};
use ignore::WalkBuilder;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

pub mod alt;

macro_rules! printerr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

struct Options {
    path: String,
    possible_alternates_path: Option<String>,
    include_hidden: bool,
    truncate: usize,
    filename_weight: f32,
    path_weight: f32,
    use_threads: bool,
}

fn get_possible_files(ignore_hidden: bool) -> Vec<PathBuf> {
    WalkBuilder::new("./")
        .follow_links(true)
        .hidden(ignore_hidden)
        .build()
        .filter_map(|direntry| {
            let entry = direntry.ok()?;
            if entry.file_type()?.is_file() {
                Some(entry.path().to_owned())
            } else {
                None
            }
        })
        .collect()
}

fn parse_args_or_exit() -> Options {
    let mut options = Options {
        path: "".to_string(),
        possible_alternates_path: None,
        include_hidden: false,
        truncate: 0,
        filename_weight: 10.0,
        path_weight: 1.0,
        use_threads: false,
    };

    {
        // block limits of borrows by refer() method calls
        let mut ap = ArgumentParser::new();
        ap.add_option(
            &["-v", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "show version",
        );
        ap.refer(&mut options.truncate).add_option(
            &["-t", "--truncate"],
            Store,
            "truncate results to provided length. 0 = don't truncate, > 0 = truncate",
        );
        ap.refer(&mut options.filename_weight).add_option(
            &["--filename-weight"],
            Store,
            "override the default weight of filenames in the scoring algorithm (default: 10.0)",
        );
        ap.refer(&mut options.path_weight).add_option(
            &["--path-weight"],
            Store,
            "override the default weight of paths in the scoring algorithm (default: 1.0)",
        );
        ap.refer(&mut options.possible_alternates_path).add_option(
            &["-f", "--file"],
            StoreOption,
            "possible alternates file, - for stdin",
        );
        ap.refer(&mut options.include_hidden).add_option(
            &["-a"],
            StoreTrue,
            "include directory entries whose names begin with a dot",
        );
        ap.refer(&mut options.use_threads).add_option(
            &["-j"],
            StoreTrue,
            "Use threads to do similarity scoring in parallel (default: false)",
        );
        ap.refer(&mut options.path)
            .add_argument("PATH", Store, "path to find alternate for")
            .required();
        ap.parse_args_or_exit();
    }

    options
}

fn scored_paths_to_string(scored_paths: &[ScoredPath]) -> String {
    let matches: Vec<String> = scored_paths
        .iter()
        .map(|scored_path| scored_path.path.clone())
        // .map(|(score, path)| format!("{:?} {}", score, path.to_string()))
        .collect();

    if matches.is_empty() {
        String::new()
    } else {
        matches.join("\n")
    }
}

fn main() {
    let options = parse_args_or_exit();

    let cleansed_path = cleanse_path(&options.path);

    let altenate_paths_string = if let Some(unwrapped_file) = options.possible_alternates_path {
        if unwrapped_file == "-" {
            let stdin = std::io::stdin();
            let paths: Vec<String> = stdin.lock().lines().map(|path| path.unwrap()).collect();
            let scored_paths: Vec<ScoredPath> = match options.use_threads {
                true => find_alt_with_threads(
                    &cleansed_path,
                    paths,
                    options.truncate,
                    options.filename_weight,
                    options.path_weight,
                )
                .expect("Failed to find available parallelism"),
                false => find_alt(
                    &cleansed_path,
                    paths,
                    options.truncate,
                    options.filename_weight,
                    options.path_weight,
                ),
            };
            scored_paths_to_string(&scored_paths)
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
            let scored_paths: Vec<ScoredPath> = match options.use_threads {
                true => find_alt_with_threads(
                    &cleansed_path,
                    paths,
                    options.truncate,
                    options.filename_weight,
                    options.path_weight,
                )
                .expect("Failed to find available parallelism"),
                false => find_alt(
                    &cleansed_path,
                    paths,
                    options.truncate,
                    options.filename_weight,
                    options.path_weight,
                ),
            };
            scored_paths_to_string(&scored_paths)
        }
    } else {
        let unwrapped_paths: Vec<String> = get_possible_files(!options.include_hidden)
            .iter()
            .map(|path| path.to_str().unwrap().to_string())
            .collect();
        let scored_paths: Vec<ScoredPath> = match options.use_threads {
            true => find_alt_with_threads(
                &cleansed_path,
                unwrapped_paths,
                options.truncate,
                options.filename_weight,
                options.path_weight,
            )
            .expect("Failed to find available parallelism"),
            false => find_alt(
                &cleansed_path,
                unwrapped_paths,
                options.truncate,
                options.filename_weight,
                options.path_weight,
            ),
        };
        scored_paths_to_string(&scored_paths)
    };
    print!("{}", altenate_paths_string);
}

#[cfg(test)]
mod tests {
    use super::{scored_paths_to_string, ScoredPath};

    #[test]
    fn scored_paths_to_string_with_no_scored_paths() {
        let scored_paths: Vec<ScoredPath> = Vec::new();
        let val = scored_paths_to_string(&scored_paths);

        assert_eq!(val, "");
    }

    #[test]
    fn scored_paths_to_string_with_some_scored_paths() {
        let scored_paths: Vec<ScoredPath> = vec![
            ScoredPath::new(0.3, "some/path/to/a/file.ts".to_owned()),
            ScoredPath::new(0.2, "another/path/to/a/foo.ts".to_owned()),
            ScoredPath::new(0.1, "foo/bar/car/zar.ts".to_owned()),
        ];
        let val = scored_paths_to_string(&scored_paths);

        assert_eq!(
            val,
            "some/path/to/a/file.ts\nanother/path/to/a/foo.ts\nfoo/bar/car/zar.ts"
        );
    }
}
