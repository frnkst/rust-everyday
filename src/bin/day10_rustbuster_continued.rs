use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::{thread};
use reqwest::{StatusCode};
use std::time::{Duration};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

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

fn main() {
    let args = Args::parse();

    let reader = BufReader::new(File::open(args.wordlist).expect("Cannot open wordlist"));

    let n_workers = 4;

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
                //println!("url is {}", url);
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

    for result in rx.iter().take(100) {
        println!("{}.{} [{}]", result.path, result.extension, result.status);
    }

    thread::sleep(Duration::from_secs(2));
    println!("program is done");
}
