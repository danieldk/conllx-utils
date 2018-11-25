use std::collections::HashSet;

use conllx::Sentence;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::EdgeDirection;

use {first_matching_edge, sentence_to_graph, DependencyGraph};

macro_rules! ok_or_continue {
    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => continue,
        }
    };
}

static AUXILIARY_RELATION: &'static str = "AUX";
static PP_RELATION: &'static str = "PP";
static POBJ_RELATION: &'static str = "OBJP";

const VERB_PREFIX: char = 'V';

lazy_static! {
    static ref PP_RELATIONS: HashSet<&'static str> = hashset! {
        PP_RELATION,
        POBJ_RELATION
    };
}

/// Re-attached PPs headed by an auxiliary/model-verb. In the TüBa-D/Z
/// these are normally topicalized PPs.
pub fn reattach_aux_pps(sentence: &mut Sentence) {
    let updates = find_reattachments(&sentence);

    for (prep_offset, new_head) in updates {
        sentence[prep_offset].set_head(Some(new_head + 1));
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

        // Check that the head is a verb.
        let tag = ok_or_continue!(head_node.token.pos());
        if !tag.starts_with(VERB_PREFIX) {
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    use conllx::{ReadSentence, Reader};

    use reattach_aux_pps;

    static ORIGINAL_DATA: &'static str = "testdata/tdz-100.conll";
    static GOLD_REATTACHMENT_DATA: &'static str = "testdata/tdz-100-pp-reattach.conll";

    #[test]
    fn pp_reattachment() {
        let reader = Reader::new(BufReader::new(File::open(ORIGINAL_DATA).unwrap()));
        let gold_reader = Reader::new(BufReader::new(File::open(GOLD_REATTACHMENT_DATA).unwrap()));

        for (sentence, gold_sentence) in reader.sentences().zip(gold_reader.sentences()) {
            let mut sentence = sentence.unwrap();
            let gold_sentence = gold_sentence.unwrap();

            reattach_aux_pps(&mut sentence);

            assert_eq!(sentence, gold_sentence);
        }
    }
}
