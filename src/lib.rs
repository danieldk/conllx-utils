extern crate conllx;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

extern crate rand;

mod cmd;
pub use cmd::{or_exit, or_stdin, or_stdout};

mod layer;
pub use layer::{LAYER_CALLBACKS, LayerCallback};

mod tdz_morph;
pub use tdz_morph::{MorphError, expand_tdz_morph};

mod unicode;
pub use unicode::{simplify_unicode_punct, simplify_unicode};
