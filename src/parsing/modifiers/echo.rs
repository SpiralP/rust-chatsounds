use std::time::Duration;

use nom::{branch::alt, bytes::complete::tag, IResult, Parser};
use rodio::{dynamic_mixer::mixer, Source};

use super::{parse_args, ModifierTrait};
use crate::BoxSource;

#[derive(Debug, PartialEq)]
pub struct EchoModifier {
    /// in seconds
    pub duration: f32,
    pub amplitude_diff: f32,
    pub amount: u8,
}

impl Default for EchoModifier {
    fn default() -> Self {
        Self {
            duration: 0.25,
            amplitude_diff: -0.5,
            amount: 1,
        }
    }
}

impl ModifierTrait for EchoModifier {
    fn parse(input: &str) -> IResult<&str, Self> {
        // input = "echo(2)"

        let (input, _) = alt((tag("echo"), tag("reverb"))).parse(input)?;
        let (input, args) = parse_args(input)?;

        let mut modifier = Self::default();

        if let Some(duration) = args.first().copied().unwrap_or(None) {
            modifier.duration = duration.max(0.0);
        }

        if let Some(amplitude) = args.get(1).copied().unwrap_or(None) {
            modifier.amplitude_diff = -1.0 * amplitude;
        }

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        if let Some(amount) = args.get(2).copied().unwrap_or(None) {
            modifier.amount = amount.max(1.0).floor() as u8;
        }

        Ok((input, modifier))
    }

    fn modify(&self, source: BoxSource) -> BoxSource {
        let mut amplitude = 1.0 + self.amplitude_diff;
        let mut duration = Duration::from_secs_f32(self.duration);

        let (controller, mixer) = mixer(source.channels(), source.sample_rate());

        let a = source.buffered();
        controller.add(a.clone());

        for _i in 0..self.amount {
            let echo = a.clone().amplify(amplitude).delay(duration);
            controller.add(echo);
            amplitude = (amplitude + self.amplitude_diff).max(0.0);
            duration += Duration::from_secs_f32(self.duration);
        }

        Box::new(mixer)
    }
}
