use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
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

pub fn or_stdin(filename: Option<&String>) -> Input {
    match filename {
        Some(n) => Input::File(Path::new(n).to_owned()),
        None => Input::Stdin(io::stdin()),
    }
}

pub enum Output {
    Stdout(io::Stdout),
    File(PathBuf),
}

impl<'a> Output {
    pub fn buf_write(&'a self) -> io::Result<Box<Write + 'a>> {
        match self {
            &Output::Stdout(ref stdout) => Result::Ok(Box::new(BufWriter::new(stdout.lock()))),
            &Output::File(ref path) => {
                Result::Ok(Box::new(BufWriter::new(try!(File::create(path)))))
            }
        }
    }
}

pub fn or_stdout(filename: Option<&String>) -> Output {
    match filename {
        Some(n) => Output::File(Path::new(n).to_owned()),
        None => Output::Stdout(io::stdout()),
    }
}

pub fn or_exit<T, E: fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e: E| -> T {
        println!("Error: {}", e);
        process::exit(1)
    })
}
