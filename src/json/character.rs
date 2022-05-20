use std::io::Error;
use json::JsonValue;

use crate::gameobjects::character::Character;
use crate::json::json_reader::get_jsons_in_dir;

const CHARACTER_PATH: &str = "./characters/";

pub fn get_characters() -> Result<Vec<Character>, Error> {
    Ok(get_jsons_in_dir(CHARACTER_PATH)
        .expect("failed reading directory")
        .iter()
        .map(|x| construct_character(x).expect("failed construction"))
        .collect())
}

fn construct_character(data: &JsonValue) -> Result<Character, Error> {
    if let JsonValue::Object(i) = data {
        let name: String = i.get("name").unwrap().as_str().unwrap().to_string();
        let age: u8 = i.get("age").unwrap().as_u8().unwrap();
        let height: u16 = i.get("height").unwrap().as_u16().unwrap();
        let weight: u16 = i.get("weight").unwrap().as_u16().unwrap();
        let hobbies: Vec<String> = i.get("hobbies").unwrap().members()
            .map(|x| x.as_str().unwrap().to_string()).collect();
        let birthday: Vec<u8> = i.get("birthday").unwrap().members()
            .map(|x| x.as_u8().unwrap()).collect();

        let mut stats: Vec<Vec<u8>> = Vec::<Vec<u8>>::new();
        if let JsonValue::Object(j) = i.get("stats").unwrap() {
            stats.push(
                j.get("might").unwrap().members()
                .map(|x| x.as_u8().unwrap()).collect()
            );
            stats.push(
                j.get("speed").unwrap().members()
                .map(|x| x.as_u8().unwrap()).collect()
            );
            stats.push(
                j.get("sanity").unwrap().members()
                .map(|x| x.as_u8().unwrap()).collect()
            );
            stats.push(
                j.get("knowledge").unwrap().members()
                .map(|x| x.as_u8().unwrap()).collect()
            );
            stats.push(
                j.get("starting_stats").unwrap().members()
                .map(|x| x.as_u8().unwrap()).collect()
            );
        }
        let starting_stats = stats.pop().unwrap();
        stats.reverse();

        Ok(Character::new(name, age, height, weight, hobbies, birthday,
            stats, starting_stats))
    } else {panic!("Bad json format")}
}