use html_rs as _;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                job();
            }
        });

        Worker { id, thread }
    }
}

static PERFORMED_REQUESTS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

// static PERFORMED_REQUESTS: LazyLock<AtomicUsize> = LazyLock::new(AtomicUsize::default);

fn main() {
    let url = "localhost:8080";
    let num_requests = 100;
    let concurrency = 30;

    let status_codes = Arc::new(Mutex::new(HashMap::new()));
    let total_time = Arc::new(Mutex::new(Duration::from_secs(0)));

    let pool = ThreadPool::new(concurrency);

    for _ in 0..num_requests {
        let status_codes = Arc::clone(&status_codes);
        let total_time = Arc::clone(&total_time);
        let url = url.to_string();

        pool.execute(move || {
            PERFORMED_REQUESTS.fetch_add(1, Ordering::SeqCst);

            let mut buf: [u8; 1024] = [0; 1024];
            let start = Instant::now();
            let mut stream = TcpStream::connect(&url).unwrap();
            stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
            let num_bytes = stream.read(&mut buf).unwrap();
            let duration = start.elapsed();

            let mut status_codes = status_codes.lock().unwrap();
            let mut total_time = total_time.lock().unwrap();
            let status_code = get_status_code(&buf[..num_bytes]);
            *status_codes.entry(status_code).or_insert(0) += 1;
            *total_time += duration;

            let performed = PERFORMED_REQUESTS.load(Ordering::SeqCst);
            println!("Request #{} - {}", performed, status_code);
        });
    }

    for worker in pool.workers {
        worker.thread.join().unwrap();
    }

    println!("Summary:");
    println!(
        "  Total requests: {}",
        PERFORMED_REQUESTS.load(Ordering::Relaxed)
    );
    let total_time = total_time.lock().unwrap();
    println!("  Total time: {} ms", total_time.as_millis());
    println!(
        "  Requests per second: {}",
        PERFORMED_REQUESTS.load(Ordering::Relaxed) as f64 / total_time.as_secs_f64()
    );
    let status_codes = status_codes.lock().unwrap();
    println!("  Status codes:");
    for (code, count) in status_codes.iter() {
        println!("    {} - {}", code, count);
    }
}

fn get_status_code(response: &[u8]) -> u16 {
    let mut lines = response.split(|&b| b == b'\n');
    if let Some(status_line) = lines.next() {
        let response_str = String::from_utf8_lossy(status_line);
        let status_line = response_str.split("\r\n").next().unwrap();
        if let Some(code) = status_line.split(" ").nth(1) {
            return code.parse().unwrap_or(0);
        }
    }
    0
}
