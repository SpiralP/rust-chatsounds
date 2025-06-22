use std::time::Duration;

use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use rodio::Source;

use super::{parse_args, ModifierTrait};
use crate::BoxSource;

#[derive(Debug, PartialEq)]
pub struct CutModifier {
    /// in seconds
    pub duration: f32,
}

impl Default for CutModifier {
    fn default() -> Self {
        Self { duration: 0.5 }
    }
}

impl ModifierTrait for CutModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "cut(2)"

        let (input, _) = alt((tag("cut"), tag("take"))).parse(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = Self::default();

        if let Some(duration) = args.first().copied().unwrap_or(None) {
            modifier.duration = duration.max(0.0);
        }

        Ok((input, modifier))
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        let duration = Duration::from_secs_f32(self.duration);
        Box::new(source.buffered().take_duration(duration))
    }
}
