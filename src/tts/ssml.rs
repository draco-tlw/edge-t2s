use super::edge_tts::edge_tts_config::EdgeTTSConfig;

pub fn ssml(config: &EdgeTTSConfig, content: String) -> String {
    format!(
        r#"<speak 
            xmlns="http://www.w3.org/2001/10/synthesis" 
            xmlns:mstts="http://www.w3.org/2001/mstts" 
            xmlns:emo="http://www.w3.org/2009/10/emotionml" 
            version="1.0" 
            xml:lang="en-US"
        >
            <voice name="{}">
                <prosody rate="{}%" pitch="{}%">
                    {}
                </prosody >
            </voice > 
        </speak >"#,
        config.narrator_name, config.rate, config.pitch, content
    )
}
