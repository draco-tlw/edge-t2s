use std::error::Error;

use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub mod edge_tts;
pub mod ssml;

type TTSSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub trait TTS {
    async fn connect(&self) -> Result<TTSSocket, Box<dyn Error>>;
}
