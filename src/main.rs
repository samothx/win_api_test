//#[macro_use]
extern crate log;
extern crate clap;
extern crate stderrlog;

use clap::{App, Arg};

#[cfg(not(windows))]
fn main() {
    println!("this program is meant to run on windows OS ");
}

#[cfg(windows)]
fn main() {
    let matches = App::new("win-api-test")
        .version("0.1")
        .author("Thomas Runte <thomasr@balena.io>")
        .about("Test win-api calls in rust")
        .arg(
            Arg::with_name("info")
                .short("i")
                .help("reports system info")
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let log_level = matches.occurrences_of("verbose") as usize;
    
    stderrlog::new()
        .module(module_path!())
        .verbosity(log_level)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();

    test_win_api::enumerate_volumes().unwrap();
}