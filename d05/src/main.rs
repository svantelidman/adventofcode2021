#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize
}

fn parse_lines(s: &str) -> Vec<Line> {
    let lines: Vec<_> = s.lines().map(|l|
        {
            let parts: Vec<_> = l.split(" -> ").collect();
            let x1y1: Vec<usize> = parts[0].split(',').map(|c| c.parse().unwrap()).collect();
            let x2y2: Vec<usize> = parts[1].split(',').map(|c| c.parse().unwrap()).collect();
            Line {
                x1: x1y1[0],
                y1: x1y1[1],
                x2: x2y2[0],
                y2: x2y2[1]
            }
        }).collect();
    lines
}

fn mk_grid(lines: &Vec<Line>, part: usize) -> Vec<Vec<usize>> {
    let orto_lines: Vec<_> = if part == 1 {
        lines.iter().filter(|l| l.x1 == l.x2 || l.y1 == l.y2).collect()
    } else {
        lines.iter().collect()
    };
    let max_x = orto_lines.iter().map(|l| l.x1.max(l.x2)).max().unwrap();
    let max_y = orto_lines.iter().map(|l| l.y1.max(l.y2)).max().unwrap();
    let mut first_line: Vec<usize> = vec!();
    first_line.resize(max_x + 1, 0);
    let mut grid: Vec<Vec<usize>> = vec!();
    grid.resize(max_y + 1, first_line);
    for line in orto_lines {
        let mut x = line.x1 as isize;
        let mut y = line.y1 as isize;
        let xd = if line.x2 > line.x1 {
            1
        } else if line.x2 < line.x1 {
            -1
        } else {
            0
        };
        let yd = if line.y2 > line.y1 {
            1
        } else if line.y2 < line.y1 {
            -1
        } else {
            0
        };
        loop {
            grid[y as usize][x as usize] +=1;
            if x as usize == line.x2 && y as usize == line.y2 {
                break
            }
            x += xd;
            y += yd;

        }
    }
    grid
}

fn count_severe_danger(grid: &Vec<Vec<usize>>) -> usize {
    grid.iter().flatten().fold(0, |acc, n| if *n >= 2 { acc + 1 } else { acc } )
}

fn main() {
    let lines = parse_lines(include_str!("../input"));
    let grid = mk_grid(&lines, 1);
    println!("Answer part 1 : {}", count_severe_danger(&grid));
    let grid = mk_grid(&lines, 2);
    println!("Answer part 2 : {}", count_severe_danger(&grid));
}
