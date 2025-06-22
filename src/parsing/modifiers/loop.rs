use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use rodio::Source;

use super::{parse_args, ModifierTrait};
use crate::BoxSource;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LoopModifier {}

impl ModifierTrait for LoopModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "loop()"

        let (input, _) = alt((tag("loop"), tag("repeat"))).parse(input)?;
        let (input, _args) = parse_args(input)?;

        let modifier = Self::default();

        Ok((input, modifier))
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        Box::new(source.buffered().repeat_infinite())
    }
}
