use std::collections::HashMap;
use std::fmt;

use conllx::{Features, Token};
use lazy_static::lazy_static;
use maplit::hashmap;

#[derive(Debug, Hash, Eq, PartialEq)]
enum MorphAttribute {
    Case,
    Gender,
    Number,
    Mood,
    Person,
    Tense,
}

impl fmt::Display for MorphAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MorphAttribute::Case => f.write_str("case"),
            MorphAttribute::Gender => f.write_str("gender"),
            MorphAttribute::Number => f.write_str("number"),
            MorphAttribute::Mood => f.write_str("mood"),
            MorphAttribute::Person => f.write_str("person"),
            MorphAttribute::Tense => f.write_str("tense"),
        }
    }
}

const UNDERSPECIFIED_SHORT: char = '*';

const UNDERSPECIFIED_LONG: &str = "underspecified";

lazy_static! {
    static ref TAG_ATTRIBUTES: HashMap<&'static str, Vec<MorphAttribute>> = hashmap! {
        "ADJA" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "APPR" => vec![MorphAttribute::Case],
        "APPRART" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "APPO" => vec![MorphAttribute::Case],
        "ART" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "NN" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "NE" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PDS" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PDAT" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PIS" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PIAT" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PIDAT" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PPER" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender,
            MorphAttribute::Person],
        "PPOSS" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PPOSAT" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PRELS" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PRELAT" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PRF" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender,
            MorphAttribute::Person],
        "PWS" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "PWAT" => vec![MorphAttribute::Case, MorphAttribute::Number, MorphAttribute::Gender],
        "VAFIN" => vec![MorphAttribute::Person, MorphAttribute::Number, MorphAttribute::Mood,
            MorphAttribute::Tense],
        "VAIMP" => vec![MorphAttribute::Number],
        "VMFIN" => vec![MorphAttribute::Person, MorphAttribute::Number, MorphAttribute::Mood,
            MorphAttribute::Tense],
        "VVFIN" => vec![MorphAttribute::Person, MorphAttribute::Number, MorphAttribute::Mood,
            MorphAttribute::Tense],
        "VVIMP" => vec![MorphAttribute::Number],
    };
    static ref MORPH_LONG_NAMES: HashMap<MorphAttribute, HashMap<char, &'static str>> = hashmap! {
        MorphAttribute::Case => hashmap!{
            'n' => "nominative",
            'g' => "genitive",
            'd' => "dative",
            'a' => "accusative",
            UNDERSPECIFIED_SHORT => UNDERSPECIFIED_LONG
        },
        MorphAttribute::Gender => hashmap!{
            'm' => "masculine",
            'f' => "feminine",
            'n' => "neuter",
            UNDERSPECIFIED_SHORT => UNDERSPECIFIED_LONG
        },
        MorphAttribute::Number => hashmap!{
            's' => "singular",
            'p' => "plural",
            UNDERSPECIFIED_SHORT => UNDERSPECIFIED_LONG
        },
        MorphAttribute::Mood => hashmap!{
            'i' => "indicative",
            'k' => "subjunctive",
        },
        MorphAttribute::Person => hashmap!{
            '1' => "1",
            '2' => "2",
            '3' => "3",
            UNDERSPECIFIED_SHORT => UNDERSPECIFIED_LONG
        },
        MorphAttribute::Tense => hashmap!{
            's' => "present",
            't' => "past",
            UNDERSPECIFIED_SHORT => UNDERSPECIFIED_LONG
        }
    };
}

pub enum MorphError {
    IncorrectLength,
    IncorrectChar(usize, char),
}

impl fmt::Display for MorphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MorphError::IncorrectLength => f.write_str("Incorrect morphology features length"),
            MorphError::IncorrectChar(p, c) => {
                write!(f, "Incorrect character '{}' at position {}", c, p)
            }
        }
    }
}

pub fn expand_tdz_morph(token: &mut Token, preserve_orig: bool) -> Result<(), MorphError> {
    let expanded_features = expand_features(token, preserve_orig)?;
    if expanded_features.is_some() {
        token.set_features(expanded_features);
    };

    Ok(())
}

fn expand_features(token: &Token, preserve_orig: bool) -> Result<Option<Features>, MorphError> {
    let tag: &str = match token.pos() {
        Some(tag) => tag,
        None => return Ok(None),
    };

    let morph = match token.features().as_ref() {
        Some(features) => features.as_str(),
        None => return Ok(None),
    };

    if morph == "--" {
        return Ok(None);
    }

    let attributes = match TAG_ATTRIBUTES.get(tag) {
        Some(attributes) => attributes,
        None => return Ok(None),
    };

    if morph.len() != attributes.len() {
        return Err(MorphError::IncorrectLength);
    }

    let mut features = Vec::new();

    for (i, c) in morph.chars().enumerate() {
        // Note: this can only fail if our static tables are wrong!
        //       so, it's safe to unwrap here.
        let short_long = MORPH_LONG_NAMES.get(&attributes[i]).unwrap();

        if let Some(long) = short_long.get(&c) {
            features.push(format!("{}:{}", &attributes[i], long))
        } else {
            return Err(MorphError::IncorrectChar(i, c));
        }
    }

    // Add feature with the original morphological tag.
    if preserve_orig {
        features.push(format!("morph:{}", morph));
    }

    Ok(Some(Features::from_string(features.join("|"))))
}
