fn main() {
    println!("Hello, world!");
}

pub fn score(result_of_game: &str) -> i32 {
    let frames = result_of_game.split_whitespace();
    let mut score = 0;
    let mut previous_frame_was_spare = false;

    for frame in frames {
        let mut frame_chars = frame.chars();
        let first_roll_str = frame_chars.next().unwrap().to_string();
        let second_roll_str = frame_chars.next().unwrap().to_string();
        let first_roll = add_roll(&first_roll_str);
        let second_roll = add_roll(&second_roll_str);

        if previous_frame_was_spare {
            score += first_roll;
            previous_frame_was_spare = false;
        }
        if second_roll_str == "/" {
            previous_frame_was_spare = true;
            score += 10;
        }
        else {
            score += first_roll + second_roll;
        }

        if let Some(last_frame_extra_roll) = frame_chars.next() {
            score += add_roll(&last_frame_extra_roll.to_string())
        }
    }

    return score;
}

fn add_roll(roll_str: &String) -> i32 {
    if roll_str == "/" {
        return 0;
    }
    if roll_str != "-" {
        return roll_str.parse::<i32>().unwrap();
    }
    return 0;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_nothing_but_misses() {
        assert_eq!(score("-- -- -- -- -- -- -- -- -- --"), 0);
    }

    #[test]
    fn test_knock_some_pins_down_but_not_all() {
        assert_eq!(score("1- 1- 1- 1- 1- 1- 1- 1- 1- 1-"), 10);
    }

    #[test]
    fn test_knock_some_pins_down_during_each_roll_but_not_all() {
        assert_eq!(score("13 13 13 13 13 13 13 13 13 13"), 40);
    }

    #[test]
    fn test_spare() {
        assert_eq!(score("5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/ 5/5"), 150);
    }
}
