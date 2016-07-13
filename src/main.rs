extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store, StoreOption, Print};

fn main() {
    let mut debug = false;
    let mut path = "".to_string();
    let mut file: Option<String> = None;
    { // block limits of borrows by refer() method calls
        let mut ap = ArgumentParser::new();
        ap.add_option(&["-v", "--version"], Print(env!("CARGO_PKG_VERSION").to_string()), "show version");
        ap.refer(&mut debug).add_option(&["-d", "--debug"], StoreTrue, "enable debug output");
        ap.refer(&mut file).add_option(&["-f", "--file"], StoreOption, "possible alternates file, - for stdin");
        ap.refer(&mut path).add_argument("PATH", Store, "path to find alternate for").required();
        ap.parse_args_or_exit();
    }

    println!("required path: {}", path);

    if let Some(unwrapped_file) = file {
        println!("file: {}", unwrapped_file);
    }

    println!("debug: {}", debug);
}
