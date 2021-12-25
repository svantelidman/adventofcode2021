enum Direction {
    Forward,
    Up,
    Down
}

struct Instruction {
    dir: Direction,
    dist: usize
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let mut it = s.split(' ');
        let dir = it.next().unwrap();
        let dir = match dir {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Unknown direction {}", dir)
        };
        let dist = it.next().unwrap().parse().unwrap();
        Self { dir, dist}
    }
}

fn calc_pos(ins: &Vec<Instruction>) -> (usize, usize) {
    ins.iter().fold((0, 0), 
        |(x, y), ins| {
            match ins.dir {
                Direction::Forward => (x + ins.dist, y),
                Direction::Up => (x, y - ins.dist),
                Direction::Down => (x, y + ins.dist),
            }
        }
    )
}

fn calc_pos_2(ins: &Vec<Instruction>) -> (usize, usize, usize) {
    ins.iter().fold((0, 0, 0), 
        |(x, y, aim), ins| {
            match ins.dir {
                Direction::Forward => (x + ins.dist, y + aim * ins.dist, aim),
                Direction::Up => (x, y, aim - ins.dist),
                Direction::Down => (x, y, aim + ins.dist),
            }
        }
    )
}

fn main() {
    let instructions = parse_instructions(include_str!("../input"));
    let pos = calc_pos(&instructions);
    println!("Answer part 1: {}", pos.0 * pos.1);
    let pos_2 = calc_pos_2(&instructions);
    println!("Answer part 2: {}", pos_2.0 * pos_2.1)
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.split('\n').map(|s| Instruction::from_str(s)).collect()
}

mod test {
    use super::*;

    #[test]
    fn  test_part_1() {
        let instructions = parse_instructions(include_str!("../test"));
        assert_eq!(
            calc_pos(&instructions),
            (15, 10)
        );
    }

    #[test]
    fn  test_part_2() {
        let instructions = parse_instructions(include_str!("../test"));
        assert_eq!(
            calc_pos_2(&instructions),
            (15, 60, 10)
        );
    }
}