use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use std::{thread};
use reqwest::{StatusCode};
use std::time::{Duration};

struct FileResult {
    path: String,
    status: StatusCode
}

fn main() {
    let wordlist = ["aaa", "bbb", "ccc", "index"];

    let n_workers = 4;
    let n_jobs = 8;

    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel();

    for line in wordlist {
        let tx = tx.clone();
        pool.execute(move|| {
            let url = format!("{}{}{}{}", "https://www.rust-lang.org", "/", line, ".html");
            let status = reqwest::blocking::get(&url).unwrap().status();
            let res = FileResult {
                status,
                path: line.parse().unwrap()
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
