mod echo;
mod pitch;
mod volume;

pub use self::{echo::EchoModifier, pitch::PitchModifier, volume::VolumeModifier};
use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::{map, opt},
  multi::separated_list,
  number::complete::float,
  IResult,
};
use rodio::Source;

pub trait Modifier
where
  Self: Send,
{
  fn parse(input: &str) -> IResult<&str, Self>
  where
    Self: Sized;

  fn modify(
    &self,
    source: Box<dyn Source<Item = i16> + Send>,
  ) -> Box<dyn Source<Item = i16> + Send>;
}

pub fn parse_modifier(input: &str) -> IResult<&str, Box<dyn Modifier>> {
  // input = ":pitch(2)"

  let (input, _) = tag(":")(input)?;
  // input = "pitch(2)"

  let (input, modifier) = alt((
    map(PitchModifier::parse, |a| Box::new(a) as Box<dyn Modifier>),
    map(VolumeModifier::parse, |a| Box::new(a) as Box<dyn Modifier>),
    map(EchoModifier::parse, |a| Box::new(a) as Box<dyn Modifier>),
  ))(input)?;

  Ok((input, modifier))
}

type Args = Vec<Option<f32>>;

fn parse_arg(input: &str) -> IResult<&str, Option<f32>> {
  // " 123 ,"
  //  ^
  let input = input.trim_start();

  let (input, maybe) = opt(float)(input)?;

  // " ,"
  //  ^
  let input = input.trim_start();

  Ok((input, maybe))
}

fn parse_arg_list(s: &str) -> IResult<&str, Args> {
  let sep = tag(",");

  let mut list = Vec::new();

  // fix separated_list not working if you start with a separator
  let mut s = s;
  loop {
    let (s2, maybe_first) = opt(&sep)(s)?;
    s = s2;

    if maybe_first.is_some() {
      list.push(None);
    } else {
      break;
    }
  }

  let (s, mut items) = separated_list(&sep, parse_arg)(s)?;
  list.append(&mut items);

  Ok((s, list))
}

pub fn parse_args(input: &str) -> IResult<&str, Args> {
  let (input, _) = tag("(")(input)?;

  // fix for "( )"
  // " )"
  //  ^
  let input = input.trim_start();

  let (input, args) = opt(parse_arg_list)(input)?;
  let (input, _) = tag(")")(input)?;

  Ok((input, args.unwrap_or_else(|| vec![])))
}

#[test]
fn test_parse_args() {
  assert_eq!(parse_args("()"), Ok(("", vec![])));
  assert_eq!(parse_args("(    )"), Ok(("", vec![])));

  assert_eq!(parse_args("(1)"), Ok(("", vec![Some(1.0)])));
  assert_eq!(parse_args("(1, 3)"), Ok(("", vec![Some(1.0), Some(3.0)])));
  assert_eq!(
    parse_args("(1, 3, 5)"),
    Ok(("", vec![Some(1.0), Some(3.0), Some(5.0)]))
  );

  assert_eq!(
    parse_args("( 1 , 3 , 5 )"),
    Ok(("", vec![Some(1.0), Some(3.0), Some(5.0)]))
  );

  assert_eq!(parse_args("( , 2 )"), Ok(("", vec![None, Some(2.0)])));
  assert_eq!(
    parse_args("( , 2 , )"),
    Ok(("", vec![None, Some(2.0), None]))
  );

  assert_eq!(
    parse_args("(, , 2 , ,)"),
    Ok(("", vec![None, None, Some(2.0), None, None]))
  );
  assert_eq!(
    parse_args("( , 2 , ,)"),
    Ok(("", vec![None, Some(2.0), None, None]))
  );
}
