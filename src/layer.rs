use std::borrow::Cow;

use conllx::{Features, Token};

pub type LayerCallback = Box<dyn Fn(&Token) -> Option<Cow<str>>>;

pub fn layer_callback(layer: &str) -> Option<LayerCallback> {
    match layer {
        "cpos" => Some(Box::new(|t| t.cpos().map(Cow::Borrowed))),
        "features" => Some(Box::new(|t| {
            t.features().map(Features::as_str).map(Cow::Borrowed)
        })),
        "form" => Some(Box::new(|t| Some(Cow::Borrowed(t.form())))),
        "head" => Some(Box::new(|t| {
            t.head().map(|h| h.to_string()).map(Cow::Owned)
        })),
        "headrel" => Some(Box::new(|t| t.head_rel().map(Cow::Borrowed))),
        "phead" => Some(Box::new(|t| {
            t.p_head().map(|h| h.to_string()).map(Cow::Owned)
        })),
        "pheadrel" => Some(Box::new(|t| t.p_head_rel().map(Cow::Borrowed))),
        "lemma" => Some(Box::new(|t| t.lemma().map(Cow::Borrowed))),
        "pos" => Some(Box::new(|t| t.pos().map(Cow::Borrowed))),
        _ => None,
    }
}
