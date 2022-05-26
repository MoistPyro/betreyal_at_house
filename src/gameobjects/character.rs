use crate::prelude::CharacterData;

pub struct Character {
    pub name: String,
    pub age: u64,
    pub height: u64,
    pub weight: u64,
    pub hobbies: Vec<String>,
    pub birthday: Vec<u64>,
    pub stats: Vec<Vec<u8>>,
    pub starting_indexes: Vec<u8>,
    pub stat_indexes: Vec<u8>,
}

impl Character {
    pub fn new(data: CharacterData) -> Self {
        Self {
            name: data.0,
            age: data.1,
            height: data.2,
            weight: data.3,
            hobbies: data.4,
            birthday: data.5,
            stats: data.6,
            starting_indexes: data.7.to_owned(),
            stat_indexes: data.7,
        }
    }

    pub fn get_stat(&self, stat:usize) -> u8 {
        let index: usize = self.stat_indexes[stat].into();
        self.stats[stat][index]
    }
    pub fn modify_stat(&mut self, i: usize, value: i8) {
        if value > 0 {
            let positive: u8 = value.try_into().unwrap();
            self.stat_indexes[i] += positive;
        } else {
            let negative: u8 = (value * -1).try_into().unwrap();
            self.stat_indexes[i] -= negative;
        }
    }
    pub fn set_stat(&mut self, i: usize, value: u8) {
        self.stats[i][0] = value;
    }
    pub fn get_starting_stat(&self, stat: usize) -> u8 {
        let index: usize = self.starting_indexes[stat].into();
        self.stats[stat][index]
    }

    pub fn take_dmg_physical(&mut self, dmg: u8) {}
    pub fn take_dmg_mental(&mut self, dmg: u8) {}
}