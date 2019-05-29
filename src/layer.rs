use std::borrow::Cow;

use conllx::graph::Sentence;
use conllx::token::{Features, Token};

pub type LayerCallback<'a> = Box<Fn(&'a Sentence, usize) -> Option<Cow<'a, str>>>;

pub fn layer_callback(layer: &str) -> Option<LayerCallback> {
    match layer {
        "cpos" => Some(Box::new(|s, idx| {
            s[idx].token().and_then(Token::cpos).map(Cow::Borrowed)
        })),
        "features" => Some(Box::new(|s, idx| {
            s[idx]
                .token()
                .and_then(Token::features)
                .map(Features::as_str)
                .map(Cow::Borrowed)
        })),
        "form" => Some(Box::new(|s, idx| {
            s[idx].token().map(|t| Cow::Borrowed(t.form()))
        })),
        "head" => Some(Box::new(|s, idx| {
            s.dep_graph()
                .head(idx)
                .map(|t| t.head().to_string())
                .map(Cow::Owned)
        })),
        "headrel" => Some(Box::new(|s, idx| {
            s.dep_graph()
                .head(idx)
                .and_then(|t| t.relation())
                .map(Cow::Borrowed)
        })),
        "phead" => Some(Box::new(|s, idx| {
            s.proj_dep_graph()
                .head(idx)
                .map(|t| t.head().to_string())
                .map(Cow::Owned)
        })),
        "pheadrel" => Some(Box::new(|s, idx| {
            s.proj_dep_graph()
                .head(idx)
                .and_then(|t| t.relation())
                .map(Cow::Borrowed)
        })),
        "lemma" => Some(Box::new(|s, idx| {
            s[idx].token().and_then(Token::lemma).map(Cow::Borrowed)
        })),
        "pos" => Some(Box::new(|s, idx| {
            s[idx].token().and_then(Token::pos).map(Cow::Borrowed)
        })),
        _ => None,
    }
}
