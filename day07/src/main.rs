const INPUT: &str = include_str!("../input.txt");

fn directory_size(reader: &mut std::str::Lines<'_>, list: &mut Vec<usize>) -> usize {
    let mut size: usize = 0;

    loop {
        let line = reader.next();
        if line.is_none() { break }

        let line = line.unwrap();
        if line == "$ cd .." {
            break;
        }
        let words: Vec<&str> = line.split(' ').collect();
        if words[1] == "cd" {
            size += directory_size(reader, list);
        }
        size += words[0].parse::<usize>().unwrap_or(0);
    }
    list.push(size);

    size
}


fn part1(input: &str) -> usize {
    let mut reader = input.lines();
    let mut directories: Vec<usize> = Vec::new();
    directory_size(&mut reader, &mut directories);

    directories.iter()
        .fold(0, | acc, directory | {
            acc + if *directory <= 100000 { *directory } else { 0 }
        })
}

fn part2(input: &str) -> usize {
    let mut reader = input.lines();
    let mut directories: Vec<usize> = Vec::new();
    let total = directory_size(&mut reader, &mut directories);

    let upgrade_size = 30_000_000;
    let max_space = 70_000_000;
    let target_space = upgrade_size - (max_space - total);

    directories.iter()
        .fold(total, | acc, directory | {
            if *directory <= acc && *directory >= target_space { *directory } else { acc }
        })
}

fn main() {
    println!("day7 p1: {:?}", part1(INPUT));
    println!("day7 p2: {:?}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn p1() {
        let result = part1(TEST_INPUT);
        assert_eq!(result, 95_437);
    }

    #[test]
    fn p2() {
        let result = part2(TEST_INPUT);
        assert_eq!(result, 24_933_642);
    }
}
