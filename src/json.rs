use std::{fs, path::Path, ffi::OsStr};
use serde_json::{from_str, Value};
use serde::Deserialize;

use crate::prelude::*;

///the .json file must fit the CardFields stuct, or this panics.
pub fn get_card_data(path: &str) -> CardFields {
    let data: String = fs::read_to_string(path).expect("failed reading file.");
    let fields: CardFields = from_str(&data).expect("failed parsing json");

    fields
}

pub fn get_character_data(path: &str) -> CharacterData {
    let data: String = fs::read_to_string(path).expect("failed reading file.");
    let v: Value = from_str(&data).expect("failed parsing json");

    let name: String = v["name"].as_str().unwrap().to_string();
    let age: u64 = v["age"].as_u64().unwrap();
    let height: u64 = v["height"].as_u64().unwrap();
    let weight: u64 = v["weight"].as_u64().unwrap();

    let hobbies: Vec<String> = vec![
        v["hobbies"][0].as_str().unwrap().to_string(),
        v["hobbies"][1].as_str().unwrap().to_string()
        ];
    let birthday: Vec<u64> = vec![
        v["birthday"][0].as_u64().unwrap(),
        v["birthday"][1].as_u64().unwrap()
        ];
    
    let stat_map: &serde_json::Map<String, Value> = v["stats"].as_object().unwrap();
    let might: Vec<u8> = stat_map.get("might").unwrap()
        .as_array().unwrap()
        .iter().map(|s| -> u8 {
            s.as_u64().unwrap().try_into().expect("number too large")
        })
        .collect();
    let speed: Vec<u8> = stat_map.get("speed").unwrap()
        .as_array().unwrap()
        .iter().map(|s| -> u8 {
            s.as_u64().unwrap().try_into().expect("number too large")
        })
        .collect();
    let sanity: Vec<u8> = stat_map.get("sanity").unwrap()
        .as_array().unwrap()
        .iter().map(|s| -> u8 {
            s.as_u64().unwrap().try_into().expect("number too large")
        })
        .collect();
    let knowledge: Vec<u8> = stat_map.get("knowledge").unwrap()
        .as_array().unwrap()
        .iter().map(|s| -> u8 {
            s.as_u64().unwrap().try_into().expect("number too large")
        })
        .collect();
    
    let stats: Vec<Vec<u8>> = vec![might, speed, sanity, knowledge];
    let starting_stats: Vec<u8> = stat_map.get("starting_stats").unwrap()
        .as_array().unwrap()
        .iter().map(|s| -> u8 {s.as_u64().unwrap().try_into().expect("number too large")})
        .collect();

        (name, age, height, weight, hobbies, birthday, stats, starting_stats)
    }
    
pub fn get_all_character_data() -> Vec<CharacterData> {
    let character_path = get_settings().character_path;
    let path_list = get_json_in_dir(&character_path);
    path_list.iter().map(|p| get_character_data(p)).collect()
}

fn get_json_in_dir(p: &str) -> Vec<String> {
    let path: &Path = Path::new(p);
    let mut r: Vec<String> = Vec::new();
    if path.is_dir() {
        let directory: fs::ReadDir = fs::read_dir(path).unwrap();
        for file in directory {
            let f = file.expect("").path();
            let ext: &OsStr = f.extension().unwrap();
            if ext == OsStr::new("json") {
                let f: String = f.to_str().unwrap().to_owned();
                r.push(f)
            };
        };
    }
    r
}

#[derive(Deserialize)]
pub struct Settings {
    pub game_title: String,
    pub window_size: [f64; 2],
    pub background_colour: [f32; 4],
    pub card_border_colour: [f32; 4],
    pub card_text_colour: [f32; 4],
    pub card_event_bg_colour: [f32; 4],
    pub card_item_bg_colour: [f32; 4],
    pub card_omen_bg_colour: [f32; 4],
    pub card_size: [f64; 2],
    pub card_title_font_path: String,
    pub card_head_font_path: String,
    pub card_body_font_path: String,
    pub card_title_size: u32,
    pub card_head_size: u32,
    pub card_body_size: u32,
    pub character_path: String,
}

///reads settings.json, and returns an object with all the settings.
pub fn get_settings() -> Settings {
    let path = SETTINGS_PATH;
    let data: String = fs::read_to_string(path).expect("failed reading file.");
    let settings: Settings = from_str(&data).expect("failed parsing json");
    settings
}