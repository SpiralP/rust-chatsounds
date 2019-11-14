use nom::{branch::alt, bytes::complete::tag, number::complete::float, IResult};
use rodio::Source;
use std::time::Duration;

pub trait Modifier {
  fn parse(input: &str) -> IResult<&str, Self>
  where
    Self: Sized;

  fn modify(
    &self,
    source: Box<dyn Source<Item = i16> + Send>,
  ) -> Box<dyn Source<Item = i16> + Send>;
}

fn args_float1(input: &str) -> IResult<&str, f32> {
  let (input, _) = tag("(")(input)?;
  let (input, number) = float(input)?;
  let (input, _) = tag(")")(input)?;

  Ok((input, number))
}

fn args_float2(input: &str) -> IResult<&str, (f32, f32)> {
  let (input, _) = tag("(")(input)?;
  let (input, number1) = float(input)?;
  let (input, _) = tag(",")(input)?;
  let (input, number2) = float(input)?;
  let (input, _) = tag(")")(input)?;

  Ok((input, (number1, number2)))
}

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
