use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

fn parse_input(s: &str) -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    s.lines().map(
        |l| {
            let mut it = l.split(" | ");
            let p1 = it.next().unwrap();
            let p2 = it.next().unwrap();
            (
                p1.split_whitespace().map(|s| s.chars().collect()).collect(),
                p2.split_whitespace().map(|s| s.chars().collect()).collect(),
            )
        }
    ).collect()
}

fn part_1(observed: &Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    let valid_lengths = vec!(2, 3, 4, 7);
    observed.iter().fold(0,
        |acc, (_, digs)|
            acc + digs.iter().filter(|d| valid_lengths.contains(&d.len())).count()
    )
}

fn find_num(line: &(Vec<HashSet<char>>, Vec<HashSet<char>>)) -> usize {
    let segs_by_dig: Vec<(HashSet<char>, char)> = 
    [
        ("abcefg".chars().collect(), '0'),
        ("cf".chars().collect(), '1'),
        ("acdeg".chars().collect(), '2'),
        ("acdfg".chars().collect(), '3'),
        ("bcdf".chars().collect(), '4'),
        ("abdfg".chars().collect(), '5'),
        ("abdefg".chars().collect(), '6'),
        ("acf".chars().collect(), '7'),
        ("abcdefg".chars().collect(), '8'),
        ("abcdfg".chars().collect(), '9'),
        ].into_iter()
    .collect();
    let valid_segs: Vec<_> = segs_by_dig.iter().map(|(set, _)| set.clone()).collect();
    let (obs, scrambled) = line;

    let straight = vec!('a', 'b', 'c', 'd', 'e', 'f', 'g');
    for perm in vec!('a', 'b', 'c', 'd', 'e', 'f', 'g').into_iter().permutations(7) {
        let translation: HashMap<char, &char > = perm.into_iter().zip(&straight).collect();
        let translated_obs: Vec<_> = obs.iter().map(|o| o.iter().map(|c| **translation.get(c).unwrap()).collect::<HashSet<_>>()).collect();
        if translated_obs.iter().all(|to| valid_segs.contains(to)) {
            let num_string: String = scrambled.iter().map(
                |sc|
                {
                    let translated: HashSet<_> = sc.iter().map(|c| **translation.get(c).unwrap()).collect();
                    segs_by_dig.iter().find(|(seg, _)| translated.len() == seg.len() && translated.difference(seg).count() == 0).unwrap().1
                }
            ).collect();
            let num = num_string.parse().unwrap();
            return num;
        }
    }
    panic!("No translation found!!")
}

fn part_2(observed: &Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    observed.iter().map(|l| find_num(l)).sum()
}

fn main() {
    let input = parse_input(include_str!("../input"));
    println!("Answer part 1: {}", part_1(&input));
    println!("Answer part 2: {}", part_2(&input));
}

mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = parse_input(include_str!("../test2"));
        assert_eq!(
            part_1(&input),
            26
        );
    }

    #[test]
    fn test_2_0() {
        let input = parse_input("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        assert_eq!(
            part_2(&input),
            8394
        );
    }

    #[test]
    fn test_2_1() {
        let input = parse_input(include_str!("../test1"));
        assert_eq!(
            part_2(&input),
            5353
        );
    }

    #[test]
    fn test_2_2() {
        let input = parse_input(include_str!("../test2"));
        assert_eq!(
            part_2(&input),
            61229
        );
    }
}