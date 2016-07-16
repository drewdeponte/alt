extern crate argparse;
extern crate glob;

use argparse::{ArgumentParser, StoreTrue, Store, StoreOption, Print};
use std::io::BufRead;
use glob::glob;

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
                for path in paths.iter() {
                    println!("{:?}", path.display());
                }
            },
            Err(e) => println!("{:?}", e)
        }
    }

    println!("debug: {}", options.debug);
}
