use std::collections::HashSet;

use conllx::graph::{DepGraph, DepTriple, Node, Sentence};

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
    let mut g = sentence.graph_mut();

    for triple in updates {
        g.add_deprel(triple);
    }
}

/// Given a node `verb` that represents a verb, find the content
/// (non-auxiliary/model) verb. If the given verb is already a
/// content verb, the index of the verb itself is returned.
fn resolve_verb(graph: &DepGraph, verb: usize) -> usize {
    // Look for non-aux.
    match graph
        .dependents(verb)
        .filter(|t| t.relation() == Some(AUXILIARY_RELATION))
        .next()
    {
        Some(triple) => resolve_verb(graph, triple.dependent()),
        None => verb,
    }
}

/// Find PPs that are attached to an auxiliary finite verb and
/// need re-attachment. This function returns tuples containing
///
/// 1. The index into the sentence of a PP requiring re-attachment.
/// 2. The index into the sentence of the re-attachment site.
fn find_reattachments(sentence: &Sentence) -> Vec<DepTriple<String>> {
    let mut updates = Vec::new();
    let g = sentence.graph();

    for i in 0..sentence.len() {
        let triple = ok_or_continue!(g.head(i));

        // Skip unlabeled edges.
        let rel = ok_or_continue!(triple.relation());

        // We are only interested in PP/OBJP edges.
        if !PP_RELATIONS.contains(rel) {
            continue;
        }

        let head = if let Node::Token(ref token) = sentence[triple.head()] {
            token
        } else {
            continue;
        };

        // Check that the head is a verb.
        let tag = ok_or_continue!(head.pos());
        if !tag.starts_with(VERB_PREFIX) {
            continue;
        }

        let content_verb_idx = resolve_verb(&g, triple.head());
        if content_verb_idx != triple.head() {
            updates.push(DepTriple::new(
                content_verb_idx,
                triple.relation().map(ToOwned::to_owned),
                triple.dependent(),
            ));
        }
    }

    updates
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;

    use conllx::io::{ReadSentence, Reader};

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
