fn main() {
    let crab_pos: Vec<usize> = include_str!("../input").split(',').map(|x| x.parse().unwrap()).collect();
    println!("Answer part 1: {}", part_1(&crab_pos));
    println!("Answer part 2: {}", part_2(&crab_pos))
}

fn part_1(crab_pos: &Vec<usize>) -> usize {
    let first_pos = *crab_pos.iter().min().unwrap();
    let last_pos = *crab_pos.iter().max().unwrap();
    let test_positions: Vec<_> = (first_pos..=last_pos).collect();
    
    let mut fuel_costs_by_ind: Vec<_> = test_positions.iter().map(
        |pos|
        (pos,
        crab_pos.iter().fold(0,
            |acc, cp| acc + (if cp > pos { cp - pos} else { pos - cp}))
        )
    ).collect();
    fuel_costs_by_ind.sort_by_key(|(_, cost)| cost.clone());
    fuel_costs_by_ind[0].1
}

fn part_2(crab_pos: &Vec<usize>) -> usize {
    let first_pos = *crab_pos.iter().min().unwrap();
    let last_pos = *crab_pos.iter().max().unwrap();
    let mut dist_cost: Vec<usize> = vec!();
    let mut acc = 0;
    for delta in 0..=(last_pos - first_pos) {
        acc = acc + delta;
        dist_cost.push(acc)
    }

    let test_positions: Vec<_> = (first_pos..=last_pos).collect();
    
    let mut fuel_costs_by_ind: Vec<_> = test_positions.iter().map(
        |pos|
        (pos,
        crab_pos.iter().fold(0,
            |acc, cp| {
                if cp > pos {
                    dist_cost[cp - pos] + acc
                } else {
                    dist_cost[pos - cp] + acc
                }
            })
        )
    ).collect();
    fuel_costs_by_ind.sort_by_key(|(_, cost)| cost.clone());
    fuel_costs_by_ind[0].1
}


mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let crab_pos: Vec<usize> = include_str!("../test").split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(
            part_1(&crab_pos),
            37
        )
    }

    #[test]
    fn test_part_2() {
        let crab_pos: Vec<usize> = include_str!("../test").split(',').map(|x| x.parse().unwrap()).collect();
        assert_eq!(
            part_2(&crab_pos),
            168
        )
    }
}
