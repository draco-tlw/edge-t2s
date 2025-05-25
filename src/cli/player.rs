use std::io::Cursor;

use rodio::{Decoder, OutputStream, Sink};

pub fn play_mp3(audio: Vec<u8>) {
    let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to open a connection to the default audio output device. Is an audio device connected and enabled?");
    let sink = Sink::try_new(&stream_handle)
        .expect("Failed to create an audio sink on the output device.");
    let cursor = Cursor::new(audio);

    let source = Decoder::new(cursor).expect("Failed to decode audio data. The data may be corrupted or not in a supported format (e.g., MP3).");
    sink.append(source);

    println!("Playing audio...");
    sink.sleep_until_end();
    println!("Audio finished.");
}
