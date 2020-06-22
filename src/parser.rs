#![allow(dead_code)]

use crate::modifiers::{parse_modifier, Modifier};
use nom::{
    bytes::complete::take_while1,
    character::{is_alphanumeric, is_space},
    multi::{many0, many1},
    IResult,
};

pub struct ParsedChatsound {
    pub sentence: String,
    pub modifiers: Vec<Box<dyn Modifier>>,
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

pub fn parse(input: &str) -> Result<Vec<ParsedChatsound>, String> {
    many1(parse_chatsound)(input)
        .map_err(|e| format!("{:?}", e))
        .map(|(_input, chatsounds)| chatsounds)
}

#[test]
fn test_parser() {
    // assert_eq!(
    //   parse("hello:pitch(2)"),
    //   Ok(vec![ParsedChatsound {
    //     sentence: "hello".to_string(),
    //     modifiers: vec![Box::new(PitchModifier { pitch: 2.0 })]
    //   }])
    // );
    // assert_eq!(
    //   parse("hello:volume(4):pitch(2)"),
    //   Ok(vec![ParsedChatsound {
    //     sentence: "hello".to_string(),
    //     modifiers: vec![
    //       Box::new(VolumeModifier { volume: 4.0 }),
    //       Box::new(PitchModifier { pitch: 2.0 })
    //     ]
    //   }])
    // );
    // assert_eq!(
    //   parse("hello:pitch(2) more stuff"),
    //   Ok(vec![
    //     ParsedChatsound {
    //       sentence: "hello".to_string(),
    //       modifiers: vec![Box::new(PitchModifier { pitch: 2.0 })]
    //     },
    //     ParsedChatsound {
    //       sentence: "more stuff".to_string(),
    //       modifiers: vec![]
    //     }
    //   ])
    // );
    // assert_eq!(
    //   parse("hello"),
    //   Ok(vec![ParsedChatsound {
    //     sentence: "hello".to_string(),
    //     modifiers: vec![]
    //   }])
    // );

    // assert_eq!(
    //   parse("hello world:pitch(123) another one:volume(22):pitch(33) "),
    //   Ok(vec![
    //     ParsedChatsound {
    //       sentence: "hello world".to_string(),
    //       modifiers: vec![Box::new(PitchModifier { pitch: 123.0 })]
    //     },
    //     ParsedChatsound {
    //       sentence: "another one".to_string(),
    //       modifiers: vec![
    //         Box::new(VolumeModifier { volume: 22.0 }),
    //         Box::new(PitchModifier { pitch: 33.0 })
    //       ]
    //     }
    //   ])
    // );

    // println!(
    //   "partial success: {:#?}",
    //   parse("helloh:pitch(2) bad:pitch(bad)")
    // );
    // println!("partial success: {:#?}", parse("bad pitch:pitch(asdf)"));

    // println!("error: {:#?}", parse(""));
    // println!("error: {:#?}", parse("😂"));
}
