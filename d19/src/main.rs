use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let scans = load_scans(include_str!("../input"));
    let (a1, a2) = solve_it(&scans);
    println!("Answer part 1: {}", a1);
    println!("Answer part 2: {}", a2);
}

fn rotate(pt_in: (isize, isize, isize), orientation: usize) -> (isize, isize, isize){
    let (x, y, z) = pt_in;
    match orientation {
        0 => (x, y, z),
        1 => (-y, x, z),
        2 => (-x, -y, z),
        3 => (y, -x, z),
        
        4 => (-z, y, x),
        5 => (-y, -z, x),
        6 => (z, -y, x),
        7 => (y, z, x),

        8 => (-x, y, -z),
        9 => (-y, -x, -z),
        10 => (x, -y, -z),
        11 => (y, x, -z),
        
        12 => (z, y, -x),
        13 => (-y, z, -x),
        14 => (-z, -y, -x),
        15 => (y, -z, -x),
        
        16 => (x, z, -y),
        17 => (-z, x, -y),
        18 => (-x, -z, -y),
        19 => (z, -x, -y),
        
        20 => (x, -z, y),
        21 => (z, x, y),
        22 => (-x, z, y),
        23 => (-z, -x, y),
        _ => panic!("Invalid turn")
    }
}

fn rotate_set(in_set: &HashSet<(isize, isize, isize)>, orientation: usize) -> HashSet<(isize, isize, isize)> {
    in_set.iter().map(|pt| rotate(*pt, orientation)).collect()
}

fn translate(pt_in: (isize, isize, isize), delta: (isize, isize, isize)) -> (isize, isize, isize) {
    (pt_in.0 + delta.0, pt_in.1 + delta.1, pt_in.2 + delta.2)
}

fn translate_set(in_set: &HashSet<(isize, isize, isize)>, delta: (isize, isize, isize)) -> HashSet<(isize, isize, isize)> {
    in_set.iter().map(|pt| translate(*pt, delta)).collect()
}

fn load_scans(input: &str) -> Vec<HashSet<(isize, isize, isize)>> {
    input.split("\n\n").map(
        |s| {
            let mut it = s.lines();
            it.next();
            it.map(
                |l| {
                    let vec: Vec<_> = l.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
                    (vec[0], vec[1], vec[2])
                }
            ).collect::<HashSet<_>>()
        }
    ).collect()
}

fn find_12_intersection(set_1: &HashSet<(isize, isize, isize)>, set_2: &HashSet<(isize, isize, isize)>) -> Option<(HashSet<(isize, isize, isize)>, HashSet<(isize, isize, isize)>, (isize, isize, isize), usize)> {
    let set_1_x: HashSet<_> = set_1.iter().map(|(x, _, _)| *x).collect();
    let set_1_xy: HashSet<_> = set_1.iter().map(|(x, y, _)| (*x, *y)).collect();
    for orientation in 0..24 {
        let rotated = rotate_set(&set_2, orientation);
        for xd in -8000..8000 {
            let translated = translate_set(&rotated, (xd, 0, 0));
            let translated_x: Vec<_> = translated.iter().map(|(x, _, _)| *x).collect();
            if translated_x.iter().filter(|tx| set_1_x.contains(tx)).count() < 12 {
                continue
            }
            for yd in -8000..8000 {
                let translated = translate_set(&rotated, (xd, yd, 0));
                let translated_xy: Vec<_> = translated.iter().map(|(x, y, _)| (*x, *y)).collect();
                if translated_xy.iter().filter(|txy| set_1_xy.contains(txy)).count() < 12 {
                    continue
                }
                for zd in -8000..8000 {
                    let translated = translate_set(&rotated, (xd, yd, zd));
                    let intersecting: HashSet<_> = set_1.intersection(&translated).map(|pt| pt.clone()).collect();
                    if intersecting.len() >= 12 {
                        return Some((intersecting, translated, (xd, yd, zd), orientation))
                    }
                }
            }
        }
    }
    None
}

fn manhattan_distance(pt1: (isize, isize, isize), pt2: (isize, isize, isize)) -> usize {
    ((pt1.0 - pt2.0).abs() + (pt1.1 - pt2.1).abs() + (pt1.2 - pt2.2).abs()) as usize 
}

fn solve_it(scans: &Vec<HashSet<(isize, isize, isize)>>) -> (usize, usize) {
    let mut scans = scans.clone();
    let first_scan = scans.remove(0);
    let mut paired_scans = vec!((first_scan.clone(), first_scan.clone(), (0, 0, 0), 0));
    loop {
        let prev_scans_len = scans.len();
        println!("Number of scans: {}", scans.len());
        for ind in 0..(scans.len()) {
            let mut did_pair = false;
            for jnd in 0..(paired_scans.len()) {
                let p_scan = &paired_scans[jnd].0;
                if let Some((_intersection, transformed, delta, orientation)) = find_12_intersection(p_scan, &scans[ind]) {
                    let _paired = scans.remove(ind);
                    paired_scans.push((transformed, p_scan.clone(), delta, orientation));
                    did_pair = true;
                    break;
                }                            
            }
            if did_pair {
                break
            }
        }
        if scans.len() == 0 {
            break
        }
        if scans.len() == prev_scans_len {
            panic!("Could not pair.")
        }
    }
    let mut merged: HashSet<(isize, isize, isize)> = HashSet::new();
    let scanner_positions: Vec<_> = paired_scans.iter().map(|(_, _, pos, _)| pos.clone()).collect();
    let max_dist = scanner_positions.iter().combinations(2).map(|pos_pair| manhattan_distance(*pos_pair[0], *pos_pair[1])).max().unwrap();
    for ps in paired_scans {
        merged = merged.union(&ps.0).map(|pt| pt.clone()).collect();
    }

    (merged.len(), max_dist)
}
