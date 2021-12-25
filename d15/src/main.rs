use std::collections::HashMap;

fn main() {
    let scan = parse_input(include_str!("../input"));
    let scan = grow_scan(&scan);
    println!("Answer part 2: {}", calc_risc_score(&scan))
}

fn grow_scan(scan: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    fn big_val(val: usize, rg: usize, cg: usize) -> usize {
        let new_val = val + rg + cg;
        if new_val < 10 {
            new_val
        } else {
            new_val - 9
        }
    }
    let n_row = scan.len();
    let n_col = scan[0].len();
    let mut big_scan: Vec<Vec<usize>> = vec!();
    for rg in 0..5 {
        for r in 0..n_row {
            let mut big_row: Vec<usize> = vec!();
            for cg in 0..5 {
                for c in 0..n_col {
                    big_row.push(big_val(scan[r][c], rg, cg))
                }
            }
            big_scan.push(big_row)
        }
    }
    big_scan
}

fn parse_input(s: &str) -> Vec<Vec<usize>> {
    let mut rows: Vec<Vec<usize>> = vec!();
    for line in s.lines() {
        let row: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        rows.push(row)
    }
    rows
}

fn calc_risc_score(scan: &Vec<Vec<usize>>) -> usize {
    let n_row = scan.len();
    let n_col = scan[0].len();
    let current_row = 0;
    let current_col = 0;
    let all_scores = explore(current_row, current_col, &vec!(), &scan, n_row, n_col, (n_row - 1, n_col - 1), usize::MAX, &vec!(), &mut HashMap::new());
    all_scores.into_iter().min().unwrap()
}

fn next_positions(current_row: usize, current_col: usize, visited: &Vec<(usize, usize)>, n_row: usize, n_col: usize, scan: &Vec<Vec<usize>>, limit: usize, cheapest_path_so_far: &Vec<(usize, usize)>, end: (usize, usize)) -> Vec<(usize, usize)> {
    let deltas: Vec<(isize, isize)> = vec!((1, 0), (-1, 0), (0, 1), (0, -1));
    let mut next_positions: Vec<(usize, usize, isize, isize)> = deltas.into_iter().map(|(dr, dc)| 
        {
            let nr = current_row as isize + dr;
            let nc = current_col as isize + dc;
            (nr, nc, dr, dc)
        }
    ).filter(
        |(nr, nc, _, _)| *nr >= 0 && *nr < n_row as isize && *nc >= 0 && *nc < n_col as isize
    ).filter(|(nr, nc, _, _)|
        !visited.contains(&(*nr as usize, *nc as usize))
    ).filter(|(nr, nc, _, _)| {
        
        let score_so_far = visited.iter().map(|(r, c)| scan[*r][*c]).sum::<usize>() + scan[*nr as usize][*nc as usize];
        let min_remaining = if cheapest_path_so_far.contains(&(*nr as usize, *nc as usize)) {
            let inter_pos  = cheapest_path_so_far.iter().position(|(r, c)| *r == *nr as usize && *c == *nc as usize).unwrap();
            cheapest_path_so_far.iter().skip(inter_pos + 1).map(|(r, c)| scan[*r][*c]).sum::<usize>()
        } else {
            end.0 - *nr as usize + end.1 - *nc as usize
        };
        score_so_far + min_remaining < limit
    }
    ).map(|(nr, nc, dr, dc)| (nr as usize, nc as usize, dr, dc)).collect();
    next_positions.sort_by_key(
        |(r, c, dr, dc)|
            if (*r, *c) == end { 
                0 
            } else if *dr > 0 || *dc > 0 {
                scan[*r][*c]
            } else {
                scan[*r][*c] + 10
            }
    
    );
    next_positions.into_iter().map(|(r, c, _, _)| (r, c)).collect()
}

fn explore( current_row: usize, current_col:usize , visited: &Vec<(usize, usize)>, scan: &Vec<Vec<usize>>, n_row: usize, n_col: usize, end: (usize, usize), limit: usize, cheapest_path_so_far: &Vec<(usize, usize)>, cheapest_to_pos: &mut HashMap<(usize, usize), usize>) -> Vec<usize> {
    let next_positions = next_positions(current_row, current_col, visited, n_row, n_col, scan, limit, cheapest_path_so_far, end);
    let mut risc_scores: Vec<usize> = vec!();
    let mut cheapest_path_so_far = cheapest_path_so_far.clone();
    let mut limit = limit;
    for next_pos in next_positions {
        let mut visited = visited.clone();
        visited.push(next_pos);
        if next_pos == end {
            let score = visited.iter().map(|(r, c)| scan[*r][*c]).sum();
            if score < limit {
                limit = score;
                cheapest_path_so_far = visited;
            }
            println!("{}", score);
            risc_scores.push(score);
        } else {
            let score: usize = visited.iter().map(|(r, c)| scan[*r][*c]).sum();
            if let Some(old_score) = cheapest_to_pos.get(&next_pos) {
                if *old_score <= score {
                    continue
                } else {
                    cheapest_to_pos.insert(next_pos, score);
                }
            } else {
                cheapest_to_pos.insert(next_pos, score);
            }
            let (current_row, current_col) = next_pos;
            risc_scores.append(&mut explore(current_row, current_col, &visited, scan, n_row, n_col, end, limit, &cheapest_path_so_far, cheapest_to_pos));
            if let Some(sub_score) = risc_scores.iter().min() {
                limit = limit.min(*sub_score)
            }
        }
    }
    risc_scores
}

mod test {
    use super::*;
    #[test]
    fn p1() {
        let scan = parse_input(include_str!("../test"));
        assert_eq!(
            calc_risc_score(&scan),
            40
        )
    }

    #[test]
    fn p2() {
        let scan = parse_input(include_str!("../test"));
        let scan = grow_scan(&scan);
        assert_eq!(
            calc_risc_score(&scan),
            315
        )
    }

}
