use std::io::{self, BufRead};

use clap::{App, AppSettings, Arg};
use conllx::{PartitioningWriter, Sentence, WriteSentence};
use conllx_utils::{open_writer, or_exit};
use rand::{Rng, StdRng};
use std::process;
use stdinout::{Input, OrExit};

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

static INPUT: &str = "INPUT";
static PARTS: &str = "PARTS";
static PREFIX: &str = "PREFIX";
static RANDOM: &str = "RANDOM";
static SUFFIX: &str = "SUFFIX";

struct RandomPartitioningWriter<R, W>
where
    R: Rng,
    W: WriteSentence,
{
    rng: R,
    writers: Vec<W>,
}

impl<R, W> RandomPartitioningWriter<R, W>
where
    R: Rng,
    W: WriteSentence,
{
    pub fn new(rng: R, writers: Vec<W>) -> Self {
        RandomPartitioningWriter { rng, writers }
    }
}

impl<R, W> WriteSentence for RandomPartitioningWriter<R, W>
where
    R: Rng,
    W: WriteSentence,
{
    fn write_sentence(&mut self, sentence: &Sentence) -> Result<(), io::Error> {
        let fold = self.rng.gen_range(0, self.writers.len());
        self.writers[fold].write_sentence(sentence)
    }
}

pub struct PartitionApp {
    input: Option<String>,
    parts: usize,
    prefix: String,
    random: bool,
    suffix: String,
}

impl PartitionApp {
    fn new() -> Self {
        let matches = App::new("conllx-partition")
            .settings(DEFAULT_CLAP_SETTINGS)
            .arg(Arg::with_name(PARTS).help("Output data").index(1))
            .arg(
                Arg::with_name(PREFIX)
                    .help("Output data")
                    .index(2)
                    .required(true),
            )
            .arg(
                Arg::with_name(SUFFIX)
                    .help("Output data")
                    .index(3)
                    .required(true),
            )
            .arg(Arg::with_name(INPUT).help("Input data").index(4))
            .arg(
                Arg::with_name(RANDOM)
                    .long("random")
                    .short("r")
                    .help("Write to random partitions"),
            )
            .get_matches();

        let parts = matches
            .value_of(PARTS)
            .unwrap()
            .parse()
            .or_exit("Cannot parse number of parts", 1);
        if parts == 0 {
            eprintln!("Input should be split into at least one part");
            process::exit(1);
        }

        let prefix = matches.value_of(PREFIX).unwrap().to_owned();
        let suffix = matches.value_of(SUFFIX).unwrap().to_owned();
        let input = matches.value_of(INPUT).map(ToOwned::to_owned);

        PartitionApp {
            input,
            parts,
            prefix,
            random: matches.is_present(RANDOM),
            suffix,
        }
    }
}

fn main() {
    let app = PartitionApp::new();

    let input = Input::from(app.input.as_ref());
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let writers: Vec<_> = (0..app.parts)
        .map(|part| {
            or_exit(open_writer(&format!(
                "{}{}{}",
                app.prefix, part, app.suffix
            )))
        })
        .collect();

    let mut writer: Box<dyn WriteSentence> = if app.random {
        Box::new(RandomPartitioningWriter::new(
            StdRng::new().or_exit("Cannot initialize RNG from entropy", 1),
            writers,
        ))
    } else {
        Box::new(PartitioningWriter::new(writers))
    };

    copy_sents(reader, &mut *writer)
}

fn copy_sents<R>(reader: conllx::Reader<R>, writer: &mut dyn WriteSentence)
where
    R: BufRead,
{
    for sentence in reader {
        let sentence = or_exit(sentence);
        or_exit(writer.write_sentence(&sentence))
    }
}
