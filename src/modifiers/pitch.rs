use super::{parse_args, Modifier};
use nom::{branch::alt, bytes::complete::tag, IResult};
use rodio::Source;

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
      modifier.pitch = pitch;
    }

    Ok((input, modifier))
  }

  fn modify(
    &self,
    source: Box<dyn Source<Item = i16> + Send>,
  ) -> Box<dyn Source<Item = i16> + Send> {
    Box::new(source.speed(self.pitch))
  }
}
