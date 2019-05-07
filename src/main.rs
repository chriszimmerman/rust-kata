fn main() {
    println!("Hello, world!");
}

pub fn score(result_of_game: &str) -> i32 {
    let frames = result_of_game.split_whitespace();
    let mut score = 0;
    let mut previous_frame_was_spare = false;
    let mut strike_bonus_rolls = 0;

    for (frame_number, frame) in frames.enumerate() {
        let mut frame_chars = frame.chars();

        let first_roll_str = match frame_chars.next() {
            Some(c) => c.to_string(),
            None => String::from("-")
        };
        let second_roll_str = match frame_chars.next() {
            Some(c) => c.to_string(),
            None => String::from("-")
        };

        let first_roll = add_roll(&first_roll_str);
        let second_roll = add_roll(&second_roll_str);

        if previous_frame_was_spare && is_standard_frame(frame_number) {
            score += first_roll;
        }

        previous_frame_was_spare = false;

        if is_spare(&second_roll_str) {
            previous_frame_was_spare = true;
            score += 10;

            if strike_bonus_rolls > 0 {
                score += 10;
                strike_bonus_rolls -= 2;
            }
        }
        else if is_strike(&first_roll_str) {
            if is_standard_frame(frame_number) {
                if strike_bonus_rolls > 0 {
                    score += 10;
                    strike_bonus_rolls -= 1;
                }
                if strike_bonus_rolls > 0 {
                    score += 10;
                    strike_bonus_rolls -= 1;
                }
            }
            strike_bonus_rolls += 2;
            score += 10;
        }
        else {
            score += first_roll + second_roll;

            if strike_bonus_rolls > 0 {
                score += first_roll + second_roll;
                strike_bonus_rolls -= 2;
            }
        }

        if let Some(last_frame_extra_roll) = frame_chars.next() {
            score += add_roll(&last_frame_extra_roll.to_string())
        }
    }

    return score;
}

fn add_roll(roll_str: &String) -> i32 {
    if roll_str == "X" {
        return 10;
    }
    if roll_str == "/" {
        return 0;
    }
    if roll_str == "-" {
        return 0;
    }
    return roll_str.parse::<i32>().unwrap();
}

fn is_standard_frame(frame_number: usize) -> bool {
    return frame_number < 10;
}

fn is_spare(second_roll: &String) -> bool {
    return second_roll == "/";
}

fn is_strike(first_roll: &String) -> bool {
    return first_roll == "X";
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

    #[test]
    fn test_strikes() {
        assert_eq!(score("X X X X X X X X X X X X"), 300);
    }

    #[test]
    fn test_mixed_game() {
        assert_eq!(score("14 45 6/ 5/ X -1 7/ 6/ X 2/ 6"), 133);
    }
}
