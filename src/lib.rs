use rodio::{Decoder, Device, Source};
use std::io::{prelude::*, BufReader, Cursor};

pub fn load() {
  let repo = "Metastruct/garrysmod-chatsounds";
  let api_url = format!(
    "https://api.github.com/repos/{}/git/trees/master?recursive=1",
    repo
  );

  // reqwest::get(api_url).unwrap().json();

  let path = "sound/chatsounds/autoadd/cracklife/banditradio.ogg";
  let download_url = format!("https://raw.githubusercontent.com/{}/master/{}", repo, path);

  let device = rodio::default_output_device().unwrap();

  #[cfg(test)]
  {
    let data = download(download_url);

    play(&device, data);

    std::thread::sleep_ms(10000);
  }
}

fn download(url: String) -> Vec<u8> {
  let mut response = reqwest::get(&url).unwrap();

  let mut data = Vec::new();
  response.read_to_end(&mut data).unwrap();

  data
}

// https://raw.githubusercontent.com/" .. repo .. "/master/" .. location .. "/" .. path

fn play(device: &Device, data: Vec<u8>) {
  let reader = BufReader::new(Cursor::new(data));

  let source = Decoder::new(reader).unwrap();
  rodio::play_raw(&device, source.convert_samples());
}

#[test]
fn it_works() {
  load();
}
