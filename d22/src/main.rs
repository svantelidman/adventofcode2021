use itertools::Itertools;

#[derive(Clone, Debug)]
struct Cube {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
    zmin: isize,
    zmax: isize,
}

fn parse_input(s: &str) -> Vec<(bool, Cube)> {
    s.lines().map(|line| {
        let mut it = line.split(' ');
        let on = it.next().unwrap() == "on";
        let coord_specs = it.next().unwrap();
        let mut coord_it = coord_specs.split(&[',', '.', '=', 'x', 'y', 'z'][..]).filter(|s| s.len() > 0).map(|coord|
            coord.parse::<isize>().unwrap()
        );
        let cube = Cube {
            xmin: coord_it.next().unwrap(),
            xmax: coord_it.next().unwrap(),
            ymin: coord_it.next().unwrap(),
            ymax: coord_it.next().unwrap(),
            zmin: coord_it.next().unwrap(),
            zmax: coord_it.next().unwrap()
        };
        (on, cube)
    }).collect()
}

fn overlaps(min1: isize, max1: isize, min2: isize, max2: isize) -> bool {
    min2 >= min1 && min2 <= max1 || max2 >= min1 && max2 <= max1 ||
    min1 >= min2 && min1 <= max2 || max1 >= min2 && max1 <= max2
}

impl Cube {
    fn contains(&self, other: &Cube) -> bool {
        other.xmin >= self.xmin && other.xmax <= self.xmax &&
        other.ymin >= self.ymin && other.ymax <= self.ymax &&
        other.zmin >= self.zmin && other.zmax <= self.zmax
    }

    fn intersects(&self, other: &Cube) -> bool {
        let inter = overlaps(self.xmin, self.xmax, other.xmin, other.xmax) &&
        overlaps(self.ymin, self.ymax, other.ymin, other.ymax) &&
        overlaps(self.zmin, self.zmax, other.zmin, other.zmax);
        inter
    }

    fn merge (&self, set: Vec<Cube>) -> Vec<Cube>{
        if set.iter().any(|member| member.contains(self)) {
            set
        } else {
            let set = set.into_iter().filter(|c| !self.contains(c));
            let (intersecting, mut set): (Vec<Cube>, Vec<Cube>) = set.into_iter().partition(|p| self.intersects(p));
            let mut diff_set: Vec<_> = intersecting.into_iter().map(|p| p.difference(self)).flatten().collect();
            set.append(&mut diff_set);
            set.push(self.clone());
            set
        }
    }

    fn difference(&self, other: &Cube) -> Vec<Cube> {
        if other.contains(&self) {
            vec!()
        } else if !other.intersects(&self) {
            vec!(self.clone())
        } else {
            let parts = self.split(&other);
            let parts: Vec<_> = parts.into_iter().filter(|part| !other.contains(&part)).collect();
            parts
        }
    }

    fn split(&self, wedge: &Cube) -> Vec<Cube> {
        fn split_segments(omin: isize, omax: isize, wmin: isize, wmax: isize) -> Vec<(isize, isize)> {
            if wmin > omax || wmax < omin {
                vec!((omin, omax))
            } else {
                let wmin = wmin.max(omin);
                let wmax = wmax.min(omax);  
                let mut segs = vec!();
                if omin < wmin {
                    segs.push((omin, wmin - 1))
                }
                segs.push((wmin, wmax));
                if omax > wmax {
                    segs.push((wmax + 1, omax))
                }
                segs
            }
        }
        let x_segments = split_segments(self.xmin, self.xmax, wedge.xmin, wedge.xmax);
        let y_segments = split_segments(self.ymin, self.ymax, wedge.ymin, wedge.ymax);
        let z_segments = split_segments(self.zmin, self.zmax, wedge.zmin, wedge.zmax);
        let mut parts: Vec<Cube> = vec!();
        for x_seg in &x_segments {
            for y_seg in &y_segments {
                for z_seg in &z_segments {
                    parts.push(
                        Cube {
                            xmin: x_seg.0,
                            xmax: x_seg.1,
                            ymin: y_seg.0,
                            ymax: y_seg.1,
                            zmin: z_seg.0,
                            zmax: z_seg.1,
                        }
                    )
                }
            }
        }
        parts
    }
    
    fn volume(&self) -> usize {
        ((self.xmax - self.xmin + 1) * 
         (self.ymax - self.ymin + 1) * 
         (self.zmax - self.zmin + 1) ) as usize
    }
}

fn reboot(cubes: &Vec<(bool, Cube)>) -> Vec<Cube> {
    let mut it = cubes.iter();
    let first_cube = it.next().unwrap().1.clone();
    let mut merged = vec!(first_cube);
    for (on, cube) in it {
        if *on {
            merged = cube.merge(merged)
        } else {
            merged = merged.iter().map(|c| c.difference(cube)).flatten().collect()
        }
    }
    merged
}

fn main() {
    let cubes = parse_input(include_str!("../input"));
    let merged = reboot(&cubes);
    println!("Answer part 2: {}", merged.iter().map(|c| c.volume()).sum::<usize>());
}

mod test {
    use super::*;

    #[test]
    fn diff() {
        let c1 = Cube{
            xmin: 0,
            ymin: 0,
            zmin: 0,
            xmax: 2,
            ymax: 2,
            zmax: 2,
        };
        let c2 = Cube{
            xmin: 1,
            ymin: 1,
            zmin: 1,
            xmax: 1,
            ymax: 1,
            zmax: 1,
        };

        assert_eq!(
            c1.volume(),
            27
        );
        assert_eq!(
            c2.volume(),
            1
        );
        let diff = c1.difference(&c2);
        println!("The diff {:#?}", diff);
        assert_eq!(
            diff.iter().map(|d| d.volume()).sum::<usize>(),
            26
        )
    }

    #[test]
    fn t5() {
        let cubes = parse_input(include_str!("../test5"));
        let merged = reboot(&cubes);
        assert_eq!(merged.iter().map(|c| c.volume()).sum::<usize>(), 590784);
    }
}