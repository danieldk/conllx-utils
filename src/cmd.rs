extern crate conllx;

use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;

use flate2::FlateReadExt;

pub fn open_reader<P>(path: &P) -> io::Result<conllx::Reader<Box<BufRead>>>
where
    P: AsRef<Path>,
{
    let reader = File::open(path)?;

    let boxed_reader: Box<BufRead> = if path.as_ref().extension() == Some(OsStr::new("gz")) {
        Box::new(BufReader::new(reader.gz_decode()?))
    } else {
        Box::new(BufReader::new(reader))
    };

    Ok(conllx::Reader::new(boxed_reader))
}

pub fn or_exit<T, E: fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e: E| -> T {
        println!("Error: {}", e);
        process::exit(1)
    })
}
