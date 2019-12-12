use std::io::BufWriter;

use clap::{App, AppSettings, Arg};
use conllx_ng::io::{WriteSentence, Writer};
use conllx_utils::io::open_reader;
use stdinout::{OrExit, Output};

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

static INPUTS: &str = "INPUTS";
static OUTPUT: &str = "OUTPUT";

pub struct MergeApp {
    inputs: Vec<String>,
    output: Option<String>,
}

impl MergeApp {
    fn new() -> Self {
        let matches = App::new("conllx-from-text")
            .settings(DEFAULT_CLAP_SETTINGS)
            .arg(
                Arg::with_name(INPUTS)
                    .help("Input corpora")
                    .required(true)
                    .min_values(1),
            )
            .arg(
                Arg::with_name(OUTPUT)
                    .short("w")
                    .takes_value(true)
                    .help("Write merged corpus to a file"),
            )
            .get_matches();

        let inputs = matches
            .values_of(INPUTS)
            .unwrap()
            .map(ToOwned::to_owned)
            .collect();
        let output = matches.value_of(OUTPUT).map(ToOwned::to_owned);

        MergeApp { inputs, output }
    }
}

fn main() {
    let app = MergeApp::new();

    let output = Output::from(app.output);
    let mut writer = Writer::new(BufWriter::new(
        output.write().or_exit("Cannot open output for writing", 1),
    ));

    copy_sents(&mut writer, &app.inputs)
}

fn copy_sents(writer: &mut impl WriteSentence, filenames: &[String]) {
    for filename in filenames {
        let reader = open_reader(&filename).or_exit("Cannot open file for reading", 1);

        for sentence in reader {
            let sentence = sentence.or_exit("Cannot read sentence", 1);
            writer
                .write_sentence(&sentence)
                .or_exit("Cannot write sentence", 1);
        }
    }
}
