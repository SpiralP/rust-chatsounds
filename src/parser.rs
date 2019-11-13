#![allow(dead_code)]
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  character::{is_alphanumeric, is_space},
  multi::{many0, many1},
  number::complete::float,
  IResult,
};

#[derive(Debug)]
pub enum Modifier {
  /// 1.0 default
  Pitch(f32),
  Volume(f32),

  /// duration in secs, amplitude
  Echo(f32, f32),
}

#[derive(Debug)]
pub struct ParsedChatsound {
  pub sentence: String,
  pub modifiers: Vec<Modifier>,
}

fn args_float1(input: &str) -> IResult<&str, f32> {
  let (input, _) = tag("(")(input)?;
  let (input, number) = float(input)?;
  let (input, _) = tag(")")(input)?;

  Ok((input, number))
}

fn args_float2(input: &str) -> IResult<&str, (f32, f32)> {
  let (input, _) = tag("(")(input)?;
  let (input, number1) = float(input)?;
  let (input, _) = tag(",")(input)?;
  let (input, number2) = float(input)?;
  let (input, _) = tag(")")(input)?;

  Ok((input, (number1, number2)))
}

fn pitch(input: &str) -> IResult<&str, Modifier> {
  // input = "pitch(2)"

  let (input, _) = alt((tag("pitch"), tag("speed")))(input)?;
  let (input, number) = args_float1(input)?;

  Ok((input, Modifier::Pitch(number)))
}

fn volume(input: &str) -> IResult<&str, Modifier> {
  // input = "volume(2)"

  let (input, _) = alt((tag("volume"), tag("amplify")))(input)?;
  let (input, number) = args_float1(input)?;

  Ok((input, Modifier::Volume(number)))
}

fn echo(input: &str) -> IResult<&str, Modifier> {
  // input = "echo(2)"

  let (input, _) = alt((tag("echo"), tag("reverb")))(input)?;
  let (input, (duration, amplitude)) = args_float2(input)?;

  Ok((input, Modifier::Echo(duration, amplitude)))
}

fn modifier(input: &str) -> IResult<&str, Modifier> {
  // input = ":pitch(2)"

  let (input, _) = tag(":")(input)?;
  // input = "pitch(2)"

  let (input, modifier) = alt((pitch, volume, echo))(input)?;

  Ok((input, modifier))
}

fn chatsound(input: &str) -> IResult<&str, ParsedChatsound> {
  // input = "hello:pitch(2)"

  let input = input.trim();
  let (input, sentence) = take_while1(|c| is_alphanumeric(c as u8) || is_space(c as u8))(input)?;

  // input = ":pitch(2)"
  // sentence = "hello"

  let (input, modifiers) = many0(modifier)(input)?;

  let chatsound = ParsedChatsound {
    sentence: sentence.to_string(),
    modifiers,
  };

  Ok((input, chatsound))
}

pub fn parse(input: &str) -> Result<Vec<ParsedChatsound>, String> {
  many1(chatsound)(input)
    .map_err(|e| format!("{:?}", e))
    .map(|(_input, chatsounds)| chatsounds)
}

#[test]
fn test_parser() {
  println!("got: {:#?}", parse("hello:pitch(2)").unwrap());
  println!("got: {:#?}", parse("hello:volume(4):pitch(2)").unwrap());
  println!("got: {:#?}", parse("hello:pitch(2) more stuff").unwrap());
  println!("got: {:#?}", parse("hello").unwrap());
  println!(
    "got: {:#?}",
    parse("hello world:pitch(123) another one:volume(22):pitch(33) ").unwrap()
  );

  println!(
    "partial success: {:#?}",
    parse("helloh:pitch(2) bad:pitch(bad)")
  );
  println!("partial success: {:#?}", parse("bad pitch:pitch(asdf)"));

  println!("error: {:#?}", parse(""));
  println!("error: {:#?}", parse("😂"));
}
