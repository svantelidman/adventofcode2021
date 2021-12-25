use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let (poly, insertions) = parse_input(include_str!("../input"));
    println!("Answer part 1: {}", solve(&poly, &insertions, 10));
    println!("Answer part 2: {}", solve(&poly, &insertions, 40))
}

fn parse_input(s: &str) -> (String, HashMap<(char, char), char>) {
    let mut it = s.lines();
    let poly = String::from(it.next().unwrap());
    it.next();
    let insertions = it.map(|s| {
        let mut it = s.split(" -> ");
        let mut from = it.next().unwrap().chars();
        let from = (from.next().unwrap(), from.next().unwrap());
        let to = it.next().unwrap().chars().next().unwrap();
        (from, to)
    }).collect();
    (poly, insertions)
}

fn insert(poly: &String, insertions: &HashMap<(char, char), char>) -> String {
    let mut new_poly: Vec<char> = vec!();
    let mut it = poly.chars().peekable();
    loop {
        if let Some(c) = it.next() {
            new_poly.push(c);
            if let Some(next_c) = it.peek() {
                new_poly.push(*insertions.get(&(c, *next_c)).unwrap());
            } else {
                break
            };
        } else {
            break
        }
    }
    new_poly.into_iter().collect()
}

fn insert_n(poly: &String, insertions: &HashMap<(char, char), char>, n: usize) -> String {
    let mut poly = poly.clone();
    for _ in 0..n {
        poly = insert(&poly, insertions);
    }
    String::from(&poly[1..(poly.len()-1)])
}

fn pairify(s: &str) -> Vec<(char, char)> {
    let mut pairs: Vec<(char, char)> = vec!();
    let mut it = s.chars().peekable();
    loop {
        if let Some(c) = it.next() {
            if let Some(next_c) = it.peek() {
                pairs.push((c, *next_c));
            } else {
                break
            };
        } else {
            break
        }
    }
    pairs
}

fn get_expanded_counts_recursive(poly: &String, expansions: &HashMap<(char, char), (String, HashMap<char, usize>)>) -> HashMap<char, usize> {
    let poly_pairs = pairify(poly);
    let poly_vec: Vec<char> = poly.chars().collect();
    let mut all_pairs: Vec<(char, char)> = vec!();
    let mut poly_ind = 0;
    for this_pair in poly_pairs  {
        let (sub_str, _) = expansions.get(&this_pair).unwrap();
        let mut sub_pairs = pairify(&sub_str);
        all_pairs.push((this_pair.0, sub_pairs[0].0));
        let last_pair = (sub_pairs.last().unwrap().1, this_pair.1);
        all_pairs.append(&mut sub_pairs);
        all_pairs.push(last_pair);
    }

    let mut expanded_counts = HashMap::new();
    expanded_counts.insert(all_pairs[0].0, 1);
    for sp in all_pairs {
        let (_, sp_counts) = expansions.get(&sp).unwrap();
        for (c, count) in sp_counts {
            if let Some(x) = expanded_counts.get(c) {
                expanded_counts.insert(*c, x + count);
            } else {
                expanded_counts.insert(*c, *count);
            }
        }
        if let Some(x) = expanded_counts.get(&sp.1) {
            expanded_counts.insert(sp.1, x + 1);
        } else {
            expanded_counts.insert(sp.1, 1);
        }
    }
    expanded_counts
}

fn get_expansions(insertions: &HashMap<(char, char), char>, n_steps: usize) -> HashMap<(char, char), (String, HashMap<char, usize>)> {
    let mut all_present_chars: Vec<char> = insertions.iter().map(|((c1, c2), c3)| vec!(*c1, *c2, *c3)).flatten().collect();
    all_present_chars.sort();
    all_present_chars.dedup();
    let mut all_char_doubles: Vec<_> = all_present_chars.iter().map(|c| (*c, *c)).collect();
    let all_char_combs: Vec<_> = all_present_chars.iter().combinations(2).collect();
    let mut all_char_pairs: Vec<_> = all_char_combs.into_iter().map(|v| vec!((*v[0], *v[1]), (*v[1], *v[0]))).flatten().collect();
    all_char_pairs.append(&mut all_char_doubles);
    let expansions: HashMap<(char, char), (String, HashMap<char, usize>)> = 
    all_char_pairs.into_iter().map(
        |(c1, c2)| {
        let expansion =  insert_n( &String::from(format!("{}{}", c1, c2)) , insertions, n_steps);
        let mut exp_vec: Vec<char> = expansion.chars().collect();
        exp_vec.sort();
        let frequencies: HashMap<char, usize>  = exp_vec.into_iter().group_by(|c| *c).into_iter().map(|(k, v)| (k, v.count())).collect();
        ((c1, c2), (expansion, frequencies))
    }).collect();
    expansions
}

fn solve(poly: &String, insertions: &HashMap<(char, char), char>, n_steps: usize) -> usize {
    let expansions = get_expansions(insertions, n_steps/2);
    let mut poly_counts: Vec<_> =  get_expanded_counts_recursive(poly, &expansions).into_iter().collect();
    
    poly_counts.sort_by_key(|x| x.1);
    let poly_min = poly_counts.first().unwrap().1;
    let poly_max = poly_counts.last().unwrap().1;
    let poly_answer =  poly_max - poly_min;
    poly_answer
}

mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let (poly, insertions) = parse_input(include_str!("../test"));
        assert_eq!(
            solve(&poly, &insertions, 10),
            1588
        )
    }
    #[test]
    fn test_p2() {
        let (poly, insertions) = parse_input(include_str!("../test"));
        assert_eq!(
            solve(&poly, &insertions, 40),
            2188189693529
        )
    }
}