use conllx::{Features, Token};
use std::collections::HashMap;

pub type LayerCallback = fn(&Token) -> Option<&str>;

lazy_static! {
    pub static ref LAYER_CALLBACKS: HashMap<&'static str, LayerCallback> =
        {
            let mut m: HashMap<&'static str, LayerCallback> = HashMap::new();
            m.insert("cpos", cpos);
            m.insert("features", features);
            m.insert("form", form);
            m.insert("headrel", head_rel);
            m.insert("pheadrel", p_head_rel);
            m.insert("lemma", lemma);
            m.insert("pos", pos);
            m
        };
}

fn cpos(t: &Token) -> Option<&str> {
    t.cpos()
}

fn features(t: &Token) -> Option<&str> {
    t.features().map(Features::as_str)
}

fn form(t: &Token) -> Option<&str> {
    t.form()
}

fn head_rel(t: &Token) -> Option<&str> {
    t.head_rel()
}

fn p_head_rel(t: &Token) -> Option<&str> {
    t.p_head_rel()
}

fn lemma(t: &Token) -> Option<&str> {
    t.lemma()
}

fn pos(t: &Token) -> Option<&str> {
    t.pos()
}
