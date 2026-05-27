mod modifiers;

use std::convert::Into;

use nom::{
    IResult, Parser,
    bytes::complete::take_while1,
    multi::{many0, many1},
};
use rand::{Rng, seq::IndexedRandom};

pub use self::modifiers::ModifierTrait;
use self::modifiers::{Modifier, SelectModifier, parse_modifier};
use crate::{
    Chatsound,
    error::{Error, Result},
};

#[derive(Debug, PartialEq)]
pub struct ParsedChatsound {
    pub sentence: String,
    pub modifiers: Vec<Modifier>,
}

/// Normalize a sentence for `map_store` lookup. Rules:
/// - ASCII alphanumerics are kept verbatim (case preserved).
/// - `'` and `,` are dropped without inserting a space, so contractions
///   (`we've` → `weve`) and thousands separators (`1,000` → `1000`)
///   stay one token.
/// - Any other character (whitespace, `_`, `-`, `.`, `!`, non-ASCII, ...)
///   is treated as a word boundary; runs collapse to a single space and
///   leading/trailing spaces are trimmed.
#[must_use]
pub fn normalize_sentence(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_space = true;
    for c in s.chars() {
        if c == '\'' || c == ',' {
            continue;
        }
        if c.is_ascii_alphanumeric() {
            out.push(c);
            prev_space = false;
        } else if !prev_space {
            out.push(' ');
            prev_space = true;
        }
    }
    if out.ends_with(' ') {
        out.pop();
    }
    out
}

impl ParsedChatsound {
    pub fn choose<'a, R: Rng>(
        &self,
        chatsounds: &'a [Chatsound],
        mut rng: R,
    ) -> Option<&'a Chatsound> {
        let modifier = self
            .modifiers
            .iter()
            .find(|m| matches!(m, Modifier::Select(_)));
        if let Some(Modifier::Select(SelectModifier { select })) = modifier {
            chatsounds.get(*select as usize)
        } else {
            chatsounds.choose(&mut rng)
        }
    }
}

fn parse_chatsound(input: &str) -> IResult<&str, ParsedChatsound> {
    // input = "hello:pitch(2)"

    let input = input.trim();
    let (input, sentence) = take_while1(|c: char| {
        c.is_ascii_alphanumeric() || c.is_ascii_whitespace() || c == '\'' || c == ','
    })(input)?;

    // input = ":pitch(2)"
    // sentence = "hello"

    let (input, modifiers) = many0(parse_modifier).parse(input)?;

    let chatsound = ParsedChatsound {
        sentence: sentence.to_string(),
        modifiers,
    };

    Ok((input, chatsound))
}

pub fn parse(input: &str) -> Result<Vec<ParsedChatsound>> {
    match many1(parse_chatsound).parse(input) {
        Ok((_input, chatsounds)) => Ok(chatsounds),
        Err(err) => Err(Error::Nom {
            err: err.map_input(Into::into),
            text: input.into(),
        }),
    }
}

#[test]
#[expect(clippy::too_many_lines, reason = "table-driven test cases")]
fn test_parser() {
    use self::modifiers::*;

    assert_eq!(
        parse("hello:pitch(2)").unwrap(),
        vec![ParsedChatsound {
            sentence: "hello".to_string(),
            modifiers: vec![Modifier::Pitch(PitchModifier { pitch: 2.0 })],
        }]
    );
    assert_eq!(
        parse("hello:volume(4):pitch(2)").unwrap(),
        vec![ParsedChatsound {
            sentence: "hello".to_string(),
            modifiers: vec![
                Modifier::Volume(VolumeModifier { volume: 4.0 }),
                Modifier::Pitch(PitchModifier { pitch: 2.0 })
            ]
        }]
    );
    assert_eq!(
        parse("hello:pitch(2) more stuff").unwrap(),
        vec![
            ParsedChatsound {
                sentence: "hello".to_string(),
                modifiers: vec![Modifier::Pitch(PitchModifier { pitch: 2.0 })]
            },
            ParsedChatsound {
                sentence: "more stuff".to_string(),
                modifiers: vec![]
            }
        ]
    );
    assert_eq!(
        parse("hello").unwrap(),
        vec![ParsedChatsound {
            sentence: "hello".to_string(),
            modifiers: vec![]
        }]
    );
    assert_eq!(
        parse("hello:select(2)").unwrap(),
        vec![ParsedChatsound {
            sentence: "hello".to_string(),
            modifiers: vec![Modifier::Select(SelectModifier { select: 1 })]
        }]
    );

    assert_eq!(
        parse("hello world:pitch(123) another one:volume(22):pitch(33) ").unwrap(),
        vec![
            ParsedChatsound {
                sentence: "hello world".to_string(),
                modifiers: vec![Modifier::Pitch(PitchModifier { pitch: 123.0 })]
            },
            ParsedChatsound {
                sentence: "another one".to_string(),
                modifiers: vec![
                    Modifier::Volume(VolumeModifier { volume: 22.0 }),
                    Modifier::Pitch(PitchModifier { pitch: 33.0 })
                ]
            }
        ]
    );

    assert_eq!(
        parse("they're on").unwrap(),
        vec![ParsedChatsound {
            sentence: "they're on".to_string(),
            modifiers: vec![]
        }]
    );
    assert_eq!(
        parse("they're on:pitch(2)").unwrap(),
        vec![ParsedChatsound {
            sentence: "they're on".to_string(),
            modifiers: vec![Modifier::Pitch(PitchModifier { pitch: 2.0 })]
        }]
    );
    // `,` is allowed inside the sentence span so user input with thousands
    // separators / Oxford-comma phrasing round-trips through normalization.
    assert_eq!(
        parse("hello,world").unwrap(),
        vec![ParsedChatsound {
            sentence: "hello,world".to_string(),
            modifiers: vec![]
        }]
    );
    assert_eq!(
        parse("hello, world").unwrap(),
        vec![ParsedChatsound {
            sentence: "hello, world".to_string(),
            modifiers: vec![]
        }]
    );
    assert_eq!(
        parse("1,000:pitch(2)").unwrap(),
        vec![ParsedChatsound {
            sentence: "1,000".to_string(),
            modifiers: vec![Modifier::Pitch(PitchModifier { pitch: 2.0 })]
        }]
    );
    assert_eq!(
        parse("we've got 1,000:volume(4)").unwrap(),
        vec![ParsedChatsound {
            sentence: "we've got 1,000".to_string(),
            modifiers: vec![Modifier::Volume(VolumeModifier { volume: 4.0 })]
        }]
    );
    // Commas inside modifier args (e.g. `echo(duration, amplitude)`) still
    // parse as arg separators — the sentence span ends at `:`, so allowing
    // `,` in the sentence only affects what comes before the first `:`.
    assert_eq!(
        parse("1,000:pitch(1.5):echo(0.5,0.2)").unwrap(),
        vec![ParsedChatsound {
            sentence: "1,000".to_string(),
            modifiers: vec![
                Modifier::Pitch(PitchModifier { pitch: 1.5 }),
                Modifier::Echo(EchoModifier {
                    duration: 0.5,
                    amplitude_diff: -0.2,
                    amount: 1,
                }),
            ]
        }]
    );
    // `-` is not in the allowed set; the sentence ends at the dash.
    assert_eq!(
        parse("hello-world").unwrap(),
        vec![ParsedChatsound {
            sentence: "hello".to_string(),
            modifiers: vec![]
        }]
    );

    println!(
        "partial success: {:#?}",
        parse("helloh:pitch(2) bad:pitch(bad)")
    );
    println!("partial success: {:#?}", parse("bad pitch:pitch(asdf)"));

    println!("error: {:#?}", parse(""));
    println!("error: {:#?}", parse("😂"));
}

#[test]
fn test_normalize_sentence() {
    // Apostrophes drop without inserting a space, so contractions stay one word.
    assert_eq!(normalize_sentence("they're on"), "theyre on");
    assert_eq!(normalize_sentence("don't"), "dont");
    assert_eq!(normalize_sentence("we're're"), "werere");
    assert_eq!(normalize_sentence("'leading"), "leading");
    assert_eq!(normalize_sentence("trailing'"), "trailing");
    assert_eq!(normalize_sentence("'''"), "");

    // Commas drop without inserting a space, so thousands separators stay one number.
    assert_eq!(normalize_sentence("1,000"), "1000");
    assert_eq!(normalize_sentence("a,b,c"), "abc");
    assert_eq!(normalize_sentence("hello,world"), "helloworld");
    assert_eq!(normalize_sentence(",,,"), "");
    assert_eq!(normalize_sentence(",leading"), "leading");
    assert_eq!(normalize_sentence("trailing,"), "trailing");
    assert_eq!(normalize_sentence("5,4,3,2,1"), "54321");
    // Comma is dropped, then the space is the only boundary between words.
    assert_eq!(normalize_sentence("hello, world"), "hello world");
    // `,` drops; `.` and `$` become spaces and collapse with the spaces around them.
    assert_eq!(normalize_sentence("$1,000.00"), "1000 00");
    // Apostrophe + comma combined.
    assert_eq!(normalize_sentence("we've got 1,000"), "weve got 1000");

    // Any other non-alnum char becomes a space; runs collapse; ends trim.
    assert_eq!(normalize_sentence("dad-please"), "dad please");
    assert_eq!(normalize_sentence("Hello,   World!"), "Hello World");
    assert_eq!(normalize_sentence("0-a"), "0 a");
    assert_eq!(normalize_sentence(" file test  "), "file test");
    assert_eq!(normalize_sentence("!file_test!"), "file test");
    assert_eq!(normalize_sentence("a_-_b"), "a b");
    assert_eq!(normalize_sentence("a---b"), "a b");
    assert_eq!(normalize_sentence("a.b.c"), "a b c");
    assert_eq!(normalize_sentence("___"), "");
    assert_eq!(normalize_sentence("@@@"), "");
    assert_eq!(normalize_sentence("\thello\tworld\n"), "hello world");

    // Case preserved.
    assert_eq!(
        normalize_sentence("yes no yes no YES NO"),
        "yes no yes no YES NO"
    );
    assert_eq!(
        normalize_sentence("hell yeah now we've got business"),
        "hell yeah now weve got business"
    );
    assert_eq!(normalize_sentence("Mixed CASE 123"), "Mixed CASE 123");

    // Empty / whitespace-only.
    assert_eq!(normalize_sentence(""), "");
    assert_eq!(normalize_sentence("   "), "");
    assert_eq!(normalize_sentence("  spaced  "), "spaced");

    // Non-ASCII is treated as non-alnum (becomes space, then trims).
    assert_eq!(normalize_sentence("café"), "caf");
}
