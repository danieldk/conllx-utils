use std::borrow::Cow;

use conllx_ng::token::{Features, Token};

pub type LayerCallback = Box<dyn Fn(&Token) -> Option<Cow<str>>>;

pub fn layer_callback(layer: &str) -> Option<LayerCallback> {
    match layer {
        "cpos" => Some(Box::new(|t| t.cpos().map(Cow::Borrowed))),
        "features" => Some(Box::new(|t| {
            t.features().map(Features::to_string).map(Cow::Owned)
        })),
        "form" => Some(Box::new(|t| Some(Cow::Borrowed(t.form())))),
        "lemma" => Some(Box::new(|t| t.lemma().map(Cow::Borrowed))),
        "pos" => Some(Box::new(|t| t.pos().map(Cow::Borrowed))),
        _ => None,
    }
}
