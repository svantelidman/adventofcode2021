fn main() {
    let diag_codes = parse_input(include_str!("../input"));
    let (gamma, epsilon) = calc_rates(&diag_codes);
    println!("Answer part 1: {}", gamma * epsilon);
    let (ox, co2) = calc_ox_co2(&diag_codes);
    println!("Answer part 2: {}", ox * co2);
}

fn parse_input(s:  &str) -> Vec<Vec<char>> {
    s.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect()
}

fn calc_rates(diag_codes: &Vec<Vec<char>>) -> (usize, usize) {
    let mut counts: Vec<usize> = vec!();
    counts.resize(diag_codes[0].len(), 0);
    let code_length = diag_codes[0].len();
    for pos in 0..code_length {
        for code in diag_codes {
            if code[pos] == '1' {
                counts[pos] += 1
            }
        }    
    }
    let n_codes = diag_codes.len();
    let mut gamma: Vec<char> = vec!();
    let mut epsilon: Vec<char> = vec!();
    for pos in 0..code_length {
        if counts[pos]*2 > n_codes {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    let gamma_str: String = gamma.iter().collect();
    let epsilon_str: String = epsilon.iter().collect();
    let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_str, 2).unwrap();
    (gamma, epsilon)
}

fn calc_ox_co2(diag_codes: &Vec<Vec<char>>) -> (usize, usize) {
    (calc_ox(&diag_codes), calc_co2(&diag_codes))
}

fn get_most_common(diag_codes: &Vec<Vec<char>>, pos: usize) -> char {
    let n_ones = diag_codes.iter().fold(0, |acc, code| if code[pos] == '1'{ acc + 1} else { acc });
    if n_ones * 2 >= diag_codes.len() {
        '1'
    } else {
        '0'
    }
}

fn get_least_common(diag_codes: &Vec<Vec<char>>, pos: usize) -> char {
    let n_ones = diag_codes.iter().fold(0, |acc, code| if code[pos] == '1'{ acc + 1} else { acc });
    if n_ones * 2 < diag_codes.len() {
        '1'
    } else {
        '0'
    }
}

fn filter_all(remaining: Vec<Vec<char>>, pos: usize, to_keep: char) -> Vec<Vec<char>> {
    remaining.into_iter().filter(|code| code[pos] == to_keep).collect()
}

fn calc_ox(diag_codes: &Vec<Vec<char>>) -> usize {
    let mut pos = 0;
    let mut remaining = diag_codes.clone();
    while remaining.len() > 1 {
        let most_common = get_most_common(&remaining, pos);
        remaining = filter_all(remaining, pos, most_common);
        pos +=1;
    }
    let ox: String = remaining[0].iter().collect();
    usize::from_str_radix(&ox, 2).unwrap()
}

fn calc_co2(diag_codes: &Vec<Vec<char>>) -> usize {
    let mut pos = 0;
    let mut remaining = diag_codes.clone();
    while remaining.len() > 1 {
        let least_common = get_least_common(&remaining, pos);
        remaining = filter_all(remaining, pos, least_common);
        pos +=1;
    }
    let co2: String = remaining[0].iter().collect();
    usize::from_str_radix(&co2, 2).unwrap()
}

mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let diag_codes = parse_input(include_str!("../test"));
        let (gamma, epsilon) = calc_rates(&diag_codes);
        assert_eq!(
            gamma * epsilon,
            198
        )
    }

    #[test]
    fn test_ox() {
        let diag_codes = parse_input(include_str!("../test"));
        let ox = calc_ox(&diag_codes);
        assert_eq!(ox, 23)
    }

    #[test]
    fn test_part_2() {
        let diag_codes = parse_input(include_str!("../test"));
        let (ox, co2) = calc_ox_co2(&diag_codes);
        assert_eq!(
            (ox, co2),
            (23, 10)
        )
    }

}