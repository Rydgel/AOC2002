use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn search_marker(input: &[char], size: usize) -> usize {
    input
        .windows(size)
        .enumerate()
        .find(|(_, substr)| substr.iter().all_unique())
        .map(|(i, _)| i + size)
        .expect("Unable to find a marker in this input")
}

fn main() {
    println!("day6 p1: {:?}", search_marker(&parse_input(INPUT), 4));
    println!("day6 p2: {:?}", search_marker(&parse_input(INPUT), 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let input = parse_input(TEST_INPUT);
        let result = search_marker(&input, 4);
        assert_eq!(result, 7);
    }

    #[test]
    fn p2() {
        let input = parse_input(TEST_INPUT);
        let result = search_marker(&input, 14);
        assert_eq!(result, 19);
    }
}
