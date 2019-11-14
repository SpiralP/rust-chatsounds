use super::{args_float2, Modifier};
use nom::{branch::alt, bytes::complete::tag, IResult};
use rodio::Source;
use std::time::Duration;

pub struct EchoModifier {
  /// in seconds
  pub duration: f32,
  pub amplitude: f32,
}

impl Modifier for EchoModifier {
  fn parse(input: &str) -> IResult<&str, Self> {
    // input = "echo(2)"

    let (input, _) = alt((tag("echo"), tag("reverb")))(input)?;
    let (input, (duration, amplitude)) = args_float2(input)?;

    Ok((
      input,
      EchoModifier {
        duration,
        amplitude,
      },
    ))
  }

  fn modify(
    &self,
    source: Box<dyn Source<Item = i16> + Send>,
  ) -> Box<dyn Source<Item = i16> + Send> {
    let duration = Duration::from_secs_f32(self.duration);
    Box::new(source.buffered().reverb(duration, self.amplitude))
  }
}
