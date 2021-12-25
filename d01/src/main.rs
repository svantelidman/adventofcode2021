fn count_increases(meas: &Vec<usize>) -> usize {
    (1..meas.len()).filter(|i| meas[*i] > meas[*i-1]).count()
}

fn count_sliding(meas: &Vec<usize>) -> usize {
    (3..meas.len()).filter(
        |i| {
            let r_lo = (i - 3)..=(i-1);
            let r_up = (i - 2)..=*i;
            let sum_lo =  r_lo.map(|ind| meas[ind]).sum::<usize>();
            let sum_up =  r_up.map(|ind| meas[ind]).sum::<usize>();
            sum_up > sum_lo
        } 
    ).count()
}

fn main() {
    let meas: Vec<usize> = include_str!("../input").split('\n').map(|s| s.parse().unwrap()).collect();
    println!("Answer part 1: {}", count_increases(&meas));
    println!("Answer part 2: {}", count_sliding(&meas))
}

mod test {
    use super::*;
    #[test]
    fn test_count() {
        let meas: Vec<usize> = include_str!("../test").split('\n').map(|s| s.parse().unwrap()).collect();
        assert_eq!(
            count_increases(&meas),
            7
        )
    }

    #[test]
    fn test_sliding() {
        let meas: Vec<usize> = include_str!("../test").split('\n').map(|s| s.parse().unwrap()).collect();
        assert_eq!(
            count_sliding(&meas),
            5
        )
    }
}