use itertools::Itertools;
use reformation::Reformation;

const INPUT: &str = include_str!("../input.txt");

#[derive(Reformation, Debug)]
#[reformation(r"{a}-{b},{c}-{d}")]
struct Assignements {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

fn parse_input(input: &str) -> Vec<Assignements> {
    input
        .lines()
        .map(|s| Assignements::parse(s).expect("bad input"))
        .collect_vec()
}


fn fully_contains(assign: &&Assignements) -> bool {
    (assign.a >= assign.c && assign.b <= assign.d) || (assign.c >= assign.a && assign.d <= assign.b)
}

fn pair_overlaps(assign: &&Assignements) -> bool {
    assign.a <= assign.d && assign.b >= assign.c
}

fn part1(input: &[Assignements]) -> usize {
    input
        .iter()
        .filter(fully_contains)
        .count()
}

fn part2(input: &[Assignements]) -> usize {
    input
        .iter()
        .filter(pair_overlaps)
        .count()
}

fn main() {
    println!("day4 p1: {:?}", part1(&parse_input(INPUT)));
    println!("day4 p2: {:?}", part2(&parse_input(INPUT)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&input);
        assert_eq!(result, 4);
    }
}
