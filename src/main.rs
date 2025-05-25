use cli::run;

mod cli;

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

    run().await;
}
