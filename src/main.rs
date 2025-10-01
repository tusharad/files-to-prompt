mod cli;
mod core;

use core::process;
use std::process::exit;

fn main() {
    let app_config = cli::get_config();
    if app_config.paths.is_empty() {
        println!("Please provide file/directory path(s).");
        exit(1);
    }
    let _ = process(app_config);
}
