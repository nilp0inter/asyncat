use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    if args.is_empty() || args.contains(&"-".to_string()) {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let stdin = io::stdin();
            let reader = stdin.lock();
            for line in reader.lines() {
                let line = line.expect("error reading line from stdin");
                tx_clone.send(line).expect("error sending line from stdin");
            }
        });
        handles.push(handle);
    }

    for path in args {
        if path == "-" {
            continue; // Skip "-" since we've already handled stdin.
        }

        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let file = File::open(&path).expect("cannot open file");
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line.expect("error reading line from file");
                tx_clone.send(line).expect("error sending line from file");
            }
        });
        handles.push(handle);
    }

    drop(tx); // Drop the original sender so the receiver knows when to stop.

    for received in rx {
        println!("{}", received);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())
}
