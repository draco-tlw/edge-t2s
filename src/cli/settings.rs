use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, File},
    io::stdin,
};

use crate::cli::utils::print::print;

use super::SETTINGS_PATH;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub narrator: String,
    pub rate: i16,
    pub pitch: i16,
}

impl Settings {
    fn get_narrator() -> String {
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

        loop {
            println!("choose a narrator: ");

            for (i, n) in narrators.iter().enumerate() {
                println!("[{i}]: {n}");
            }

            print("=> ");
            let mut narrator_i = String::new();
            match stdin().read_line(&mut narrator_i) {
                Err(err) => {
                    println!("invalid input: {}\n", err);
                    continue;
                }
                _ => (),
            }

            let i: usize = match narrator_i.trim().parse() {
                Ok(value) => value,
                Err(err) => {
                    println!("invalid input: {}\n", err);
                    continue;
                }
            };

            break match narrators.get(i) {
                Some(narrator) => narrator.to_string(),
                None => {
                    println!("invalid input: index {} out of bounds\n", i);
                    continue;
                }
            };
        }
    }

    fn get_rate() -> i16 {
        loop {
            print("rate (in %): ");
            let mut rate = String::new();

            match stdin().read_line(&mut rate) {
                Err(err) => {
                    println!("invalid input: {}\n", err);
                    continue;
                }
                _ => (),
            };

            let rate: i16 = match rate.trim().parse() {
                Err(err) => {
                    println!("invalid input: {}\n", err);
                    continue;
                }
                Ok(v) => v,
            };

            if rate > 100 || rate < -100 {
                println!(
                    "invalid input: rate out of bounds. expected between -100 and 100, got {}\n",
                    rate
                );
                continue;
            }

            break rate;
        }
    }

    fn get_pitch() -> i16 {
        loop {
            print("pitch (in %): ");
            let mut pitch = String::new();

            match stdin().read_line(&mut pitch) {
                Err(err) => {
                    println!("invalid input: {}\n", err);
                    continue;
                }
                _ => (),
            };

            let pitch: i16 = match pitch.trim().parse() {
                Err(err) => {
                    println!("invalid input: {}\n", err);
                    continue;
                }
                Ok(v) => v,
            };

            if pitch > 100 || pitch < -100 {
                println!(
                    "invalid input: pitch out of bounds. expected between -100 and 100, got {}\n",
                    pitch
                );
                continue;
            }

            break pitch;
        }
    }

    pub fn get() -> Self {
        let narrator = Self::get_narrator();
        let rate = Self::get_rate();
        let pitch = Self::get_pitch();

        Self {
            narrator,
            rate,
            pitch,
        }
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;

        fs::write(SETTINGS_PATH, json.as_bytes())?;

        Ok(())
    }

    pub fn read() -> Result<Self, Box<dyn Error>> {
        let settings_str = fs::read_to_string(SETTINGS_PATH)?;

        let settings: Self = serde_json::from_str(&settings_str)?;
        Ok(settings)
    }
}
