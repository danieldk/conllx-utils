use unicode_normalization::UnicodeNormalization;

/// Types of unicode normalization.
#[derive(Copy, Clone)]
pub enum Normalization {
    None,
    NFD,
    NFKD,
    NFC,
    NFKC,
}

fn normalization_iter<'a, I>(iter: I, norm: Normalization) -> Box<Iterator<Item = char> + 'a>
where
    I: 'a + Iterator<Item = char>,
{
    use self::Normalization::*;

    match norm {
        None => Box::new(iter),
        NFD => Box::new(iter.nfd()),
        NFKD => Box::new(iter.nfkd()),
        NFC => Box::new(iter.nfc()),
        NFKC => Box::new(iter.nfkc()),
    }
}

pub enum Conversion {
    Char(char),
    String(String),
    None(char),
}

// Source of Unicode -> ASCII mappings:
// http://lexsrv3.nlm.nih.gov/LexSysGroup/Projects/lvg/current/docs/designDoc/UDF/unicode/DefaultTables/symbolTable.html
pub fn simplify_unicode_punct(c: char) -> Conversion {
    match c {
        '«' => Conversion::Char('"'),
        '´' => Conversion::Char('\''),
        '»' => Conversion::Char('"'),
        '÷' => Conversion::Char('/'),
        'ǀ' => Conversion::Char('|'),
        'ǃ' => Conversion::Char('!'),
        'ʹ' => Conversion::Char('\''),
        'ʺ' => Conversion::Char('"'),
        'ʼ' => Conversion::Char('\''),
        '˄' => Conversion::Char('^'),
        'ˆ' => Conversion::Char('^'),
        'ˈ' => Conversion::Char('\''),
        'ˋ' => Conversion::Char('`'),
        'ˍ' => Conversion::Char('_'),
        '˜' => Conversion::Char('~'),
        '։' => Conversion::Char(':'),
        '׀' => Conversion::Char('|'),
        '׃' => Conversion::Char(':'),
        '٪' => Conversion::Char('%'),
        '٭' => Conversion::Char('*'),
        '‐' => Conversion::Char('-'),
        '‑' => Conversion::Char('-'),
        '‒' => Conversion::Char('-'),
        '–' => Conversion::Char('-'),
        '—' => Conversion::Char('-'),
        '―' => Conversion::Char('-'),
        '‗' => Conversion::Char('_'),
        '‘' => Conversion::Char('\''),
        '’' => Conversion::Char('\''),
        '‚' => Conversion::Char(','),
        '‛' => Conversion::Char('\''),
        '“' => Conversion::Char('"'),
        '”' => Conversion::Char('"'),
        '„' => Conversion::Char('"'),
        '‟' => Conversion::Char('"'),
        '′' => Conversion::Char('\''),
        '″' => Conversion::Char('"'),
        '‵' => Conversion::Char('`'),
        '‶' => Conversion::Char('"'),
        '‸' => Conversion::Char('^'),
        '‹' => Conversion::Char('<'),
        '›' => Conversion::Char('>'),
        '‽' => Conversion::Char('?'),
        '⁄' => Conversion::Char('/'),
        '⁎' => Conversion::Char('*'),
        '⁒' => Conversion::Char('%'),
        '⁓' => Conversion::Char('~'),
        '−' => Conversion::Char('-'),
        '∕' => Conversion::Char('/'),
        '∖' => Conversion::Char('\\'),
        '∗' => Conversion::Char('*'),
        '∣' => Conversion::Char('|'),
        '∶' => Conversion::Char(':'),
        '∼' => Conversion::Char('~'),
        '⌃' => Conversion::Char('^'),
        '♯' => Conversion::Char('#'),
        '✱' => Conversion::Char('*'),
        '❘' => Conversion::Char('|'),
        '❢' => Conversion::Char('!'),
        '⟦' => Conversion::Char('['),
        '⟨' => Conversion::Char('<'),
        '⟩' => Conversion::Char('>'),
        '⦃' => Conversion::Char('{'),
        '⦄' => Conversion::Char('}'),
        '〃' => Conversion::Char('"'),
        '〈' => Conversion::Char('<'),
        '〉' => Conversion::Char('>'),
        '〛' => Conversion::Char(']'),
        '〜' => Conversion::Char('~'),
        '〝' => Conversion::Char('"'),
        '〞' => Conversion::Char('"'),
        '‖' => Conversion::String("||".to_string()),
        '‴' => Conversion::String("'''".to_string()),
        '‷' => Conversion::String("'''".to_string()),
        '≤' => Conversion::String("<=".to_string()),
        '≥' => Conversion::String(">=".to_string()),
        '≦' => Conversion::String("<=".to_string()),
        '≧' => Conversion::String(">=".to_string()),
        '…' => Conversion::String("...".to_string()),
        _ => Conversion::None(c),
    }
}

pub fn simplify_unicode(s: &str, norm: Normalization) -> String {
    normalization_iter(s.chars(), norm).fold(String::with_capacity(s.len()), |mut s, c| {
        match simplify_unicode_punct(c) {
            Conversion::Char(c) => s.push(c),
            Conversion::String(ss) => s.push_str(&ss),
            Conversion::None(c) => s.push(c),
        }

        s
    })
}
