use rand::{thread_rng, Rng};

pub fn roll_die() -> u8 {
    let mut rng = thread_rng();
    *rng.choose(&[0, 1, 2]).unwrap()
}

pub fn roll_multiple(dice: u8) -> u8 {
    let mut rolls: Vec<u8> = Vec::new();
    for _ in 0..dice {
        rolls.push(roll_die());
    }
    rolls.into_iter()
        .reduce(|x, y| {x + y})
        .unwrap()
}

pub fn trait_roll(dice: u8, target: u8) -> bool {
    let rolls = roll_multiple(dice);
    rolls >= target
}