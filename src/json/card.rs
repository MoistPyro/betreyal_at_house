use std::io::Error;
use json::JsonValue;

use crate::gameobjects::card::{Card, LifeTime, PermanentType, CardTypes};
use crate::json::json_reader::get_jsons_in_dir;

const EVENT_CARD_PATH: &str = "./cards/event";
const ITEM_CARD_PATH: &str = "./cards/item";
const OMEN_CARD_PATH: &str = "./cards/omen";

fn construct_card(data: &JsonValue, card_type: CardTypes) -> Result<Card, Error> {
    if let JsonValue::Object(i) = data {
        let title = i.get("title").unwrap()
            .as_str().unwrap().to_string();
        let head = i.get("head").unwrap()
            .as_str().unwrap().to_string();
        let body = i.get("body").unwrap()
            .as_str().unwrap().to_string();

        let lifetime: Result<LifeTime, Error> = match i.get("lifetime").unwrap().as_str() {
            Some(i) if i == "OneShot" => Ok(LifeTime::OneShot),
            Some(i) if i == "Consumable" => Ok(LifeTime::Consumable),
            Some(i) if i == "Weapon" => Ok(LifeTime::Permanent(PermanentType::Weapon)),
            Some(i) if i == "Companion" => Ok(LifeTime::Permanent(PermanentType::Companion)),
            Some(i) if i == "PermanentItem" => Ok(LifeTime::Permanent(PermanentType::Item)),
            Some(_) => panic!("bad formating"),
            None => panic!("missing field")
        };
        let lifetime: LifeTime = lifetime?;

        if let JsonValue::Object(m) = i.get("methods").unwrap() {
            //how to do this?
        };
        let draw_method = Card::dummy_func;
        let activate_method = Card::dummy_func;
        let spend_method = Card::dummy_func;
        let discard_method = Card::dummy_func;

        Ok(Card::new(title, head, body, lifetime, card_type,
            draw_method, activate_method, spend_method, discard_method))
    } else {panic!("Bad json format")}
}

fn get_cards(path: &str, card_type: CardTypes) -> Result<Vec<Card>, Error> {
    let data_list: Vec<JsonValue> = get_jsons_in_dir(path)
        .expect("failed reading directory");
    let mut cards = Vec::<Card>::new();

    for data in data_list {
        cards.push(construct_card(&data, card_type).expect("failed construction"));
    };
    Ok(cards)
}

pub fn get_event_cards() -> Result<Vec<Card>, Error> {
    get_cards(EVENT_CARD_PATH, CardTypes::Event)
}

pub fn get_item_cards() -> Result<Vec<Card>, Error> {
    get_cards(ITEM_CARD_PATH, CardTypes::Item)
}

pub fn get_omen_cards() -> Result<Vec<Card>, Error> {
    get_cards(OMEN_CARD_PATH, CardTypes::Omen)
}