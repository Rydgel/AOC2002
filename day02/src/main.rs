use itertools::Itertools;
use reformation::Reformation;

const INPUT: &str = include_str!("../input.txt");

#[derive(Reformation, PartialEq, Copy, Clone, Debug)]
enum Plays {
    #[reformation(r"A|X")]
    Rock = 1,
    #[reformation(r"B|Y")]
    Paper = 2,
    #[reformation(r"C|Z")]
    Scissors = 3,
}

#[derive(Reformation, Debug)]
#[reformation(r"{a} {b}")]
struct Strategy {
    a: Plays,
    b: Plays,
}

fn choose_win(p: Plays) -> Plays {
    match p {
        Plays::Rock => Plays::Paper,
        Plays::Scissors => Plays::Rock,
        Plays::Paper => Plays::Scissors,
    }
}

fn choose_loss(p: Plays) -> Plays {
    match p {
        Plays::Paper => Plays::Rock,
        Plays::Scissors => Plays::Paper,
        Plays::Rock => Plays::Scissors,
    }
}

#[derive(Debug, Reformation, Copy, Clone)]
enum Outcome {
    #[reformation(r"X")]
    Loss = 0,
    #[reformation(r"Y")]
    Draw = 3,
    #[reformation(r"Z")]
    Win = 6,
}

#[derive(Reformation, Debug)]
#[reformation(r"{a} {b}")]
struct NewStrategy {
    a: Plays,
    b: Outcome,
}

fn play(s: &Strategy) -> Outcome {
    match s {
        Strategy { a, b } if a == b => Outcome::Draw,
        Strategy { a, b } if *a == choose_loss(*b) => Outcome::Win,
        _ => Outcome::Loss,
    }
}

fn parse_input_part1(input: &str) -> Vec<Strategy> {
    input
        .lines()
        .map(|s| Strategy::parse(s).expect("bad input"))
        .collect_vec()
}

fn part1(input: &[Strategy]) -> usize {
    input
        .iter()
        .map(|s| s.b as usize + play(s) as usize)
        .sum()
}

fn parse_input_part2(input: &str) -> Vec<NewStrategy> {
    input
        .lines()
        .map(|s| NewStrategy::parse(s).expect("bad input"))
        .collect_vec()
}

fn choose_input(s: &NewStrategy) -> Plays {
    match s {
        NewStrategy {a, b: Outcome::Draw } => *a,
        NewStrategy {a, b: Outcome::Win } => choose_win(*a),
        NewStrategy {a, b: Outcome::Loss } => choose_loss(*a),
    }
}

fn part2(input: &[NewStrategy]) -> usize {
    input
        .iter()
        .map(|s| choose_input(s) as usize + s.b as usize)
        .sum()
}

fn main() {
    println!("day2 p1: {:?}", part1(&parse_input_part1(INPUT)));
    println!("day2 p2: {:?}", part2(&parse_input_part2(INPUT)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input_part1(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input_part2(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 12);
    }
}
