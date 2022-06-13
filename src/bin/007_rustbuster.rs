use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::{thread};
use reqwest::{StatusCode};
use std::time::{Duration};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct FileResult {
    path: String,
    status: StatusCode
}

fn main() {
    let reader = BufReader::new(File::open("testfiles/day7_wordlist.txt").expect("Cannot open file.txt"));

    let n_workers = 4;
    let n_jobs = 8;

    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    for line in reader.lines() {
        let tx = tx.clone();
        pool.execute(move|| {
            let url = format!("{}{}{}{}", "https://www.rust-lang.org", "/", line.as_ref().unwrap(), ".html");
            let status = reqwest::blocking::get(&url).unwrap().status();
            let res = FileResult {
                status,
                path: line.unwrap()
            };

            tx.send(res).expect("channel will be there waiting for the pool");
        });
    }

    for result in rx.iter().take(n_jobs) {
        println!("{}.html [{}]", result.path, result.status);
    }

    thread::sleep(Duration::from_secs(2));
    println!("program is done");
}
