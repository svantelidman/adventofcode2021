use std::collections::HashSet;

fn main() {
    let (points, folds) = parse_input(include_str!("../input"));
    println!("Answer part 1: {}", part_1(&points, &folds));
    println!("Answer part 2:");
    part_2(&points, &folds);
}

fn parse_input(s: &str) -> (HashSet<(usize, usize)>, Vec<(char, usize)>) {
    fn parse_point(l: &str) -> Result<(usize, usize), ()> {
        if l.len() > 0 {
            let mut it = l.split(',');
            let x: usize = it.next().unwrap().parse().unwrap();
            let y: usize = it.next().unwrap().parse().unwrap();
            Ok((x, y))
        } else {
            Err(())
        }
    }

    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut folds: Vec<(char, usize)> = vec!();
    let mut parsing_points = true;
    for l in s.lines() {
        if parsing_points {
            if let Ok(pt) = parse_point(l) {
                points.insert(pt);
            } else {
                parsing_points = false;
            }
        } else {
            if l.len() > 0 {
                let l = l.trim_start_matches("fold along ");
                let mut it = l.split('=');
                let axis = it.next().unwrap().chars().next().unwrap();
                let coord: usize = it.next().unwrap().parse().unwrap();
                folds.push((axis, coord))
            }
        }
    }
    (points, folds)
}

fn print_points(points: &HashSet<(usize, usize)>) {
    let max_x = points.iter().map(|(x, _y)| x).max().unwrap();
    let max_y = points.iter().map(|(_x, y)| y).max().unwrap();
    let mut p_row: Vec<char> = vec!();
    p_row.resize(max_x + 1, '.');
    let mut p_vec: Vec<Vec<char>> = vec!();
    p_vec.resize(max_y + 1, p_row);
    for (x, y) in points {
        p_vec[*y][*x] = '#'
    } 
    for row in p_vec {
        for c in row {
            print!("{}", c)
        }
        println!();
    }
    println!();

}

fn fold(points: &HashSet<(usize, usize)>, axis: char, coord: usize) -> HashSet<(usize, usize)> {
    let mut new_points: HashSet<(usize, usize)> = HashSet::new();
    for pt in points {
        match axis {
            'x' => {
                if pt.0 < coord {
                    new_points.insert(pt.clone());
                } else {
                    let new_x = 2 * coord - pt.0;
                    new_points.insert((new_x, pt.1));
                }
            }
            ,
            'y' => {
                if pt.1 < coord {
                    new_points.insert(pt.clone());
                } else {
                    let new_y = 2 * coord - pt.1;
                    new_points.insert((pt.0, new_y));
                }
            },
            _ => panic!("Illegal axis {}", axis)
        }
    }
    new_points
}

fn part_1(points: &HashSet<(usize, usize)>, folds: &Vec<(char, usize)>) -> usize {
    let new_points = fold(points, folds[0].0, folds[0].1);
    new_points.len()
}

fn part_2(points: &HashSet<(usize, usize)>, folds: &Vec<(char, usize)>) {
    let mut points = points.clone();
    for f in folds {
        points = fold(&points, f.0, f.1);
    }
    print_points(&points)
}

mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let (points, folds) = parse_input(include_str!("../test"));
        assert_eq!(
            part_1(&points, &folds),
            17
        )
    }
}