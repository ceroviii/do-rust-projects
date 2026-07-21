use crate::searcher::learn_threads;
use clap::Parser;

mod searcher;
mod walker;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    pattern: String,
    #[arg(short, long)]
    dir: String,
    #[arg(short, long)]
    extension: Option<String>,
    #[arg(short, long)]
    threads: Option<u8>,
    #[arg(short, long)]
    case_insensitive: Option<bool>,
}

fn main() {
    let fs_cli = Cli::parse();

    // let pattern = fs_cli.pattern;
    // let dir_to_search = fs_cli.dir;
    let mut file_extension = String::new();
    let mut no_threads = 0_u8;
    let mut is_case_sensitive = false;

    if fs_cli.case_insensitive.is_some() {
        is_case_sensitive = true;
        println!("case_insensitive");
    }
    if fs_cli.extension.is_some() {
        file_extension = fs_cli.extension.unwrap();
    }
    if fs_cli.threads.is_some() {
        no_threads = fs_cli.threads.unwrap();
    }

    println!(
        "pat={} dir={} ext={} not={} ics={}",
        fs_cli.pattern, fs_cli.dir, file_extension, no_threads, is_case_sensitive
    );

    let files_to_search = walker::dir_walk(fs_cli.dir, file_extension);
    println!("{:?}", files_to_search);

    learn_threads();
}
