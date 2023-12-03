use crate::{Bag, Game};

fn calculate_max_colors(bags: Vec<Bag>) -> u32 {
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
    for bag in bags {
        if let Some(red) = bag.red {
            if max_red < red {
                max_red = red;
            }
        }
        if let Some(green) = bag.green {
            if max_green < green {
                max_green = green;
            }
        }
        if let Some(blue) = bag.blue {
            if max_blue < blue {
                max_blue = blue;
            }
        }
    }

    max_red * max_blue * max_green
}

pub fn solve(games: Vec<Game>) -> u32 {
    let mut result = 0;
    for game in games {
        result += calculate_max_colors(game.bags);
    }

    result
}
