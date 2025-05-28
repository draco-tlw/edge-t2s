use std::{error::Error, fs};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Narrator {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "ShortName")]
    pub short_name: String,

    #[serde(rename = "Gender")]
    pub gender: String,

    #[serde(rename = "Locale")]
    pub locale: String,

    #[serde(rename = "FriendlyName")]
    pub friendly_name: String,
}

pub fn read_narrators_list() -> Result<Vec<Narrator>, Box<dyn Error>> {
    let narrators_str = fs::read_to_string("narrators-list.json")?;

    let narrators: Vec<Narrator> = serde_json::from_str(&narrators_str)?;

    Ok(narrators)
}

pub fn extract_locales() -> Result<Vec<String>, Box<dyn Error>> {
    let narrators = read_narrators_list()?;
    let mut locales: Vec<String> = narrators.into_iter().map(|n| n.locale.clone()).collect();
    locales.sort();
    locales.dedup();

    Ok(locales)
}

pub fn filter_narrators_by_locale(locale: &str) -> Result<Vec<Narrator>, Box<dyn Error>> {
    let narrators = read_narrators_list()?;
    let narrators: Vec<Narrator> = narrators
        .into_iter()
        .filter(|n| n.locale == locale)
        .collect();

    Ok(narrators)
}
