fn main() {
    let inp = parse_input(include_str!("../input"));
    println!("Answer part 1: {}", part_1(&inp));
    println!("Answer part 2: {}", part_2(&inp));
}

fn parse_input(s: &str) -> Vec<String> {
    s.lines().map(|l| String::from(l)).collect()
}

fn error_score(s: &str) -> Option<usize> {
    let mut stack: Vec<char> = vec!(); 
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => if stack.pop().unwrap() != '(' { return Some(3) }
            ']' => if stack.pop().unwrap() != '[' { return Some(57) }
            '}' => if stack.pop().unwrap() != '{' { return Some(1197) }
            '>' => if stack.pop().unwrap() != '<' { return Some(25137) }
            _ => panic!("Invalid character in input {}", c as u8) 
        }
    }
    None
}
fn completion_score(s: &str) -> Option<usize> {
    let mut stack: Vec<char> = vec!(); 
    for c in s.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => if stack.pop().unwrap() != '(' { return None }
            ']' => if stack.pop().unwrap() != '[' { return None }
            '}' => if stack.pop().unwrap() != '{' { return None }
            '>' => if stack.pop().unwrap() != '<' { return None }
            _ => panic!("Invalid character in input {}", c as u8) 
        }
    }
    stack.reverse();
    let completion_score = stack.iter().fold(0,
        |acc, c| {
            acc * 5 +
            match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Invalid character in stack {}", c)
            }
        }
    );
    Some(completion_score)
}

fn part_2(input: &Vec<String>) -> usize {
    let mut scores: Vec<_> = input.iter().map(|s| completion_score(s)).filter(|score| if let Some(_) = score {true} else {false}).map(|score| if let Some(s) = score {s} else {panic!("Should not happen")}).collect();
    scores.sort();
    scores[scores.len()/2]
}

fn part_1(input: &Vec<String>) -> usize {
    input.iter().map(|s| error_score(s)).fold(0, |acc, score| if let Some(score) = score { acc + score } else { acc })
}

mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let inp = parse_input(include_str!("../test"));
        assert_eq!(
            part_1(&inp),
            26397
        )   

    }
    #[test]
    fn test_part_2() {
        let inp = parse_input(include_str!("../test"));
        assert_eq!(
            part_2(&inp),
            288957
        )   

    }
}
