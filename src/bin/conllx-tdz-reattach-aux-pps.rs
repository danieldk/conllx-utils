extern crate conllx;
extern crate conllx_utils;
extern crate getopts;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate petgraph;
extern crate stdinout;

use std::collections::HashSet;
use std::env::args;
use std::io::BufWriter;

use conllx::{Sentence, WriteSentence};
use conllx_utils::{first_matching_edge, or_exit, sentence_to_graph, DependencyGraph};
use getopts::Options;
use petgraph::EdgeDirection;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use stdinout::{Input, Output};

macro_rules! ok_or_continue {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => continue,
    })
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [INPUT_FILE] [OUTPUT_FILE]", program);
    print!("{}", opts.usage(&brief));
}

static AUXILIARY_RELATION: &'static str = "AUX";
static PP_RELATION: &'static str = "PP";
static POBJ_RELATION: &'static str = "OBJP";

static FINITE_VERB_TAG: &'static str = "VVFIN";
static FINITE_AUXILIARY_TAG: &'static str = "VAFIN";
static FINITE_MODAL_TAG: &'static str = "VMFIN";

lazy_static! {
    static ref FINITE_VERB_TAGS: HashSet<&'static str> = hashset!{
        FINITE_VERB_TAG,
        FINITE_AUXILIARY_TAG,
        FINITE_MODAL_TAG
    };

    static ref PP_RELATIONS: HashSet<&'static str> = hashset! {
        PP_RELATION,
        POBJ_RELATION
    };
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag(
        "n",
        "no-preserve",
        "do not preserve original TüBa morph tag",
    );
    let matches = or_exit(opts.parse(&args[1..]));

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        return;
    }

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(or_exit(input.buf_read()));

    let output = Output::from(matches.free.get(1));
    let mut writer = conllx::Writer::new(BufWriter::new(or_exit(output.write())));
    for sentence in reader {
        let mut sentence = or_exit(sentence);
        reattach_fronted_pps(&mut sentence);
        or_exit(writer.write_sentence(&sentence))
    }
}

/// Re-attached PPs headed by an auxiliary/model-verb. In the TüBa-D/Z
/// these are normally topicalized PPs.
fn reattach_fronted_pps(sentence: &mut Sentence) {
    let updates = find_reattachments(&sentence);

    for (prep_offset, new_head) in updates {
        sentence.as_tokens_mut()[prep_offset].set_head(Some(new_head + 1));
    }
}

/// Given a node `verb` that represents a verb, find the content
/// (non-auxiliary/model) verb. If the given verb is already a
/// content verb, the index of the verb itself is returned.
fn resolve_verb(graph: &DependencyGraph, verb: NodeIndex) -> NodeIndex {
    // Look for non-aux.
    match first_matching_edge(graph, verb, EdgeDirection::Outgoing, |e| {
        *e == Some(AUXILIARY_RELATION)
    }) {
        Some(idx) => resolve_verb(graph, idx),
        None => verb,
    }
}

/// Find PPs that are attached to an auxiliary finite verb and
/// need re-attachment. This function returns tuples containing
///
/// 1. The index into the sentence of a PP requiring re-attachment.
/// 2. The index into the sentence of the re-attachment site.
fn find_reattachments(sentence: &Sentence) -> Vec<(usize, usize)> {
    let graph = sentence_to_graph(&sentence, false);

    let mut updates = Vec::new();

    for edge_ref in graph.edge_references() {
        // Skip unlabeled edges.
        let weight = ok_or_continue!(*edge_ref.weight());

        // We are only interested in PP/OBJP edges.
        if !PP_RELATIONS.contains(weight) {
            continue;
        }

        let head_node = &graph[edge_ref.source()];

        // Check that the head is a finite verb.
        let tag = ok_or_continue!(head_node.token.pos());
        if !FINITE_VERB_TAGS.contains(tag) {
            continue;
        }

        let content_verb_idx = resolve_verb(&graph, edge_ref.source());
        if content_verb_idx != edge_ref.source() {
            let prep_offset = graph[edge_ref.target()].offset;
            let content_verb_offset = graph[content_verb_idx].offset;

            updates.push((prep_offset, content_verb_offset));
        }
    }

    updates
}
