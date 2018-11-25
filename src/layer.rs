use std::borrow::Cow;
use std::collections::HashMap;

use conllx::{Features, Token};

pub type LayerCallback = fn(&Token) -> Option<Cow<str>>;

lazy_static! {
    pub static ref LAYER_CALLBACKS: HashMap<&'static str, LayerCallback> = {
        let mut m: HashMap<&'static str, LayerCallback> = HashMap::new();
        m.insert("cpos", cpos);
        m.insert("features", features);
        m.insert("form", form);
        m.insert("lemma", lemma);
        m.insert("pos", pos);
        m
    };
}

fn cpos(t: &Token) -> Option<Cow<str>> {
    t.cpos().map(Cow::Borrowed)
}

fn features(t: &Token) -> Option<Cow<str>> {
    t.features().map(Features::as_str).map(Cow::Borrowed)
}

fn form(t: &Token) -> Option<Cow<str>> {
    Some(Cow::Borrowed(t.form()))
}

fn lemma(t: &Token) -> Option<Cow<str>> {
    t.lemma().map(Cow::Borrowed)
}

fn pos(t: &Token) -> Option<Cow<str>> {
    t.pos().map(Cow::Borrowed)
}
