use itertools::Itertools;

static VALID_POSITIONS: [(usize, usize); 27] = [
    (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10),
                    (1, 2), (2, 2), (3, 2), (4, 2),
                    (1, 4), (2, 4), (3, 4), (4, 4),
                    (1, 6), (2, 6), (3, 6), (4, 6),
                    (1, 8), (2, 8), (3, 8), (4, 8)
];

static SLOT_A: [(usize, usize); 4] = [(1, 2), (2, 2), (3, 2), (4, 2)];
static SLOT_B: [(usize, usize); 4] = [(1, 4), (2, 4), (3, 4), (4, 4)];
static SLOT_C: [(usize, usize); 4] = [(1, 6), (2, 6), (3, 6), (4, 6)];
static SLOT_D: [(usize, usize); 4] = [(1, 8), (2, 8), (3, 8), (4, 8)];
fn get_slot(kind: char) -> [(usize, usize);4] {
    match kind {
        'A' => SLOT_A,
        'B' => SLOT_B,
        'C' => SLOT_C,
        'D' => SLOT_D,
        _ => panic!("Illegal pod kind.") 
    }
}

static ENTRANCES: [(usize, usize); 4] = [(0, 2), (0, 4), (0, 6), (0, 8)];
fn get_entrance(pod: &Pod) -> (usize, usize) {
    match pod.kind {
        'A' => ENTRANCES[0],
        'B' => ENTRANCES[1],
        'C' => ENTRANCES[2],
        'D' => ENTRANCES[3],
        _ => panic!("Illegal pod kind.") 
    }
}

fn is_entrance(pos: (usize, usize)) -> bool {
    ENTRANCES.contains(&pos)
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Pod {
    kind: char,
    row: usize,
    col: usize,
    total_cost: usize
}

impl Pod {
    fn new(kind: char, row: usize, col: usize) -> Self {
        let total_cost = 0;
        Self {
            kind, row, col, total_cost
        }
    }

    fn state_without_cost(&self) -> (char, usize, usize) {
        (self.kind, self.row, self.col)
    }

    fn is_in_hallway(&self) -> bool {
        self.row == 0
    }    

    fn step_cost(&self) -> usize {
        match self.kind {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Illegal Pod kind.")
        }
    }

    fn clone_and_move(&self, new_pos: (usize, usize)) -> Pod {
        let n_steps = 
            if self.row == 0 || new_pos.0 == 0 {
                ((self.row as isize - new_pos.0 as isize).abs() + (self.col as isize - new_pos.1 as isize).abs()) as usize
            } else {
                self.row + new_pos.0 + (self.col as isize - new_pos.1 as isize).abs() as usize
            };
        let movement_cost = n_steps * self.step_cost();
        let mut pod = self.clone();
        pod.row = new_pos.0;
        pod.col = new_pos.1;
        pod.total_cost += movement_cost;
        pod
    }
}

fn is_empty(pos: (usize, usize), config: &Vec<Pod>) -> bool {
    !config.iter().any(|p| (p.row, p.col) == pos)
}

fn path_to_entrance_open(pod: &Pod, config: &Vec<Pod>) -> bool {
    let entrance = get_entrance(pod);
    let col_pod = pod.col;
    let col_entrance = entrance.1;
    is_path_from_pod_position_open(col_pod, col_entrance, config)
}

fn get_destination(pod: &Pod, config: &Vec<Pod>) -> Option<(usize, usize)> {
    let slot = get_slot(pod.kind);

    if path_to_entrance_open(pod, config) {
        if let Some(_first_empty) = slot.iter().find(|pos| is_empty(**pos, config)) {
            if let Some(first_occupied) = slot.iter().find(|pos| !is_empty(**pos, config)) {
                let first_pod = config.iter().find(|p| (p.row, p.col) == *first_occupied).unwrap();
                if is_pod_done(first_pod, config) {
                    Some((first_occupied.0 - 1, first_occupied.1))
                } else {
                    None
                }
            } else {
                Some(slot[slot.len()-1])
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn is_path_from_pod_position_open(pod_col: usize, col_other: usize, config: &Vec<Pod>) -> bool {
    let mut path_cols = if pod_col < col_other {
        (pod_col + 1)..col_other
    } else {
        (col_other)..pod_col
    };
    path_cols.all(|col| is_empty((0, col), config))
}


fn get_hallway_destinations(config: &Vec<Pod>, pod: &Pod) -> Vec<(usize, usize)> {
    VALID_POSITIONS.iter().filter(|pos| pos.0 == 0 && !is_entrance(**pos) && is_empty(**pos, config) && is_path_from_pod_position_open(pod.col, pos.1, config)).map(|pos| *pos).collect()
}

fn make_configuration(current_config: &Vec<Pod>, to_move: &Pod, destination: (usize, usize)) -> Vec<Pod> {
    let mut new_config: Vec<_> = current_config.iter().filter(|p| *p != to_move).map(|p| p.clone()).collect();
    new_config.push(
        to_move.clone_and_move(destination)
    );
    new_config
}

fn position_ok(pod: &Pod, config: &Vec<Pod>) -> bool {
    let slot = get_slot(pod.kind);
    slot.contains(&(pod.row, pod.col)) && {
        let start_row = pod.row + 1;
        let end_row = slot.len();
        (start_row..=end_row).all(|row| config.iter().any(|p| p.row == row && p.col == pod.col && p.kind == pod.kind))
    }
}

fn can_leave_slot(pod: &Pod, config: &Vec<Pod>) -> bool {
    if pod.row == 1 {
        return !position_ok(pod, config)
    }
    let start_row = 1;
    let end_row = pod.row;
    let all_empty = (start_row..end_row).all(|row| is_empty((row, pod.col), config));
    let pos_ok = position_ok(pod, config);
    let res = all_empty && !pos_ok;
    res
}

fn is_pod_done(pod: &Pod, config: &Vec<Pod>) -> bool {
    let slot = get_slot(pod.kind);
    if pod.col != slot[0].1 {
        return false
    } 
    let first_row = pod.row;
    let last_row = slot.len();
    (first_row..=last_row).all(|row| config.iter().any(|p| (p.row, p.col) == (row, pod.col) && p.kind == pod.kind)) 
}

fn is_config_done(path: &Vec<Pod>) -> bool {
    path.iter().all(|pod| is_pod_done(pod, path))
}

fn get_next_paths(path: &Vec<Pod>) -> Vec<Vec<Pod>> {
    let config = path;
    let non_done_pods: Vec<_> = config.iter().filter(|p| !is_pod_done(p, config)).collect();
    let mut next_configs: Vec<Vec<Pod>> = vec!();
    for pod in non_done_pods {
        if pod.is_in_hallway() {
            if let Some(dest) = get_destination(pod, config) {
                let next_conf = make_configuration(config, pod, dest);
                next_configs.push(next_conf)
            } 
        } else if can_leave_slot(pod, config) {
            if let Some(dest) = get_destination(pod, config) {
                let next_conf = make_configuration(config, pod, dest);
                next_configs.push(next_conf)
            }
            for dest in get_hallway_destinations(config, pod) {
                next_configs.push(make_configuration(config, pod, dest))
            }
        }
    }
    next_configs
}

fn find_minimal_cost(config: &Vec<Pod>) -> usize {
    let mut open_paths: Vec<Vec<Pod>> = vec!(config.clone());
    let mut done_paths: Vec<Vec<Pod>> = vec!();
    loop {
        let (mut new_done, new_open): (Vec<Vec<Pod>>, Vec<Vec<Pod>>) =  open_paths.iter().map(|path| get_next_paths(path)).flatten().partition(|path| is_config_done(path));
        done_paths.append(&mut new_done);
        let mut new_open_plus_state: Vec<_> = new_open.into_iter().map(|p| 
            {
                let mut config_state_without_cost: Vec<_> = p.iter().map(|pod| pod.state_without_cost()).collect();
                config_state_without_cost.sort();
                (p, config_state_without_cost)
            }
        ).collect();
        new_open_plus_state.sort_by_key(|(_, state)| state.clone());
        let mut unique_new_open_paths: Vec<Vec<Pod>> = vec!();
         for (_key, group) in &new_open_plus_state.into_iter().group_by(|(_, state)| state.clone()) {
            let mut same_state_paths: Vec<_> = group.into_iter().map(|(p, _)| p).collect();
            same_state_paths.sort_by_key(|path| path.iter().map(|pod| pod.total_cost).sum::<usize>());
            let selected_path = same_state_paths[0].clone();
            unique_new_open_paths.push(selected_path);
         }
         open_paths = unique_new_open_paths;
         println!("{}", open_paths.len());
        if open_paths.len() == 0 {
            break
        }
    }
    done_paths.iter().map(|path| path.iter().map(|pod| pod.total_cost).sum::<usize>()).min().unwrap()
}

fn main() {
    let start_config = 
    vec!(
        Pod::new('A', 1, 2),
        Pod::new('D', 2, 2),
        Pod::new('D', 3, 2),
        Pod::new('D', 4, 2),
        Pod::new('C', 1, 4),
        Pod::new('C', 2, 4),
        Pod::new('B', 3, 4),
        Pod::new('D', 4, 4),
        Pod::new('B', 1, 6),
        Pod::new('B', 2, 6),
        Pod::new('A', 3, 6),
        Pod::new('A', 4, 6),
        Pod::new('B', 1, 8),
        Pod::new('A', 2, 8),
        Pod::new('C', 3, 8),
        Pod::new('C', 4, 8),
    );
    println!("Answer part 2: {}", find_minimal_cost(&start_config));
}


fn test_config() -> Vec<Pod> {
    vec!(
        Pod::new('B', 1, 2),
        Pod::new('D', 2, 2),
        Pod::new('D', 3, 2),
        Pod::new('A', 4, 2),
        Pod::new('C', 1, 4),
        Pod::new('C', 2, 4),
        Pod::new('B', 3, 4),
        Pod::new('D', 4, 4),
        Pod::new('B', 1, 6),
        Pod::new('B', 2, 6),
        Pod::new('A', 3, 6),
        Pod::new('C', 4, 6),
        Pod::new('D', 1, 8),
        Pod::new('A', 2, 8),
        Pod::new('C', 3, 8),
        Pod::new('A', 4, 8),
    )
}

mod test {
    use super::*;
    #[test]
    fn p1() {
        assert_eq!(
            find_minimal_cost(&test_config()),
            44169
        )
    }
}
