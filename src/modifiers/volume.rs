use super::{args_float1, Modifier};
use nom::{branch::alt, bytes::complete::tag, IResult};
use rodio::Source;

pub struct VolumeModifier {
  pub volume: f32,
}

impl Modifier for VolumeModifier {
  fn parse(input: &str) -> IResult<&str, Self> {
    // input = "volume(2)"

    let (input, _) = alt((tag("volume"), tag("amplify")))(input)?;
    let (input, volume) = args_float1(input)?;

    Ok((input, VolumeModifier { volume }))
  }

  fn modify(
    &self,
    source: Box<dyn Source<Item = i16> + Send>,
  ) -> Box<dyn Source<Item = i16> + Send> {
    Box::new(source.amplify(self.volume))
  }
}
