use super::{parse_args, Modifier};
use nom::{branch::alt, bytes::complete::tag, IResult};
use rodio::Source;

pub struct VolumeModifier {
  pub volume: f32,
}

impl Default for VolumeModifier {
  fn default() -> Self {
    Self { volume: 1.0 }
  }
}

impl Modifier for VolumeModifier {
  fn parse(input: &str) -> IResult<&str, Self> {
    // input = "volume(2)"

    let (input, _) = alt((tag("volume"), tag("amplify")))(input)?;
    let (input, args) = parse_args(input)?;

    let mut modifier = VolumeModifier::default();

    if let Some(volume) = args.get(0).copied().unwrap_or(None) {
      modifier.volume = volume;
    }

    Ok((input, modifier))
  }

  fn modify(
    &self,
    source: Box<dyn Source<Item = i16> + Send>,
  ) -> Box<dyn Source<Item = i16> + Send> {
    Box::new(source.amplify(self.volume))
  }
}
