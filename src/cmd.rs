use std::fmt;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::{BufRead, BufReader};
use std::process;

pub enum Input {
    Stdin(io::Stdin),
    File(PathBuf),
}

impl<'a> Input {
    pub fn buf_read(&'a self) -> io::Result<Box<BufRead + 'a>> {
        match self {
            &Input::Stdin(ref stdin) => Result::Ok(Box::new(stdin.lock())),
            &Input::File(ref path) => Result::Ok(Box::new(BufReader::new(try!(File::open(path))))),
        }
    }
}

pub fn or_exit<T, E: fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e: E| -> T {
        println!("Error: {}", e);
        process::exit(1)
    })
}

pub fn or_stdin(filename: Option<&String>) -> Input {
    match filename {
        Some(n) => {
            Input::File(Path::new(n).to_owned())
        }
        None => Input::Stdin(io::stdin()),
    }
}

pub fn or_stdout(filename: Option<&String>) -> Box<io::Write> {
    match filename {
        Some(n) => Box::new(io::BufWriter::new(or_exit(fs::File::create(n)))),
        None => Box::new(io::BufWriter::new(io::stdout())),
    }
}
