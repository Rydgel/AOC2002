use std::collections::HashSet;
use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .flat_map(|(a, b)| common(a, b))
        .map(get_priority)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| get_priority(find_badge(chunk.collect_vec())))
        .sum()
}

fn common(a: &str, b: &str) -> Vec<char> {
    let first_set: HashSet<char> = HashSet::from_iter(a.chars());
    let second_set: HashSet<char> = HashSet::from_iter(b.chars());
    first_set.intersection(&second_set).copied().collect_vec()
}

fn find_badge(v: Vec<&str>) -> char {
    let mut item = '*';
    let mut missed;

    for character in v[0].chars() {
        missed = false;
        for line in &v {
            if !line.contains(character) {
                missed = true;
                break;
            }
        }
        if !missed {
            item = character;
            break;
        }
    }
    item
}

fn get_priority(c: char) -> usize {
    if c.is_lowercase() {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

fn main() {
    println!("day3 p1: {:?}", part1(INPUT));
    println!("day3 p2: {:?}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input = TEST_INPUT;
        let result = part1(input);
        assert_eq!(result, 157);
    }

    #[test]
    fn p2() {
        let input = TEST_INPUT;
        let result = part2(input);
        assert_eq!(result, 70);
    }
}
