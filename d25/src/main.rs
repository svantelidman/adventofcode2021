fn parse_input(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

#[derive(PartialEq)]
enum Direction {
    East,
    South
}

fn get_move(row: usize, col: usize, map: &Vec<Vec<char>>, n_row: usize, n_col: usize, direction: &Direction) -> Option<(usize, usize)> {
    let (target_row, target_col) = match direction {
        Direction::East =>  (row, if col < n_col - 1 { col + 1 } else { 0 }),
        Direction::South => (if row < n_row - 1 { row + 1 } else { 0 }, col)
    };
    if map[target_row][target_col] == '.' {
        Some((target_row, target_col))
    } else {
        None
    }
}

fn update_map(map: &Vec<Vec<char>>, direction: Direction) -> (Vec<Vec<char>>, usize) {
    let n_row = map.len();
    let n_col = map[0].len();
    let mut new_map = map.clone();

    let mut n_moved = 0;
    for row in 0..n_row {
        for col in 0..n_col {
            if map[row][col] == '>' && direction == Direction::East || map[row][col] == 'v' && direction == Direction::South {
                if let Some((new_row, new_col)) = get_move(row, col, map, n_row, n_col, &direction) {
                    new_map[new_row][new_col] = map[row][col];
                    new_map[row][col] = '.';
                    n_moved += 1;
                } else {
                    new_map[row][col] = map[row][col];
                }    
            }
        }
    }
    (new_map, n_moved)
}

fn step(map: &Vec<Vec<char>>) -> (Vec<Vec<char>>, usize) {
    let (new_map, n_moved_east) = update_map(map, Direction::East);
    let (new_map, n_moved_south) = update_map(&new_map, Direction::South);
    (new_map, n_moved_east + n_moved_south)
}

fn part_1(map: &Vec<Vec<char>>) -> usize {
    let mut working_map = map.clone();
    let mut n_steps = 1;
    loop {
        let (new_working_map, n_moved) = step(&working_map);
        if n_moved == 0 {
            break;
        }
        working_map = new_working_map;
        n_steps += 1;
    }
    n_steps
}

fn main() {
    let map = parse_input(include_str!("../input"));
    println!("Answer part 1: {}", part_1(&map));
}

mod test {
    use super::*;

    #[test]
    fn p1() {
        let map = parse_input(include_str!("../test"));
        assert_eq!(
            part_1(&map), 58
        )
    }
}