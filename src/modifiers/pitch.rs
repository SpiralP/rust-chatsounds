use super::{args_float1, Modifier};
use nom::{branch::alt, bytes::complete::tag, IResult};
use rodio::Source;

pub struct PitchModifier {
  pub pitch: f32,
}

impl Modifier for PitchModifier {
  fn parse(input: &str) -> IResult<&str, Self> {
    // input = "pitch(2)"

    let (input, _) = alt((tag("pitch"), tag("speed")))(input)?;
    let (input, pitch) = args_float1(input)?;

    Ok((input, PitchModifier { pitch }))
  }

  fn modify(
    &self,
    source: Box<dyn Source<Item = i16> + Send>,
  ) -> Box<dyn Source<Item = i16> + Send> {
    Box::new(source.speed(self.pitch))
  }
}
