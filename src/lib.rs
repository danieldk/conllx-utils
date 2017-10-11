extern crate conllx;

extern crate flate2;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

extern crate petgraph;

mod cmd;
pub use cmd::{open_reader, open_writer, or_exit};

mod graph;
pub use graph::{first_matching_edge, sentence_to_graph, DependencyGraph, DependencyNode};

mod layer;
pub use layer::{LayerCallback, LAYER_CALLBACKS};

mod tdz_morph;
pub use tdz_morph::{expand_tdz_morph, MorphError};

mod unicode;
pub use unicode::{simplify_unicode, simplify_unicode_punct};
