fn main() {
    let (grid, key) = parse_input(include_str!("../input"));
    println!("Answer part 1: {}", part_1(&grid, &key, true));
    println!("Answer part 2: {}", part_2(&grid, &key, true));
}

fn parse_input(s: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut it = s.lines();
    let key: Vec<char> = it.next().unwrap().chars().map(|c|
        match c {
            '#' => '1',
            '.' => '0',
            _ => panic!("Unexpected char when parsing key {}", c)
        }
    ).collect();
    it.next();
    let grid: Vec<Vec<char>> = it.map(
        |line| {
            line.chars().map(|c|
                match c {
                    '#' => '1',
                    '.' => '0',
                    _ => panic!("Unexpected char when parsing grid {}", c)
                }
            ).collect()
        }
    ).collect();
    (grid, key)
}

fn transform_cell(grid: &Vec<Vec<char>>, n_row: isize, n_col: isize, r: isize, c: isize, key: &Vec<char>, lit_infinity: bool) -> char {
    let r_min = r  - 1;
    let r_max = r  + 1;
    let c_min = c  - 1;
    let c_max = c  + 1;
    let mut index_chars: Vec<char> = vec!();
    for r_ind in r_min..=r_max {
        for c_ind in c_min..=c_max {
            index_chars.push(
                if r_ind >= 0 && r_ind < n_row && c_ind >= 0 && c_ind < n_col {
                    grid[r_ind as usize][c_ind as usize]
                } else if lit_infinity {
                    '1'
                } else {
                    '0'
                }
            )
        }
    }
    let index_string = index_chars.into_iter().collect::<String>();
    let index = usize::from_str_radix(&index_string, 2).unwrap();
    key[index]
}

fn transform(grid: &Vec<Vec<char>>, key: &Vec<char>, lit_infinity: bool) -> Vec<Vec<char>> {
    let n_row = grid.len() as isize;
    let n_col = grid[0].len() as isize;
    let mut new_grid: Vec<Vec<char>> = vec!();
    for r in -1..=n_row {
        let mut row: Vec<char> = vec!();
        for c in -1..=n_col {
            row.push(transform_cell(grid, n_row, n_col, r, c, key, lit_infinity))
        }
        new_grid.push(row)
    }
    new_grid
}

fn part_1(grid: &Vec<Vec<char>>, key: &Vec<char>, alternating_infinity: bool) -> usize {
    let grid_1 = transform(grid, key, false);
    let grid_2 = transform(&grid_1, key, alternating_infinity);
    grid_2.iter().map(|r| r.iter().filter(|c| **c =='1').count()).sum()
}

fn part_2(grid: &Vec<Vec<char>>, key: &Vec<char>, alternating_infinity: bool) -> usize {
    let mut transformed: Vec<Vec<char>> = grid.clone();
    for _ in 0..25 {
        transformed = transform(&transformed, key, false);
        transformed = transform(&transformed, key, alternating_infinity);
    }
    transformed.iter().map(|r| r.iter().filter(|c| **c =='1').count()).sum()
}

mod test {
    use super::*;

    #[test]
    fn p1() {
        let (grid, key) = parse_input(include_str!("../test"));
        assert_eq!(
            part_1(&grid, &key, false),
            35
        )
    }

    #[test]
    fn p2() {
        let (grid, key) = parse_input(include_str!("../test"));
        assert_eq!(
            part_2(&grid, &key, false),
            3351
        )
    }

}