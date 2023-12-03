use crate::{Bag, Game};

pub fn is_constraint_satisfied(constraint: &Bag, bag: Bag) -> bool {
    if let Some(red) = bag.red {
        if red > constraint.red.unwrap() {
            return false;
        }
    }
    if let Some(green) = bag.green {
        if green > constraint.green.unwrap() {
            return false;
        }
    }
    if let Some(blue) = bag.blue {
        if blue > constraint.blue.unwrap() {
            return false;
        }
    }

    true
}

pub fn solve(constraint: &Bag, games: Vec<Game>) -> u32 {
    let mut result = 0;
    for game in games {
        let mut is_game_in_result = true;
        for bag in game.bags {
            if !is_constraint_satisfied(constraint, bag) {
                is_game_in_result = false;
            }
        }
        if is_game_in_result {
            result += game.id;
        }
    }

    result
}
