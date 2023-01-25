use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use reqwest::{StatusCode};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use tokio::time::Instant;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// File path to the wordlist
    #[clap(short, long)]
    wordlist: String,

    /// Hostname to be tested
    #[clap(short, long)]
    url: String,

    /// File extensions
    #[clap(short, long, multiple_values=true)]
    extension: Vec<String>
}

struct FileResult {
    path: String,
    status: StatusCode,
    extension: String
}

fn get_line_count(file_path: &str) -> usize {
    let input = File::open(file_path).expect("Cannot open wordlist");
    let buffered = BufReader::new(input);
    let line_count = buffered.lines().count();
    line_count
}

fn main() {
    let now = Instant::now();
    let args = Args::parse();

    let line_count = get_line_count(&args.wordlist);
    let extension_count = &args.extension.len();

    let reader = BufReader::new(File::open(args.wordlist).expect("Cannot open wordlist"));

    let n_workers = 20;

    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    for line in reader.lines() {
        let l = line.unwrap();
        for extension in &args.extension {
            let tx = tx.clone();
            let url = args.url.to_owned().clone();
            let ext = extension.to_owned().clone();
            let m = l.clone();
            pool.execute(move|| {
                let url = format!("{}{}{}.{}", url, "/", m, ext);
                let status = reqwest::blocking::get(&url).unwrap().status();
                let res = FileResult {
                    status,
                    path: m.to_string(),
                    extension: ext.to_string()
                };

                tx.send(res).expect("channel will be there waiting for the pool");
            });
        }
    }

    for result in rx.iter().take(line_count * extension_count) {
        println!("{}.{} [{}]", result.path, result.extension, result.status);
    }

    println!("Done. [{} seconds]", now.elapsed().as_secs());
}
