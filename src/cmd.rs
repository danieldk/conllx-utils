extern crate conllx;

use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::process;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

pub fn create_writer<P>(filename: P, gzip: bool) -> io::Result<conllx::Writer<Box<Write>>>
where
    P: AsRef<Path>,
{
    let file = File::create(filename)?;
    let boxed_writer: Box<Write> = if gzip {
        Box::new(BufWriter::new(GzEncoder::new(file, Compression::Default)))
    } else {
        Box::new(BufWriter::new(file))
    };

    Ok(conllx::Writer::new(boxed_writer))
}

pub fn open_writer<P>(path: &P) -> io::Result<conllx::Writer<Box<Write>>>
where
    P: AsRef<Path>,
{
    let compress = path.as_ref().extension() == Some(OsStr::new("gz"));
    create_writer(path, compress)
}

pub fn open_reader<P>(path: &P) -> io::Result<conllx::Reader<Box<BufRead>>>
where
    P: AsRef<Path>,
{
    let reader = File::open(path)?;

    let boxed_reader: Box<BufRead> = if path.as_ref().extension() == Some(OsStr::new("gz")) {
        Box::new(BufReader::new(GzDecoder::new(reader)?))
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
