use std::{fs, path::Path, ffi::OsStr};
use serde_json::{from_str, Value};
use serde::{Serialize, Deserialize};

use crate::prelude::*;

const CHARACTER_PATH: &str = "./characters/";

#[derive(Serialize, Deserialize)]
struct CardFields {
    title: Vec<String>,
    head: Vec<String>,
    body: Vec<String>,
}

pub fn get_card_data(path: &str) -> CardData {
    let data: String = fs::read_to_string(path).expect("failed reading file.");
    let fields: CardFields = from_str(&data).expect("failed parsing json");

    (fields.title, fields.head, fields.body)
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
    let path_list = get_json_in_dir(CHARACTER_PATH);
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