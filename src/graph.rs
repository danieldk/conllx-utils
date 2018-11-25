use conllx::{Sentence, Token};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::{Directed, EdgeDirection, Graph};

#[derive(Debug)]
pub struct DependencyNode<'a> {
    pub token: &'a Token,
    pub offset: usize,
}

pub type DependencyGraph<'a> = Graph<DependencyNode<'a>, Option<&'a str>, Directed>;

pub fn sentence_to_graph(sentence: &Sentence, projective: bool) -> DependencyGraph {
    let mut g = Graph::new();

    let nodes: Vec<_> = sentence
        .iter()
        .enumerate()
        .map(|(offset, token)| {
            g.add_node(DependencyNode {
                token: token,
                offset: offset,
            })
        }).collect();

    for (idx, token) in sentence.iter().enumerate() {
        let head = if projective {
            token.p_head()
        } else {
            token.head()
        };

        let rel = if projective {
            token.p_head_rel()
        } else {
            token.head_rel()
        };

        if let Some(head) = head {
            if head != 0 {
                g.add_edge(nodes[head - 1], nodes[idx], rel);
            }
        }
    }

    g
}

pub fn first_matching_edge<F>(
    graph: &DependencyGraph,
    index: NodeIndex,
    direction: EdgeDirection,
    predicate: F,
) -> Option<NodeIndex>
where
    F: Fn(&Option<&str>) -> bool,
{
    graph
        .edges_directed(index, direction)
        .find(|edge_ref| predicate(edge_ref.weight()))
        .map(|edge_ref| edge_ref.target())
}

/*
pub fn sentence_to_labeled_graph(sentence: &Sentence) -> Result<Graph<(), String, Directed>> {
    let mut edges = Vec::with_capacity(sentence.as_tokens().len() + 1);
    for (idx, token) in sentence.iter().enumerate() {
        let (head, dependent) = match token.head() {
            Some(head) => (node_index(head), node_index(idx + 1)),
            None => continue,
        };

        let head_rel = match token.head_rel() {
            Some(head_rel) => head_rel,
            None => {
                return Err(
                    IncompleteGraphError(format!(
                        "edge from {} to {} does not have a \
                         label",
                        head.index(),
                        dependent.index()
                    )).into(),
                )
            }
        };

        edges.push((head, dependent, head_rel.to_owned()))
    }

    Ok(Graph::<(), String, Directed>::from_edges(edges))
}

*/
