mod modifiers;

use nom::{
    bytes::complete::take_while1,
    character::{is_alphanumeric, is_space},
    multi::{many0, many1},
    IResult,
};
use rand::{seq::SliceRandom, RngCore};

pub use self::modifiers::ModifierTrait;
use self::modifiers::{parse_modifier, Modifier, SelectModifier};
use crate::{
    error::{Error, Result},
    Chatsound,
};

#[derive(Debug, PartialEq)]
pub struct ParsedChatsound {
    pub sentence: String,
    pub modifiers: Vec<Modifier>,
}

impl ParsedChatsound {
    pub fn choose<'a, R: RngCore>(
        &self,
        chatsounds: &'a Vec<Chatsound>,
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
    let (input, sentence) = take_while1(|c| is_alphanumeric(c as u8) || is_space(c as u8))(input)?;

    // input = ":pitch(2)"
    // sentence = "hello"

    let (input, modifiers) = many0(parse_modifier)(input)?;

    let chatsound = ParsedChatsound {
        sentence: sentence.to_string(),
        modifiers,
    };

    Ok((input, chatsound))
}

pub fn parse(input: &str) -> Result<Vec<ParsedChatsound>> {
    match many1(parse_chatsound)(input) {
        Ok((_input, chatsounds)) => Ok(chatsounds),
        Err(err) => Err(Error::Nom {
            err: err.map_input(|s| s.into()),
            text: input.into(),
        }),
    }
}

#[test]
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

    println!(
        "partial success: {:#?}",
        parse("helloh:pitch(2) bad:pitch(bad)")
    );
    println!("partial success: {:#?}", parse("bad pitch:pitch(asdf)"));

    println!("error: {:#?}", parse(""));
    println!("error: {:#?}", parse("😂"));
}
