use std::collections::HashMap;

fn main() {
    let mut fish = parse_input(include_str!("../input"));
    for _ in 0..256 {
        fish = spawn(fish)
    }
    println!("Answer part 2: {}", count_fish(fish));
}

fn parse_input(s: &str) -> HashMap<usize, usize> {
    let mut fish: HashMap<usize, usize> = HashMap::new();
    let fishv: Vec<usize> = s.split(',').map(|s| s.parse().unwrap()).collect();
    for f in fishv {
        if fish.contains_key(&f) {
            let n = fish.get(&f).unwrap();
            fish.insert(f, n + 1);
        } else {
            fish.insert(f, 1);
        }
    }
    fish
}

fn count_fish(fish: HashMap<usize, usize>) -> usize {
    fish.values().sum()
}

fn spawn(fish: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let n_new = if let Some(n) = fish.get(&0) {
        *n
    } else {
        0
    };
    let the_fish: Vec<(usize, usize)> = fish.into_iter().map(
        |(clock, n_fish)| {
            if clock == 0 {
                (6, n_fish)
            } else {
                (clock - 1, n_fish)
            }
        }
    ).collect();
    let mut new_fish: HashMap<usize, usize> = HashMap::new();
    for f in the_fish {
        if new_fish.contains_key(&f.0) {
            let n = new_fish.get(&f.0).unwrap();
            new_fish.insert(f.0, n + &f.1);
        } else {
            new_fish.insert(f.0, f.1);
        }

    }
    if n_new > 0 {
        new_fish.insert(8, n_new);
    }
    new_fish
}

mod test {
    use super::*;

    #[test]
    fn test_1_1() {
        let mut fish = parse_input(include_str!("../test"));
        for _ in 0..18 {
            fish = spawn(fish);
            println!("{:?}", fish);
        }
        assert_eq!(
            count_fish(fish),
            26
        )
    }

    #[test]
    fn test_1_2() {
        let mut fish = parse_input(include_str!("../test"));
        for _ in 0..256 {
            fish = spawn(fish)
        }
        assert_eq!(
            count_fish(fish),
            26984457539
        )
    }
}
