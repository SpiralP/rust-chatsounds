use super::{parse_args, Modifier};
use crate::BoxSource;
use nom::{branch::alt, bytes::complete::tag, IResult};
use rodio::{buffer::SamplesBuffer, Source};

pub struct PitchModifier {
    pub pitch: f32,
}

impl Default for PitchModifier {
    fn default() -> Self {
        Self { pitch: 1.0 }
    }
}

impl Modifier for PitchModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "pitch(2)"

        let (input, _) = alt((tag("pitch"), tag("speed")))(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = PitchModifier::default();

        if let Some(pitch) = args.get(0).copied().unwrap_or(None) {
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
        if self.pitch > 0.0 {
            Box::new(source.speed(self.pitch))
        } else {
            let channels = source.channels();
            let sample_rate = source.sample_rate();
            let mut samples: Vec<_> = source.collect();
            samples.reverse();

            Box::new(SamplesBuffer::new(channels, sample_rate, samples))
        }
    }
}
