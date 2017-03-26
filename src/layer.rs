use std::borrow::Cow;
use std::collections::HashMap;

use conllx::{Features, Token};

pub type LayerCallback = fn(&Token) -> Option<Cow<str>>;

lazy_static! {
    pub static ref LAYER_CALLBACKS: HashMap<&'static str, LayerCallback> =
        {
            let mut m: HashMap<&'static str, LayerCallback> = HashMap::new();
            m.insert("cpos", cpos);
            m.insert("features", features);
            m.insert("form", form);
            m.insert("head", head);
            m.insert("headrel", head_rel);
            m.insert("phead", p_head);
            m.insert("pheadrel", p_head_rel);
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
    t.form().map(Cow::Borrowed)

}

fn head(t: &Token) -> Option<Cow<str>> {
    t.head().map(|h| h.to_string()).map(Cow::Owned)
}

fn head_rel(t: &Token) -> Option<Cow<str>> {
    t.head_rel().map(Cow::Borrowed)
}

fn p_head(t: &Token) -> Option<Cow<str>> {
    t.p_head().map(|h| h.to_string()).map(Cow::Owned)
}

fn p_head_rel(t: &Token) -> Option<Cow<str>> {
    t.p_head_rel().map(Cow::Borrowed)
}

fn lemma(t: &Token) -> Option<Cow<str>> {
    t.lemma().map(Cow::Borrowed)
}

fn pos(t: &Token) -> Option<Cow<str>> {
    t.pos().map(Cow::Borrowed)
}
