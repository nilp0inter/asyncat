use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: <program> <file1> <file2> ...");
        std::process::exit(1);
    }

    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();

    for path in args {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let file = File::open(&path).expect("cannot open file");
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line.expect("error reading line");
                tx_clone.send(line).expect("error sending line");
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
