use json::{self, JsonValue};
use std::{fs, path::Path, io::Error};
use crate::gameobject::{Card, CardTypes, PermanentType};

const EVENT_CARD_PATH: &str = "./cards/event/";
const ITEM_CARD_PATH: &str = "./cards/item/";
const OMEN_CARD_PATH: &str = "./cards/omen/";

fn read_json(path: &Path) -> JsonValue {
    let data = fs::read_to_string(path).expect("failed reading file.");
    json::parse(data.as_str()).unwrap()
}

fn get_jsons_in_dir(path: &str) -> Result<Vec<JsonValue>, Error> {
    let path: &Path = Path::new(path);
    let mut r: Vec<JsonValue> = Vec::<JsonValue>::new();
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if entry.path().extension().unwrap() == "json" {
                r.push(read_json(entry.path().as_path()));
            }
        }
    }
    Ok(r)
}

fn construct_card(data: JsonValue) -> Result<Card, Error> {
    if let JsonValue::Object(i) = data {
        let title = i.get("title").unwrap().as_str().unwrap().to_string();
        let head = i.get("head").unwrap().as_str().unwrap().to_string();
        let body = i.get("body").unwrap().as_str().unwrap().to_string();

        let card_type = match i.get("card_type").unwrap().as_str() {
            Some(i) if i == "OneShot" => Ok(CardTypes::OneShot),
            Some(i) if i == "Consumable" => Ok(CardTypes::Consumable),
            Some(i) if i == "Weapon" => Ok(CardTypes::Permanent(PermanentType::Weapon)),
            Some(i) if i == "Companion" => Ok(CardTypes::Permanent(PermanentType::Companion)),
            Some(i) if i == "PermanentItem" => Ok(CardTypes::Permanent(PermanentType::Item)),
            _ => Err(())
        }.unwrap();

        if let JsonValue::Object(m) = i.get("methods").unwrap() {}

        Ok(Card::new(title, head, body, card_type,
            draw_method, activate_method, spend_method, discard_method))
    }
}

pub fn get_event_cards() -> Result<Vec<Card>, Error> {
    get_jsons_in_dir(EVENT_CARD_PATH)
        .unwrap()
        .iter()
        .map(|x| construct_card(*x))
        .collect()
}

pub fn get_item_cards() -> Result<Vec<Card>, Error> {
    get_jsons_in_dir(ITEM_CARD_PATH)
        .unwrap()
        .iter()
        .map(|x| construct_card(*x))
        .collect()
}

pub fn get_omen_cards() -> Result<Vec<Card>, Error> {
    get_jsons_in_dir(OMEN_CARD_PATH)
        .unwrap()
        .iter()
        .map(|x| construct_card(*x))
        .collect()
}