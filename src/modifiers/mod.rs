mod echo;
mod pitch;
mod volume;

pub use self::{echo::EchoModifier, pitch::PitchModifier, volume::VolumeModifier};
use nom::{bytes::complete::tag, number::complete::float, IResult};
use rodio::Source;

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
