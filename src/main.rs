use std::{
    fs,
    io::{Write, stdin, stdout},
};

use edge_t2s::tts::{
    TTS,
    edge_tts::{EdgeTTS, edge_tts_config::EdgeTTSConfig},
};

#[tokio::main]
async fn main() {
    // let mut tts = EdgeTTS::new(EdgeTTSConfig::new(
    //     String::from("en-US-AriaNeural"),
    //     None, None,
    //     None,
    // ));
    //
    // let mut client = tts.connect().await.expect("failed to connect to tts");
    // let audio_data = tts
    //     .send_content(&mut client, String::from("hello this is test"))
    //     .await
    //     .expect("failed to send content");
    //
    // fs::write("output/test_en.mp3", audio_data).expect("failed to write auido file");
    //
    // tts.change_narrator(String::from("fa-IR-DilaraNeural"));
    // let audio_data = tts
    //     .send_content(&mut client, String::from("درود، این یک تست میباشد."))
    //     .await
    //     .expect("failed to send content");
    // fs::write("output/test_fa.mp3", audio_data).expect("failed to write auido file");

    let narrators = [
        "fa-IR-DilaraNeural",
        "fa-IR-FaridNeural",
        "en-US-AvaNeural",
        "en-US-AndrewNeural",
        "en-US-AnaNeural",
        "en-US-AriaNeural",
        "en-US-GuyNeural",
        "en-GB-LibbyNeural",
        "en-GB-MaisieNeural",
        "en-GB-RyanNeural",
        "en-GB-SoniaNeural",
        "en-GB-ThomasNeural",
    ];
    println!("edge-tts");
    println!("choose a narrator");
    for (i, n) in narrators.iter().enumerate() {
        println!("[{i}]: {n}");
    }
    print("=> ");
    let mut narrator_i = String::new();
    stdin().read_line(&mut narrator_i).expect("invalid input");
    let narrator_i: usize = narrator_i.trim().parse().expect("invalid input");
    if narrator_i > 11 {
        panic!("invalid input");
    }
    let narrator = narrators[narrator_i];

    print("rate (in %): ");
    let mut rate = String::new();
    stdin().read_line(&mut rate).expect("invalid input");
    let rate: i16 = rate.trim().parse().expect("invalid input");
    if rate > 100 || rate < -100 {
        panic!("invalid input");
    }

    print("pitch (in %): ");
    let mut pitch = String::new();
    stdin().read_line(&mut pitch).expect("invalid input");
    let pitch: i16 = pitch.trim().parse().expect("invalid input");
    if pitch > 100 || pitch < -100 {
        panic!("invalid input");
    }

    println!("text:");
    let mut text = String::new();
    stdin().read_line(&mut text).expect("invalid input");
    let text = text.trim();

    let tts = EdgeTTS::new(EdgeTTSConfig::new(
        narrator.to_string(),
        Some(rate),
        Some(pitch),
        None,
    ));

    let mut client = tts.connect().await.expect("failed to connect to tts");
    let audio_data = tts
        .send_content(&mut client, text.to_string())
        .await
        .expect("failed to send content");

    print("output path: (defualt = output/speach.mp3)");
    let mut output_path = String::new();
    stdin().read_line(&mut output_path).expect("invalid input");
    output_path = output_path.trim().to_string();
    if output_path == "" {
        output_path = String::from("output/speach.mp3");
    }

    fs::write(output_path, audio_data).expect("failed to write auido file");
}

fn print(msg: &str) {
    print!("{}", msg);
    stdout().flush().unwrap();
}
