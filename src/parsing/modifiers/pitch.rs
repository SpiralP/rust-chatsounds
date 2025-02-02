use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use rodio::{buffer::SamplesBuffer, Source};

use super::{parse_args, ModifierTrait};
use crate::BoxSource;

#[derive(Debug, PartialEq)]
pub struct PitchModifier {
    pub pitch: f32,
}

impl Default for PitchModifier {
    fn default() -> Self {
        Self { pitch: 1.0 }
    }
}

impl ModifierTrait for PitchModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "pitch(2)"

        let (input, _) = alt((tag("pitch"), tag("speed"))).parse(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = PitchModifier::default();

        if let Some(pitch) = args.first().copied().unwrap_or(None) {
            modifier.pitch = if pitch > 0.0 && pitch < 0.1 {
                0.1
            } else if pitch < 0.0 && pitch > -0.1 {
                -0.1
            } else {
                pitch
            };
        }

        Ok((input, modifier))
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        // panics if 0
        if self.pitch.abs() < 0.0000001 {
            return Box::new(source);
        }

        if self.pitch > 0.0 {
            Box::new(source.speed(self.pitch))
        } else {
            let channels = source.channels();
            let sample_rate = source.sample_rate();
            let mut samples: Vec<_> = source.collect();
            samples.reverse();

            Box::new(SamplesBuffer::new(channels, sample_rate, samples).speed(self.pitch.abs()))
        }
    }
}
