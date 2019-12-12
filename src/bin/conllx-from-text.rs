use std::io::{BufRead, BufWriter};

use clap::{App, AppSettings, Arg};
use conllx_ng::graph::Sentence;
use conllx_ng::io::WriteSentence;
use conllx_ng::token::TokenBuilder;
use stdinout::{Input, OrExit, Output};

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

static INPUT: &str = "INPUT";
static OUTPUT: &str = "OUTPUT";

pub struct FromTextApp {
    input: Option<String>,
    output: Option<String>,
}

impl FromTextApp {
    fn new() -> Self {
        let matches = App::new("conllx-from-text")
            .settings(DEFAULT_CLAP_SETTINGS)
            .arg(Arg::with_name(INPUT).help("Input data").index(1))
            .arg(Arg::with_name(OUTPUT).help("Output data").index(2))
            .get_matches();

        let input = matches.value_of(INPUT).map(ToOwned::to_owned);
        let output = matches.value_of(OUTPUT).map(ToOwned::to_owned);

        FromTextApp { input, output }
    }
}

fn main() {
    let app = FromTextApp::new();

    let input = Input::from(app.input);
    let reader = input.buf_read().or_exit("Cannot open input", 1);

    let output = Output::from(app.output);
    let mut writer = conllx_ng::io::Writer::new(BufWriter::new(
        output.write().or_exit("Cannot open output", 1),
    ));

    for line in reader.lines() {
        let line = line.or_exit("Cannot read sentence", 1);
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        let sentence: Sentence = trimmed
            .split(' ')
            .map(|t| TokenBuilder::new(t).into())
            .collect();

        if sentence.len() != 1 {
            writer
                .write_sentence(&sentence)
                .or_exit("Cannot write sentence", 1);
        }
    }
}
