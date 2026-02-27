use std::{
    f32,
    sync::{Arc, Mutex},
    time::Duration,
};

use rodio::{cpal::FromSample, mixer::Mixer, source::ChannelVolume, PlayError, Player, Source};

pub struct ChannelVolumeSink {
    pub sink: Player,
    channel_volumes: Arc<Mutex<Vec<f32>>>,
}

impl ChannelVolumeSink {
    pub fn connect_new(mixer: &Mixer, channel_volumes: Vec<f32>) -> Result<Self, PlayError> {
        Ok(Self {
            sink: Player::connect_new(mixer),
            channel_volumes: Arc::new(Mutex::new(channel_volumes)),
        })
    }

    pub fn set_channel_volumes(&self, channel_volumes: Vec<f32>) {
        *self.channel_volumes.lock().unwrap() = channel_volumes;
    }

    pub fn append<S>(&self, source: S)
    where
        S: Source + Send + 'static,
        f32: FromSample<S::Item>,
    {
        let channel_volumes = self.channel_volumes.clone();
        let source = ChannelVolume::new(source, self.channel_volumes.lock().unwrap().clone())
            .periodic_access(Duration::from_millis(10), move |i| {
                let channel_volumes = channel_volumes.lock().unwrap();
                for (channel, volume) in channel_volumes.iter().enumerate() {
                    i.set_volume(channel, *volume);
                }
            });
        self.sink.append(source);
    }
}
