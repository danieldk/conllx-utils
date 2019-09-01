use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use conllx::io::Reader;
use flate2::read::GzDecoder;

pub fn open_reader<P>(path: &P) -> io::Result<Reader<Box<dyn BufRead>>>
where
    P: AsRef<Path>,
{
    let f = File::open(path)?;

    let boxed_reader: Box<dyn BufRead> = if path.as_ref().extension() == Some(OsStr::new("gz")) {
        Box::new(BufReader::new(GzDecoder::new(f)))
    } else {
        Box::new(BufReader::new(f))
    };

    Ok(Reader::new(boxed_reader))
}
