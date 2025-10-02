use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "Files to prompt")]
#[command(version = "0.1")]
#[command(about = "Combine all files into single prompt", long_about = None)]
pub struct AppConfig {
    pub paths: Vec<PathBuf>,

    #[arg(long = "include-hidden", default_value_t = false)]
    pub include_hidden: bool,

    #[arg(short, long)]
    pub ignore: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    pub markdown: bool,

    #[arg(short, long, default_value = "output.txt")]
    pub output: String,

    #[arg(short, long)]
    pub extension: Option<Vec<String>>,

    #[arg(short, long, default_value_t = false)]
    pub line_numbers: bool,
}

pub fn get_config() -> AppConfig {
    let app_config = AppConfig::parse();
    app_config
}
