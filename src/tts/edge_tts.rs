use async_trait::async_trait;
use std::error::Error;

use edge_tts_config::EdgeTTSConfig;
use futures_util::{SinkExt, StreamExt};
use http::{
    HeaderMap, HeaderValue, Uri,
    header::{ORIGIN, USER_AGENT},
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tungstenite::client::IntoClientRequest;
use url::{ParseError, Url};

use crate::utils::{
    find_audio_data_start::find_audio_data_start, gen_request_id::gen_request_id, now::now,
};

use super::{TTS, TTSSocket, ssml::ssml};

pub mod edge_tts_config;

pub struct EdgeTTS {
    config: EdgeTTSConfig,
    request_id: String,
    headers: HeaderMap,
}

impl EdgeTTS {
    pub fn new(config: EdgeTTSConfig) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.66 Safari/537.36 Edg/103.0.1264.44"));
        Self {
            headers,
            config,
            request_id: gen_request_id(),
        }
    }

    fn connect_url(&self) -> Result<Url, ParseError> {
        let url = format!(
            "wss://speech.platform.bing.com/consumer/speech/synthesize/readaloud/edge/v1?TrustedClientToken={}&ConnectionId={}",
            "6A5AA1D4EAFF4E9FB37E23D68491D6F4", self.request_id,
        );
        Url::parse(&url)
    }

    pub async fn send_ssml(
        &self,
        client: &mut TTSSocket,
        ssml: String,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let msg = format!(
            "X-Timestamp:{}\r\nX-RequestId:{}\r\nContent-Type:application/ssml+xml\r\nPath:ssml\r\n\r\n{}",
            now().as_millis(),
            gen_request_id(),
            ssml
        );
        let msg = Message::text(msg);
        client.send(msg).await?;

        let mut audio_data = Vec::new();
        let mut reciving_audio = false;

        while let Some(msg) = client.next().await {
            match msg? {
                Message::Text(text) => {
                    if text.contains("turn.start") {
                        reciving_audio = true;
                    } else if text.contains("turn.end") {
                        break;
                    }
                }
                Message::Binary(bin) => {
                    if reciving_audio {
                        let start = find_audio_data_start(bin.to_vec());
                        audio_data.extend_from_slice(&bin[start..]);
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
        Ok(audio_data)
    }

    pub async fn send_content(
        &self,
        client: &mut TTSSocket,
        content: String,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        let ssml = ssml(&self.config, content);
        self.send_ssml(client, ssml).await
    }

    pub fn change_narrator(&mut self, narrator_name: String) {
        self.config.narrator_name = narrator_name;
    }

    pub fn change_rate(&mut self, rate: i16) {
        self.config.rate = rate;
    }

    pub fn change_pitch(&mut self, pitch: i16) {
        self.config.pitch = pitch;
    }
}

#[async_trait]
impl TTS for EdgeTTS {
    async fn connect(&self) -> Result<TTSSocket, Box<dyn Error>> {
        let url = self.connect_url()?;
        let uri = url.as_str().parse::<Uri>()?;

        let mut request = uri.into_client_request()?;

        let req_headers = request.headers_mut();
        req_headers.insert(
            ORIGIN,
            HeaderValue::from_static("chrome-extension://jdiccldimpdaibmpdkjnbmckianbfold"),
        );

        // let mut request = Request::builder().method("GET").uri(uri).header(
        //     "Origin",
        //     "chrome-extension://jdiccldimpdaibmpdkjnbmckianbfold",
        // );

        for (key, value) in &self.headers {
            req_headers.insert(key, value.clone());
        }

        let (mut ws_stream, _) = connect_async(request).await?;

        let config_msg_str = self.config.to_msg_str();
        let config_msg = Message::Text(config_msg_str.into());

        ws_stream.send(config_msg).await?;

        Ok(ws_stream)
    }
}
