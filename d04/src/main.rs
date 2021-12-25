#[derive(Clone)]
struct Cell {
    num: usize,
    drawn: bool
}

#[derive(Clone)]
struct Board {
    rows: Vec<Vec<Cell>>
}

impl Board {
    fn mark(&mut self, num: usize) {
        let rows = self.rows.clone();
        self.rows = rows.into_iter().map(|r| r.into_iter().map(|mut c| { if c.num == num { c.drawn = true} c}).collect()).collect();
    }

    fn is_winner(&self) -> bool {
        for row in &self.rows {
            if row.iter().all(|c| c.drawn) {
                return true
            }
        }
        for col in 0..5 {
            let mut all_drawn = true;
            for row in &self.rows {
                if !row[col].drawn {
                    all_drawn = false;
                    break
                }
            }
            if all_drawn {
                return true
            }
        }
        false
    }

    fn score(&self, last_pick: usize) -> usize {
        self.rows.iter().flatten().fold(0, |acc, cell| if cell.drawn { acc } else { acc + cell.num}) * last_pick
    }
}

fn parse_input(s: &str) -> (Vec<usize>, Vec<Board>) {
    let mut it = s.lines();
    let num_line = it.next().unwrap();
    let nums = num_line.split(',').map(|n| n.parse().unwrap()).collect();
    let mut boards = vec!();
    loop {
        let blank = it.next();
        if let Some(_) = blank {
            let mut rows = vec!();
            for _ in 0..5 {
                let row = it.next().unwrap();
                let row: Vec<Cell> = row.split_whitespace().map(|n| n.parse::<usize>().unwrap()).map(|n| Cell {num: n, drawn: false}).collect();
                rows.push(row);
            }
            boards.push(Board{rows})
        } else {
            break
        }
    }
    (nums, boards)
}

fn part_1(nums: &Vec<usize>, boards: &Vec<Board>) {
    let mut boards = boards.clone();
    let mut nums = nums.iter();
    loop {
        if let Some(pick) = nums.next() {
            for board in &mut boards {
                board.mark(*pick)
            }
            if let Some(wb) = boards.iter().find(|b| b.is_winner())  {
                println!("Answer part 1: {}", wb.score(*pick));
                break
            }
        } else {
            panic!("Out of numbers.")
        }
    }
}

fn part_2(nums: &Vec<usize>, boards: &Vec<Board>) {
    let mut boards = boards.clone();
    let mut nums = nums.iter();
    loop {
        if let Some(pick) = nums.next() {
            let non_winner = boards.iter().position(|b| !b.is_winner()).unwrap();
            for board in &mut boards {
                board.mark(*pick)
            }
            let non_winner = &boards[non_winner];
            if boards.iter().all(|b| b.is_winner())  {
                println!("Answer part 2: {}", non_winner.score(*pick));
                break
            }
        } else {
            panic!("Out of numbers.")
        }
    }
}

fn main() {
    let (nums, boards) = parse_input(include_str!("../input"));
    part_1(&nums, &boards);
    part_2(&nums, &boards)
}

