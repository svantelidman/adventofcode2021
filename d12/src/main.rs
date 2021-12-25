use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(s: &str) -> HashMap<String, HashSet<String>> {
    fn insert_path(a: &String, b: &String, path_colls: &mut HashMap<String, HashSet<String>>) {
        if let Some(hs) = path_colls.get_mut(a) {
            hs.insert(b.clone());
        } else {
            let mut new_set: HashSet<String> = HashSet::new();
            new_set.insert(b.clone());
            path_colls.insert(a.clone(), new_set);
        }
    }

    let mut path_colls: HashMap<String, HashSet<String>> = HashMap::new();
    for line in s.lines() {
        let mut it = line.split("-");
        let a = String::from(it.next().unwrap());
        let b = String::from(it.next().unwrap());
        if &a != "end" && &b != "start" {
            insert_path(&a, &b, &mut path_colls);
        }
        if &a != "start" && &b != "end" {
            insert_path(&b, &a, &mut path_colls);
        }
    } 
    path_colls
}

fn main() {
    let path_colls = parse_input(include_str!("../input"));
    let paths = find_routes(&path_colls, false);
    println!("Answer part 1: {}", paths.len());
    let paths = find_routes(&path_colls, true);
    println!("Answer part 2: {}", paths.len());
}

fn find_routes(path_colls: &HashMap<String, HashSet<String>>, allow_multiple: bool) -> Vec<Vec<String>> {
    fn is_large_cave(c: &str) -> bool {
        c.chars().next().unwrap().is_uppercase()
    }

    let mut path_colls = path_colls.clone();
    if let Some(start_paths) = path_colls.remove("start") {
        let mut routes_so_far: Vec<_> = start_paths.into_iter().map(|end| vec!(String::from("start"), end)).collect();
        let mut complete_routes: Vec<Vec<String>> = vec!();
        loop {
            let mut new_routes_so_far: Vec<Vec<String>> = vec!();
            for route in &routes_so_far {
                let last_in_route = route.last().unwrap();
                let valid_next: Vec<_> = if allow_multiple {
                    let mut small_caves_in_path: Vec<_> = route.iter().filter(|cave| !is_large_cave(cave)).collect();
                    let n_small = small_caves_in_path.len();
                    small_caves_in_path.sort();
                    small_caves_in_path.dedup();
                    let n_small_unique = small_caves_in_path.len();
                    let n_small_dups = n_small - n_small_unique;
                    path_colls.get(last_in_route).unwrap().iter().filter(
                        |dest|
                         dest.chars().next().unwrap().is_uppercase() || n_small_dups == 0 || n_small_dups == 1 && !route.contains(dest)  || dest == &&String::from("end")
                    ).collect()
                } else {
                    path_colls.get(last_in_route).unwrap().iter().filter(|dest| is_large_cave(dest) || !route.contains(dest)).collect()
                };
                let new_routes: Vec<_> = valid_next.into_iter().map(|vn| { let mut nr = route.clone(); nr.push(vn.clone()); nr }).collect();
                for nr in new_routes {
                    if nr.last().unwrap() == "end" {
                        complete_routes.push(nr)
                    } else {
                        new_routes_so_far.push(nr);
                    }
                }
            }
            if new_routes_so_far.is_empty() {
                break
            }
            routes_so_far = new_routes_so_far
        }
        complete_routes

    } else {
        panic!("Could not find start.")
    }
}

mod test {
    use super::*;

    #[test]
    fn test_p1_1() {
        let path_colls = parse_input(include_str!("../test1"));
        let paths = find_routes(&path_colls, false);
        assert_eq!(
            paths.len(),
            10
        )
    }

    #[test]
    fn test_p1_2() {
        let path_colls = parse_input(include_str!("../test2"));
        let paths = find_routes(&path_colls, false);
        assert_eq!(
            paths.len(),
            19
        )
    }

    #[test]
    fn test_p1_3() {
        let path_colls = parse_input(include_str!("../test3"));
        let paths = find_routes(&path_colls, false);
        assert_eq!(
            paths.len(),
            226
        )
    }

    #[test]
    fn test_p2_1() {
        let path_colls = parse_input(include_str!("../test1"));
        let paths = find_routes(&path_colls, true);
        assert_eq!(
            paths.len(),
            36
        )
    }

    #[test]
    fn test_p2_2() {
        let path_colls = parse_input(include_str!("../test2"));
        let paths = find_routes(&path_colls, true);
        assert_eq!(
            paths.len(),
            103
        )
    }

    #[test]
    fn test_p2_3() {
        let path_colls = parse_input(include_str!("../test3"));
        let paths = find_routes(&path_colls, true);
        assert_eq!(
            paths.len(),
            3509
        )
    }

}