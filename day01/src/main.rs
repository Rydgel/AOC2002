const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|s| parse_one_elf(s).iter().sum())
        .collect()
}

fn parse_one_elf(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect()
}

fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap_or(&0)
}

fn part2(input: &mut [usize]) -> usize {
    input.sort_by(|a, b| b.cmp(a));
    input.iter().take(3).sum()
}

fn main() {
    let mut input = parse_input(INPUT);
    println!("day1 p1: {:?}", part1(&input));
    println!("day1 p2: {:?}", part2(&mut input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&input);
        assert_eq!(result, 24_000);
    }

    #[test]
    fn p2() {
        let mut input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&mut input);
        assert_eq!(result, 45_000);
    }
}
