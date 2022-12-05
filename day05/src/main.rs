use itertools::Itertools;
use reformation::Reformation;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct State {
    board: Vec<Vec<char>>,
}

impl State {
    pub fn new(board: Vec<Vec<char>>) -> Self {
        Self { board }
    }

    pub fn init() -> Self {
        State::new(vec![
            vec!['D', 'H', 'N', 'Q', 'T', 'W', 'V', 'B'],
            vec!['D', 'W', 'B'],
            vec!['T', 'S', 'Q', 'W', 'J', 'C'],
            vec!['F', 'J', 'R', 'N', 'Z', 'T', 'P'],
            vec!['G', 'P', 'V', 'J', 'M', 'S', 'T'],
            vec!['B', 'W', 'F', 'T', 'N'],
            vec!['B', 'L', 'D', 'Q', 'F', 'H', 'V', 'N'],
            vec!['H', 'P', 'F', 'R'],
            vec!['Z', 'S', 'M', 'B', 'L', 'N', 'P', 'H'],
        ])
    }

    pub fn play_move_part1(&mut self, a_move: &Move) {
        for _ in 0 .. a_move.number {
            let c = self.board[a_move.from - 1].pop();
            self.board[a_move.to - 1].push(c.expect("Empty"));
        }
    }

    pub fn play_move_part2(&mut self, a_move: &Move) {
        let amount = self.board[a_move.from - 1].len() - a_move.number..;
        let crates = self.board[a_move.from - 1].drain(amount).collect_vec();
        self.board[a_move.to - 1].extend(crates);
    }

    pub fn get_top_boxes(&self) -> String {
        self.board
            .iter()
            .map(|v| v[v.len() - 1])
            .collect()
    }
}

#[derive(Reformation, Debug)]
#[reformation(r"move {number} from {from} to {to}")]
struct Move {
    number: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|s| Move::parse(s).expect("bad input"))
        .collect_vec()
}

fn part1(state: &mut State, input: &[Move]) -> String {
    input
        .iter()
        .for_each(|m| state.play_move_part1(m));
    
    state.get_top_boxes()
}

fn part2(state: &mut State, input: &[Move]) -> String {
    input
        .iter()
        .for_each(|m| state.play_move_part2(m));
    
    state.get_top_boxes()
}

fn main() {
    let mut state = State::init();
    println!("day5 p1: {:?}", part1(&mut state, &parse_input(INPUT)));

    let mut state = State::init();
    println!("day5 p2: {:?}", part2(&mut state, &parse_input(INPUT)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let mut state = State::new(
            vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P'],
            ]
        );
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part1(&mut state, &input);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn p2() {
        let mut state = State::new(
            vec![
                vec!['Z', 'N'],
                vec!['M', 'C', 'D'],
                vec!['P'],
            ]
        );
        let input: Vec<_> = parse_input(TEST_INPUT);
        let result = part2(&mut state, &input);
        assert_eq!(result, "MCD");
    }
}
