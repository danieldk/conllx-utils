extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate stdinout;

use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use conllx::WriteSentence;
use conllx_utils::or_exit;
use getopts::Options;
use stdinout::Output;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE...", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("w", "", "write to file", "NAME");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    }

    let output = Output::from(matches.opt_str("w").as_ref());
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));

    copy_sents(&mut writer, &matches.free)
}

fn copy_sents<W>(writer: &mut conllx::Writer<W>, filenames: &Vec<String>)
    where W: Write
{
    for filename in filenames {
        let file = or_exit(File::open(&filename));
        let buf_read = Box::new(BufReader::new(file));

        let reader = conllx::Reader::new(buf_read);
        for sentence in reader {
            let sentence = or_exit(sentence);
            or_exit(writer.write_sentence(&sentence))
        }
    }
}
