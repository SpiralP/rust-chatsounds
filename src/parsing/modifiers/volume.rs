use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use rodio::Source;

use super::{parse_args, ModifierTrait};
use crate::BoxSource;

#[derive(Debug, PartialEq)]
pub struct VolumeModifier {
    pub volume: f32,
}

impl Default for VolumeModifier {
    fn default() -> Self {
        Self { volume: 1.0 }
    }
}

impl ModifierTrait for VolumeModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "volume(2)"

        let (input, _) = alt((tag("volume"), tag("amplify"))).parse(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = Self::default();

        if let Some(volume) = args.first().copied().unwrap_or(None) {
            modifier.volume = volume.max(0.0);
        }

        Ok((input, modifier))
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        Box::new(source.amplify(self.volume))
    }
}
