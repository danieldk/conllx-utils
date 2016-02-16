use std::fmt;
use std::fs;
use std::io;
use std::process;

pub fn or_exit<T, E: fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e: E| -> T {
        println!("Error: {}", e);
        process::exit(1)
    })
}

pub fn or_stdin(filename: Option<&String>) -> Box<io::BufRead> {
    match filename {
        Some(n) => {
            let file = or_exit(fs::File::open(n));
            Box::new(io::BufReader::new(file))
        }
        None => Box::new(io::BufReader::new(io::stdin())),
    }
}

pub fn or_stdout(filename: Option<&String>) -> Box<io::Write> {
    match filename {
        Some(n) => Box::new(io::BufWriter::new(or_exit(fs::File::create(n)))),
        None => Box::new(io::BufWriter::new(io::stdout())),
    }
}
