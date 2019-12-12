use std::borrow::Cow;
use std::io::{BufWriter, Write};

use clap::{App, AppSettings, Arg};
use conllx_utils::layer_ng::{layer_callback, LayerCallback};
use itertools::Itertools;
use stdinout::{Input, OrExit, Output};

static DEFAULT_CLAP_SETTINGS: &[AppSettings] = &[
    AppSettings::DontCollapseArgsInUsage,
    AppSettings::UnifiedHelpMessage,
];

static INPUT: &str = "INPUT";
static OUTPUT: &str = "OUTPUT";
static LAYER: &str = "LAYER";

pub struct ToTextApp {
    input: Option<String>,
    output: Option<String>,
    layer_callback: LayerCallback,
}

impl ToTextApp {
    fn new() -> Self {
        let matches = App::new("conllx-from-text")
            .settings(DEFAULT_CLAP_SETTINGS)
            .arg(Arg::with_name(INPUT).help("Input data").index(1))
            .arg(Arg::with_name(OUTPUT).help("Output data").index(2))
            .arg(
                Arg::with_name(LAYER)
                    .short("l")
                    .possible_values(&["form", "lemma", "cpos", "pos"])
                    .default_value("form")
                    .help("Layer to output as text"),
            )
            .get_matches();

        let input = matches.value_of(INPUT).map(ToOwned::to_owned);
        let output = matches.value_of(OUTPUT).map(ToOwned::to_owned);
        let layer_callback =
            layer_callback(matches.value_of(LAYER).unwrap()).expect("Unknown layer");

        ToTextApp {
            input,
            output,
            layer_callback,
        }
    }
}

fn main() {
    let app = ToTextApp::new();

    let input = Input::from(app.input);
    let reader = conllx_ng::io::Reader::new(input.buf_read().or_exit("Cannot open input", 1));

    let output = Output::from(app.output);
    let mut writer = BufWriter::new(output.write().or_exit("Cannot open output", 1));

    let callback = app.layer_callback;
    for sentence in reader {
        let sentence = sentence.or_exit("Cannot read sentence", 1);

        writeln!(
            writer,
            "{}",
            sentence
                .iter()
                .filter_map(|n| n.token().map(|t| callback(t)
                    .map(Cow::into_owned)
                    .unwrap_or_else(|| "_".to_owned())))
                .join(" ")
        )
        .or_exit("Cannot write sentence", 1);
    }
}
