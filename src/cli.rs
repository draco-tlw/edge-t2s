use edge_t2s::tts::{
    TTS,
    edge_tts::{EdgeTTS, edge_tts_config::EdgeTTSConfig},
};
use player::play_mp3;
use settings::Settings;
use std::{error::Error, io::stdin, path::Path};
use utils::print::print;

mod player;
mod settings;
pub mod utils;

const SETTINGS_PATH: &str = "settings.json";

pub async fn run() {
    if !Path::new(SETTINGS_PATH).exists() {
        println!("settings does not found!");

        println!("creating a settings file...");
        Settings::get()
            .write()
            .expect("failed to write settings file: {}")
    }

    let mut settings = Settings::read().expect("failed to read the settings file");

    loop {
        let choice = menu().expect("failed to read the input");
        match choice.as_str() {
            "0" => break,
            "1" => {
                Settings::get()
                    .write()
                    .expect("failed to write settings file: {}");
                settings = Settings::read().expect("failed to read the settings file");
            }
            "2" => t2s(&settings).await,
            _ => {
                println!("invalid input!");
                continue;
            }
        };
    }
}

pub fn menu() -> Result<String, Box<dyn Error>> {
    println!("\n===== T2S Menu =====");
    println!("1: Update settings");
    println!("2: Text to speech");
    println!("3: Listen on clipboard");
    println!("0: Exit");
    println!("====================");
    print("Enter your choice: ");

    let mut choice = String::new();

    stdin().read_line(&mut choice)?;

    Ok(choice.trim().to_string())
}

pub async fn t2s(settings: &Settings) {
    println!("text: ");
    let mut text = String::new();
    stdin()
        .read_line(&mut text)
        .expect("failed to read the input");

    let text = text.trim().to_string();

    let tts = EdgeTTS::new(EdgeTTSConfig::new(
        settings.narrator.to_string(),
        Some(settings.rate),
        Some(settings.pitch),
        None,
    ));
    let mut client = tts.connect().await.expect("failed to connect to tts!");
    let audio = tts
        .send_content(&mut client, text)
        .await
        .expect("failed to send content to tts");

    play_mp3(audio);
}
