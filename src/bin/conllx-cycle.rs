extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
extern crate itertools;
extern crate petgraph;

use std::env::args;

use conllx::Sentence;
use conllx_utils::{or_exit, or_stdin};
use getopts::Options;
use itertools::Itertools;
use petgraph::{Directed, Graph};
use petgraph::graph::node_index;
use petgraph::algo::scc;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] FILE", program);
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

    let input = or_stdin(matches.free.get(0));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));
    for sentence in reader.sentences() {
        let sentence = or_exit(sentence);
        check_cycles(&sentence, matches.opt_present("p"))
    }
}

fn check_cycles(sentence: &Sentence, projective: bool) {
    let edges = sentence.iter().enumerate().filter_map(|(idx, token)| {
        let head = if projective {
            token.p_head()
        } else {
            token.head()
        };

        match head {
            Some(head) => Some((node_index(head), node_index(idx + 1))),
            None => None,
        }
    });

    let dep_graph = Graph::<(), (), Directed>::from_edges(edges);
    let mut sentence_printed = false;

    for component in scc(&dep_graph) {
        if component.len() == 1 {
            continue;
        }

        if !sentence_printed {
            println!("{}\n", sentence);
            sentence_printed = true
        }

        println!("Cycle: {}",
                 component.iter().map(|i| i.index().to_string()).join(", "));
    }
}
