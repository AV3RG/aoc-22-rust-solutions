use itertools::Itertools;
advent_of_code::solution!(2);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Hand {
    Rock,
    Paper,
    Scissor
}

impl Hand {
    const VALUES: [Self; 3] = [Self::Rock, Self::Paper, Self::Scissor];
}

#[derive(PartialEq)]
enum State {
    Win,
    Lose,
    Draw
}

fn char_to_hand(c: char) -> Hand {
    match c {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scissor,
        _ => panic!("Invalid hand")
    }
}

fn char_to_state(c: char) -> State {
    match c {
        'X' => State::Lose,
        'Y' => State::Draw,
        'Z' => State::Win,
        _ => panic!("Invalid state")
    }
}

fn player_win(opp: &Hand, player: &Hand) -> State {
    if opp == player {
        return State::Draw;
    }
    match (player, opp) {
        (Hand::Rock, Hand::Scissor) => State::Win,
        (Hand::Scissor, Hand::Paper) => State::Win,
        (Hand::Paper, Hand::Rock) => State::Win,
        _ => State::Lose
    }
}

fn state_to_points(state: &State) -> u32 {
    match state {
        State::Win => 6,
        State::Lose => 0,
        State::Draw => 3
    }
}

fn hand_to_points(hand: &Hand) -> u32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissor => 3
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split("\n").map(|line| {
        let (opp, player) = line.split(" ")
            .map(|c| c.chars().next().expect("Empty string"))
            .map(char_to_hand)
            .into_iter().collect_tuple().unwrap();
        state_to_points(&player_win(&opp, &player)) + hand_to_points(&player)
    }).reduce(|a, b| a + b).unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.trim().split("\n").map(|line| {
        let (opp_char, result_char) = line.split(" ")
            .map(|c| c.chars().next().expect("Empty string"))
            .into_iter().collect_tuple().unwrap();
        let opp_hand = char_to_hand(opp_char);
        let state_needed = char_to_state(result_char);
        let player_hand = Hand::VALUES.iter().find(|possible| {
            player_win(&opp_hand, *possible) == state_needed
        }).unwrap();
        state_to_points(&state_needed) + hand_to_points(player_hand)
    }).reduce(|a, b| a + b).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
