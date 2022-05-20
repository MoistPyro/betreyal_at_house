pub struct Character {
    pub name: String,
    pub age: u8,
    pub height: u16,
    pub weight: u16,
    pub hobbies: Vec<String>,
    pub birthday: Vec<u8>,
    stats: Vec<Vec<u8>>,
    starting_stats: Vec<u8>,
    pub stat_indexes: Vec<u8>,
}

impl Character {
    pub fn new(
        name: String, age: u8, height: u16, weight: u16, hobbies: Vec<String> , birthday: Vec<u8>,
        stats: Vec<Vec<u8>>, starting_stats: Vec<u8>
    ) -> Self {
        Self {
            name, age, height, weight, hobbies, birthday, stats, starting_stats: starting_stats.clone(),
            stat_indexes: starting_stats,
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

    pub fn take_dmg_physical(&mut self, dmg: u8) {}
    pub fn take_dmg_mental(&mut self, dmg: u8) {}
}