use std::{
    io::{BufReader, Cursor},
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use chatsounds::*;
#[cfg(all(test, not(feature = "wasm"), feature = "fs"))]
use futures::stream::{StreamExt, TryStreamExt};
use rand::rng;
#[cfg(feature = "playback")]
use rodio::OutputStreamBuilder;
use rodio::{source::ChannelVolume, Decoder, Sink};
use tokio::fs;

#[derive(Debug, Clone)]
enum Source {
    Api(&'static str, &'static str),
    Msgpack(&'static str, &'static str),
}

const SOURCES: &[Source] = &[
    Source::Api("PAC3-Server/chatsounds", "sounds/chatsounds"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "csgo"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "css"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "ep1"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "ep2"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "hl1"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "hl2"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "l4d"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "l4d2"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "portal"),
    Source::Msgpack("PAC3-Server/chatsounds-valve-games", "tf2"),
    Source::Api("NotAwesome2/chatsounds", "sounds"),
    Source::Api(
        "Metastruct/garrysmod-chatsounds",
        "sound/chatsounds/autoadd",
    ),
];

async fn setup() -> (Chatsounds, PathBuf) {
    let cache_path = PathBuf::from("cache");
    fs::create_dir_all(&cache_path).await.unwrap();

    let chatsounds = Arc::new(futures::lock::Mutex::new(
        Chatsounds::new(&cache_path).unwrap(),
    ));

    {
        println!("fetching sources");
        let chatsounds = chatsounds.clone();
        futures::stream::iter(SOURCES)
            .map(Ok)
            .try_for_each_concurrent(4, move |source| {
                let source = source.to_owned();
                let chatsounds = chatsounds.clone();
                async move {
                    let mut chatsounds = chatsounds.lock().await;
                    match source {
                        Source::Api(repo, repo_path) => {
                            let data = chatsounds.fetch_github_api(repo, repo_path).await?;
                            chatsounds.load_github_api(repo, repo_path, data).unwrap();
                        }

                        Source::Msgpack(repo, repo_path) => {
                            let data = chatsounds.fetch_github_msgpack(repo, repo_path).await?;
                            chatsounds
                                .load_github_msgpack(repo, repo_path, data)
                                .unwrap();
                        }
                    }
                    Ok::<_, Error>(())
                }
            })
            .await
            .unwrap();

        println!("done");
    }
    let chatsounds = Arc::try_unwrap(chatsounds).unwrap().into_inner();

    (chatsounds, cache_path)
}

#[cfg(feature = "playback")]
#[ignore]
#[tokio::test]
async fn negative_pitch() {
    let (mut chatsounds, _) = setup().await;

    let (sink, _) = chatsounds
        .play("helloh:speed(1) musicbox:pitch(-1)", rng())
        .await
        .unwrap();
    sink.sleep_until_end();
}

#[cfg(feature = "playback")]
#[ignore]
#[tokio::test]
async fn it_works() {
    let (mut chatsounds, _) = setup().await;

    let (sink, played_chatsounds) = chatsounds
        .play(
            "helloh:speed(1) idubbbz cringe:speed(1.2):echo(0.5,0.2) dad please:speed(0.5)",
            rng(),
        )
        .await
        .unwrap();
    let urls = played_chatsounds
        .iter()
        .map(|c| c.get_web_url())
        .collect::<Vec<_>>();
    println!("{:#?}", urls);
    sink.sleep_until_end();
}

#[tokio::test]
async fn test_search() {
    let (chatsounds, _) = setup().await;

    let search = "and thats what";

    let t0 = Instant::now();
    let positions = chatsounds.search(search);
    let t1 = Instant::now();
    println!("took {:?}", t1 - t0);

    println!(
        "{:#?}",
        positions.iter().rev().take(10).rev().collect::<Vec<_>>()
    );
}

#[tokio::test]
async fn test_get() {
    let (chatsounds, _) = setup().await;

    let chatsounds = chatsounds.get("a").unwrap();

    println!(
        "{:#?}",
        chatsounds.iter().map(|c| c.get_url()).collect::<Vec<_>>()
    );
}

#[cfg(feature = "playback")]
#[ignore]
#[tokio::test]
async fn test_spatial() {
    let (mut chatsounds, _) = setup().await;

    let mut emitter_pos = [2.0, 0.0, 0.0];
    let left_ear_pos = [-1.0, 0.0, 0.0];
    let right_ear_pos = [1.0, 0.0, 0.0];

    println!("play");
    let (sink, _) = chatsounds
        .play_spatial(
            "fuckbeesremastered",
            rng(),
            emitter_pos,
            left_ear_pos,
            right_ear_pos,
        )
        .await
        .unwrap();

    // start in right headphone, move left
    let mut i: f32 = 0.0;
    while !sink.empty() {
        emitter_pos[0] = i.cos() * 2.0;

        sink.set_left_ear_position(left_ear_pos);
        sink.set_right_ear_position(right_ear_pos);
        sink.set_emitter_position(emitter_pos);

        std::thread::sleep(Duration::from_millis(10));
        i += 0.01;
    }

    sink.sleep_until_end();

    chatsounds.sleep_until_end();
}

#[cfg(feature = "playback")]
#[ignore]
#[tokio::test]
async fn test_mono_bug() {
    let (mut chatsounds, cache_path) = setup().await;

    let sounds = chatsounds.get("mktheme").unwrap();
    if let Some(chatsound) = sounds.first().cloned() {
        println!("play");

        let data = chatsound.load(&cache_path).await.unwrap();

        let reader = BufReader::new(Cursor::new(data));
        let source = Decoder::new(reader).unwrap();

        let source = ChannelVolume::new(source, vec![0.2, 1.0]);
        let output_stream = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(output_stream.mixer());
        sink.set_volume(0.1);
        sink.append(source);

        sink.sleep_until_end();
        drop(output_stream);
    }

    chatsounds.sleep_until_end();
}

#[tokio::test]
async fn test_get_samples() {
    let (mut chatsounds, _) = setup().await;

    let (mut sources, _): (Vec<_>, Vec<_>) = chatsounds
        .get_sources("mktheme", rng())
        .await
        .unwrap()
        .into_iter()
        .unzip();
    let (sink, queue) = rodio::queue::queue(false);
    for source in sources.drain(..) {
        sink.append(source);
    }

    let queue = rodio::source::UniformSourceIterator::new(queue, 2, 44100);

    println!("{} samples", queue.count());
}
