use super::{parse_args, ModifierTrait};
use crate::BoxSource;
use nom::{branch::alt, bytes::complete::tag, IResult};
use rodio::Source;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub struct EchoModifier {
    /// in seconds
    pub duration: f32,
    pub amplitude: f32,
}

impl Default for EchoModifier {
    fn default() -> Self {
        Self {
            duration: 0.25,
            amplitude: 0.25,
        }
    }
}

impl ModifierTrait for EchoModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "echo(2)"

        let (input, _) = alt((tag("echo"), tag("reverb")))(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = EchoModifier::default();

        if let Some(duration) = args.get(0).copied().unwrap_or(None) {
            modifier.duration = duration.max(0.0);
        }

        if let Some(amplitude) = args.get(1).copied().unwrap_or(None) {
            modifier.amplitude = amplitude.max(0.0);
        }

        Ok((input, modifier))
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        let duration = Duration::from_secs_f32(self.duration);
        Box::new(source.buffered().reverb(duration, self.amplitude))
    }
}
