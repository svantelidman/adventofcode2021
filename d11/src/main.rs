#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<(u32, bool)>>,
    n_rows: usize,
    n_cols: usize
}

impl Grid {
    fn new(s: &str) -> Grid {
        let rows: Vec<Vec<(u32, bool)>> = s.lines().map(|s| s.chars().map(|c| (c.to_digit(10).unwrap(), false)).collect()).collect();
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
        if irow > 0 && icol > 0 {
            adjacent.push((irow - 1, icol - 1))
        }
        if irow > 0 && icol < self.n_cols - 1 {
            adjacent.push((irow - 1, icol + 1))
        }
        if irow < self.n_rows - 1 && icol > 0 {
            adjacent.push((irow + 1, icol - 1))
        }
        if irow < self.n_rows - 1 && icol < self.n_cols - 1 {
            adjacent.push((irow + 1, icol + 1))
        }
        adjacent 
    }

    fn tick(&mut self) -> usize {
        for ir in 0..self.n_rows {
            for ic in 0..self.n_rows {
                self.rows[ir][ic].0 += 1
            }
        }
        loop {
            let mut flashed = false;
            for ir in 0..self.n_rows {
                for ic in 0..self.n_rows {
                    if self.rows[ir][ic].0 > 9 && !self.rows[ir][ic].1 {
                        self.rows[ir][ic].1 = true;
                        flashed = true;
                        let adjacent = self.get_adjacent(ir, ic);
                        for (ir, ic) in adjacent {
                            self.rows[ir][ic].0 += 1;
                        }
                    }
                }
            }
            if !flashed {
                break
            }
        }

        let n_flash = self.rows.iter().flatten().filter(|(_, flashed)| *flashed).count();

        for ir in 0..self.n_rows {
            for ic in 0..self.n_rows {
                if self.rows[ir][ic].1 {
                    self.rows[ir][ic] = (0, false)
                }
            }
        }
        n_flash
    }
}

fn part_1(s: &str, n_steps: usize) -> usize{
    let mut grid = Grid::new(s);
    (0..n_steps).map(|_| grid.tick()).sum()
}

fn part_2(s: &str) -> usize{
    let mut grid = Grid::new(s);
    let n_cells = grid.n_rows * grid.n_cols;
    for n_tick in 1.. {
        if grid.tick() == n_cells {
            return n_tick
        }
    }
    0
}


fn main() {
    // println!("Answer part 1: {}", part_1(include_str!("../input"), 100))
    println!("Answer part 2: {}", part_2(include_str!("../input")))
}

mod test {
    use super::*;

    #[test]
    fn test_part_1_1() {
        assert_eq!(
            part_1(include_str!("../test1"), 1),
            9
        )
    }

    #[test]
    fn test_part_1_2() {
        assert_eq!(
            part_1(include_str!("../test2"), 100),
            1656
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(include_str!("../test2")),
            195
        )
    }
}
