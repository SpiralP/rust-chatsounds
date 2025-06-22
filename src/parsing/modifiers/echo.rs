use std::time::Duration;

use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use rodio::Source;

use super::{parse_args, ModifierTrait};
use crate::BoxSource;

#[derive(Debug, PartialEq)]
pub struct EchoModifier {
    /// in seconds
    pub duration: f32,
    pub amplitude_diff: f32,
}

impl Default for EchoModifier {
    fn default() -> Self {
        Self {
            duration: 0.25,
            amplitude_diff: -0.5,
        }
    }
}

impl ModifierTrait for EchoModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "echo(2)"

        let (input, _) = alt((tag("echo"), tag("reverb"))).parse(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = Self::default();

        if let Some(duration) = args.first().copied().unwrap_or(None) {
            modifier.duration = duration.max(0.0);
        }

        if let Some(amplitude) = args.get(1).copied().unwrap_or(None) {
            modifier.amplitude_diff = -1.0 * amplitude;
        }

        Ok((input, modifier))
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        let duration = Duration::from_secs_f32(self.duration);
        Box::new(
            source
                .buffered()
                .reverb(duration, 1.0 + self.amplitude_diff),
        )
    }
}
