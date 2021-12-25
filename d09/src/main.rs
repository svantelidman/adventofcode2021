use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<u32>>,
    n_rows: usize,
    n_cols: usize
}

impl Grid {
    fn new(s: &str) -> Grid {
        let rows: Vec<Vec<u32>> = s.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
        let n_rows = rows.len();
        let n_cols = rows[0].len();
        Grid{ rows, n_rows, n_cols }
    }

    fn get_adjacent(&self, irow: usize, icol: usize) -> Vec<(usize, usize)> {
        let mut adjacent: Vec<(usize, usize)> = vec!();
        if irow > 0 {
            adjacent.push((irow - 1, icol))
        }
        if icol > 0 {
            adjacent.push((irow, icol - 1))
        }
        if irow < self.n_rows - 1 {
            adjacent.push((irow + 1, icol))
        }
        if icol < self.n_cols - 1 {
            adjacent.push((irow, icol + 1))
        }
        adjacent
    }

    fn is_low_point(&self, irow: usize, icol: usize) -> bool {
        self.get_adjacent(irow, icol).iter().all(|(ar, ac)| self.rows[*ar][*ac] > self.rows[irow][icol])
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        let mut low_points: Vec<(usize, usize)> = vec!();
        for ir in 0..self.n_rows {
            for ic in 0..self.n_cols {
                if self.is_low_point(ir, ic) {
                    low_points.push((ir, ic))
                }
            }
        }
        low_points
    }

    fn risk_level(&self, ir: usize, ic: usize) -> u32 {
        self.rows[ir][ic] + 1
    }

    fn basin_size(&self, ir: usize, ic: usize) -> usize {
        let mut basin_members: HashSet<(usize, usize)> = [(ir, ic)].into_iter().collect();
        let mut newly_added: HashSet<(usize, usize)> = [(ir, ic)].into_iter().collect();
        while newly_added.len() > 0 {
            let mut new_newly_added: HashSet<(usize, usize)> = HashSet::new();
            for (nr, nc) in &newly_added {
                let new_adjacent: HashSet<(usize, usize)>  = self.get_adjacent(*nr, *nc).into_iter().filter(|(nr, nc)| !basin_members.contains(&(*nr, *nc)) && self.rows[*nr][*nc] < 9).collect();
                new_newly_added = new_newly_added.union(&new_adjacent).map(|l| *l).collect()
            }
            newly_added = new_newly_added;
            basin_members = basin_members.union(&newly_added).map(|l| *l).collect()
        }
        basin_members.len()
    }
}

fn part_2(grid: &Grid) -> usize {
    let low_points = grid.get_low_points();
    let mut basin_sizes: Vec<_> = low_points.iter().map(|(ir, ic)| grid.basin_size(*ir, *ic)).collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes.iter().take(3).product()
}

fn main() {
    let grid = Grid::new(include_str!("../input"));
    let low_points = grid.get_low_points();
    let a1 = low_points.iter().map(|(ir, ic)| grid.risk_level(*ir, *ic)).sum::<u32>();
    println!("Answer part 1: {}", a1);
    println!("Answer part 2: {}", part_2(&grid))
}

mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let grid = Grid::new(include_str!("../test"));
        let low_points = grid.get_low_points();
        assert_eq!(
            low_points.iter().map(|(ir, ic)| grid.risk_level(*ir, *ic)).sum::<u32>(),
            15
        )
    }

    #[test]
    fn test_part_2() {
        let grid = Grid::new(include_str!("../test"));
        let a2 = part_2(&grid);
        assert_eq!(
            a2,
            1134
        )
    }
}