use std::fs;

use tts::{
    TTS,
    edge_tts::{EdgeTTS, edge_tts_config::EdgeTTSConfig},
};

pub mod tts;
pub mod utils;

#[tokio::main]
async fn main() {
    let tts = EdgeTTS::new(EdgeTTSConfig::new(
        String::from("en-US-AriaNeural"),
        None,
        None,
        None,
    ));

    let mut client = tts.connect().await.expect("failed to connect to tts");
    let audio_data = tts
        .send_content(&mut client, String::from("hello this is test"))
        .await
        .expect("failed to send content");

    fs::write("output/test.mp3", audio_data).expect("failed to write auido file");
}
