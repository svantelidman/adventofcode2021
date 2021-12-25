fn main() {
    println!("Answer part 1: {}", find_best(150, 193, -136, -86));
    println!("Answer part 2: {}", find_all(150, 193, -136, -86));
}

fn hits_target(vxi: isize, vyi: isize, xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> Option<isize> {
    let mut vx = vxi;
    let mut vy = vyi;
    let mut x = 0;
    let mut y = 0;
    let mut ymax_height = 0;

    while x <= xmax && y >= ymin {
        if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
            return Some(ymax_height)
        }
        x += vx;
        y += vy;
        ymax_height = ymax_height.max(y);
        if vx > 0 {
            vx -= 1
        }
        vy -= 1;
    } 
    None
}

fn find_best(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> isize {
    let mut best = 0;
    for vxi in 0..=(xmax*2) {
        for vyi in ymin..(xmax*2) {
            if let Some(height) = hits_target(vxi, vyi, xmin, xmax, ymin, ymax) {
                best = best.max(height)
            }
        }
    }
    best
}

fn find_all(xmin: isize, xmax: isize, ymin: isize, ymax: isize) -> isize {
    let mut count = 0;
    for vxi in 0..=(xmax*2) {
        for vyi in ymin..(xmax*2) {
            if let Some(height) = hits_target(vxi, vyi, xmin, xmax, ymin, ymax) {
                count += 1
            }
        }
    }
    count
}

mod test {
    use super::*;

    #[test]
    fn hit_1() {
        assert_eq!(
            hits_target(7, 2, 20, 30, -10, -5),
            Some(3)
        )
    }

    #[test]
    fn hit_2() {
        assert_eq!(
            hits_target(6, 3, 20, 30, -10, -5),
            Some(6)
        )
    }

    #[test]
    fn hit_3() {
        assert_eq!(
            hits_target(9, 0, 20, 30, -10, -5),
            Some(0)
        )
    }

    #[test]
    fn hit_4() {
        assert_eq!(
            hits_target(17, -4, 20, 30, -10, -5),
            None
        )
    }

    #[test]
    fn hit_5() {
        assert_eq!(
            hits_target(6, 9, 20, 30, -10, -5),
            Some(45)
        )
    }

    #[test]
    fn test_best() {
        assert_eq!(
            find_best(20, 30, -10, -5),
            45
        )        
    }
    #[test]
    fn test_all() {
        assert_eq!(
            find_all(20, 30, -10, -5),
            112
        )        
    }
}
