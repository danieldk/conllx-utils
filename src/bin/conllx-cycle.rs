extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate itertools;
extern crate petgraph;
extern crate stdinout;

use std::env::args;

use conllx::Projectivity;
use conllx::Sentence;
use conllx_utils::or_exit;
use getopts::Options;
use itertools::Itertools;
use petgraph::algo::tarjan_scc;
use petgraph::visit::EdgeFiltered;
use stdinout::Input;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [FILE]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("p", "projective", "find cycles in the projective column");
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 1 {
        print_usage(&program, opts);
        return;
    }

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));
    for sentence in reader {
        let sentence = or_exit(sentence);

        let projectivity = if matches.opt_present("p") {
            Projectivity::Projective
        } else {
            Projectivity::NonProjective
        };

        check_cycles(&sentence, projectivity);
    }
}

fn check_cycles(sentence: &Sentence, projectivity: Projectivity) {
    let dep_graph = EdgeFiltered::from_fn(sentence.get_ref(), |e| e.weight().0 == projectivity);

    let mut sentence_printed = false;
    for component in tarjan_scc(&dep_graph) {
        if component.len() == 1 {
            continue;
        }

        if !sentence_printed {
            println!("{}\n", sentence);
            sentence_printed = true
        }

        println!(
            "Cycle: {}",
            component.iter().map(|i| i.index().to_string()).join(", ")
        );
    }
}
