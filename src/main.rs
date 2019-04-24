fn main() {
    println!("Hello, world!");
}

pub fn score(result_of_game: &str) -> i32 {
    let frames = result_of_game.split_whitespace();
    let mut score = 0;
    for frame in frames {
        let first_roll_str = frame.chars().next().unwrap().to_string();
        if(first_roll_str != "-") {
            score += first_roll_str.parse::<i32>().unwrap();
        }
    }
    return score;
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
}