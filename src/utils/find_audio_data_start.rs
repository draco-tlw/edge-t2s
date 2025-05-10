const PATH_AUDIO: &[u8] = "Path:audio\r\n".as_bytes();
pub fn find_audio_data_start(vec: Vec<u8>) -> usize {
    match vec.windows(PATH_AUDIO.len()).position(|w| w == PATH_AUDIO) {
        Some(i) => i + PATH_AUDIO.len(),
        None => 0,
    }
}
