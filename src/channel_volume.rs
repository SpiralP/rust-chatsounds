use std::{
    f32,
    sync::{Arc, Mutex},
    time::Duration,
};

use rodio::{
    cpal::FromSample, source::ChannelVolume, OutputStreamHandle, PlayError, Sample, Sink, Source,
};

pub struct ChannelVolumeSink {
    pub sink: Sink,
    channel_volumes: Arc<Mutex<Vec<f32>>>,
}

impl ChannelVolumeSink {
    pub fn try_new(
        stream: &OutputStreamHandle,
        channel_volumes: Vec<f32>,
    ) -> Result<ChannelVolumeSink, PlayError> {
        Ok(ChannelVolumeSink {
            sink: Sink::try_new(stream)?,
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
        S::Item: Sample + Send,
    {
        let channel_volumes = self.channel_volumes.clone();
        let channel_volumes_lock = self.channel_volumes.lock().unwrap();
        let source = ChannelVolume::new(source, channel_volumes_lock.clone()).periodic_access(
            Duration::from_millis(10),
            move |i| {
                let channel_volumes = channel_volumes.lock().unwrap();
                for (channel, volume) in channel_volumes.iter().enumerate() {
                    i.set_volume(channel, *volume);
                }
            },
        );
        self.sink.append(source);
    }
}
