mod echo;
mod pitch;
mod volume;

pub use self::{echo::EchoModifier, pitch::PitchModifier, volume::VolumeModifier};
use crate::BoxSource;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, opt},
    multi::separated_list0,
    number::complete::float,
    IResult,
};

pub trait ModifierTrait: Send + std::fmt::Debug + PartialEq {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;

    fn modify(&self, source: BoxSource) -> BoxSource;
}

#[derive(Debug, PartialEq)]
pub enum Modifier {
    Pitch(PitchModifier),
    Volume(VolumeModifier),
    Echo(EchoModifier),
}

impl ModifierTrait for Modifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(PitchModifier::parse, Modifier::Pitch),
            map(VolumeModifier::parse, Modifier::Volume),
            map(EchoModifier::parse, Modifier::Echo),
        ))(input)
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        match self {
            Modifier::Pitch(modifier) => modifier.modify(source),
            Modifier::Volume(modifier) => modifier.modify(source),
            Modifier::Echo(modifier) => modifier.modify(source),
        }
    }
}

pub fn parse_modifier(input: &str) -> IResult<&str, Modifier> {
    // input = ":pitch(2)"

    let (input, _) = tag(":")(input)?;
    // input = "pitch(2)"

    let (input, modifier) = Modifier::parse(input)?;

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

    // fix separated_list0 not working if you start with a separator
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

    let (s, mut items) = separated_list0(&sep, parse_arg)(s)?;
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

    Ok((input, args.unwrap_or_default()))
}

#[test]
fn test_parse_args() {
    assert_eq!(parse_args("()"), Ok(("", vec![None])));
    assert_eq!(parse_args("(    )"), Ok(("", vec![None])));

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

    assert_eq!(parse_args("( , )"), Ok(("", vec![None, None])));

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

    assert_eq!(
        parse_args("( , -2.1 , ,)"),
        Ok(("", vec![None, Some(-2.1), None, None]))
    );
}
