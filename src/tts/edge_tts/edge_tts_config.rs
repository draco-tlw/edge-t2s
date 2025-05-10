use crate::utils::now::now;

pub struct EdgeTTSConfig {
    // short_name of narrators
    // https://speech.platform.bing.com/consumer/speech/synthesize/readaloud/voices/list?trustedclienttoken=6A5AA1D4EAFF4E9FB37E23D68491D6F4
    pub narrator_name: String,

    // between -100 to 100
    pub rate: i16,
    pub pitch: i16,

    pub output_codec: String,
}

impl EdgeTTSConfig {
    pub fn new(
        narrater_name: String,
        rate: Option<i16>,
        pitch: Option<i16>,
        output_codec: Option<String>,
    ) -> Self {
        Self {
            narrator_name: narrater_name,
            rate: rate.unwrap_or(0),
            pitch: pitch.unwrap_or(0),
            output_codec: output_codec.unwrap_or(String::from("audio-24khz-48kbitrate-mono-mp3")),
        }
    }

    fn to_json_str(&self) -> String {
        format!(
            r#"{{
                "context": {{
                    "synthesis": {{
                        "audio": {{
                            "metadataoptions": {{
                                "sentenceBoundaryEnabled": "false",
                                "wordBoundaryEnabled": "false"
                            }},
                            "outputFormat": "{}"
                        }}
                    }}
                }}
            }}"#,
            self.output_codec
        )
    }

    pub fn to_msg_str(&self) -> String {
        format!(
            "X-Timestamp:{}\r\nContent-Type:application/json; charset=utf-8\r\nPath:speech.config\r\n\r\n{}",
            now().as_millis(),
            self.to_json_str()
        )
    }
}
