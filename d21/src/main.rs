use itertools::Itertools;

struct Dice {
    next: usize,
    n_rolls: usize
}

impl Dice {
    fn new() -> Self {
        Self {next: 1, n_rolls: 0}
    }

    fn roll(&mut self) -> usize {
        let roll = self.next;
        self.next = if self.next < 100 {
            self.next + 1
        } else {
            1
        };  
        self.n_rolls +=1;
        roll
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct QuantumGame {
    n_instances: usize,
    roll_state: usize,
    roll_acc: usize,
    pos_1: usize,
    pos_2: usize,
    score_1: usize,
    score_2: usize,
    finished: bool
}

fn new_pos(current: usize, roll: usize) -> usize {
    (current + roll - 1) % 10 + 1
}

impl QuantumGame {
    fn new(start_1: usize, start_2: usize) -> Self {
        Self {
            n_instances: 1,
            roll_state: 0,
            roll_acc: 0,
            pos_1: start_1,
            pos_2: start_2,
            score_1: 0,
            score_2: 0,
            finished: false
        }
    }

    fn clone_empty(&self) -> Self {
        Self {
            n_instances: 0,
            roll_state: self.roll_state,
            roll_acc: self.roll_acc,
            pos_1: self.pos_1,
            pos_2: self.pos_2,
            score_1: self.score_1,
            score_2: self.score_2,
            finished: self.finished
        }
    }

    fn spawn(&self, dice_outcome: usize) -> Self {
        match self.roll_state {
            0 | 1 | 3 | 4 => { 
                Self {
                    n_instances: self.n_instances,
                    roll_state: self.roll_state + 1,
                    roll_acc: self.roll_acc + dice_outcome,
                    pos_1: self.pos_1,
                    pos_2: self.pos_2,
                    score_1: self.score_1,
                    score_2: self.score_2,
                    finished: self.finished
                }
            },
            2 => {
                let acc_dice = self.roll_acc + dice_outcome;
                let new_pos_1 = new_pos(self.pos_1, acc_dice);
                let new_score_1 = self.score_1 + new_pos_1;
                Self {
                    n_instances: self.n_instances,
                    roll_state: self.roll_state + 1,
                    roll_acc: 0,
                    pos_1: new_pos_1,
                    pos_2: self.pos_2,
                    score_1: new_score_1,
                    score_2: self.score_2,
                    finished: new_score_1 >= 21
                }
            },
            5 => {
                let acc_dice = self.roll_acc + dice_outcome;
                let new_pos_2 = new_pos(self.pos_2, acc_dice);
                let new_score_2 = self.score_2 + new_pos_2;
                Self {
                    n_instances: self.n_instances,
                    roll_state: 0,
                    roll_acc: 0,
                    pos_1: self.pos_1,
                    pos_2: new_pos_2,
                    score_1: self.score_1,
                    score_2: new_score_2,
                    finished: new_score_2 >= 21
                }
            },
            _ => panic!("Illegal roll state: {}", self.roll_state)
        }
    }

    fn split(&self) -> Vec<QuantumGame> {
        vec!(self.spawn(1), self.spawn(2), self.spawn(3))
    }
}

fn part_2(start_1: usize, start_2: usize)  -> usize {
    let mut ongoing_games = vec!(QuantumGame::new(start_1, start_2)); 
    let mut finished_games: Vec<QuantumGame> = vec!();
    while !ongoing_games.is_empty() {
        let (mut new_finished_games, mut new_ongoing_games): (Vec<QuantumGame>, Vec<QuantumGame>) =  ongoing_games.iter().map(|g| g.split()).flatten().into_iter().partition(|qg| qg.finished);
        new_ongoing_games.sort();
        new_ongoing_games = new_ongoing_games.into_iter().group_by(|qg| qg.clone_empty()).into_iter().map(|(k, v)| (k, v.map(|g| g.n_instances).sum::<usize>())).map(|(mut k, v)| {k.n_instances = v; k}).collect();
        ongoing_games = new_ongoing_games;
        finished_games.append(&mut new_finished_games);
    }
    let total_games = finished_games.iter().fold(0, |acc, g| acc + g.n_instances);
    let player_1_wins = finished_games.iter().filter(|g| g.score_1 >= 21).fold(0, |acc, g| acc + g.n_instances);
    let player_2_wins = total_games - player_1_wins;
    player_1_wins.max(player_2_wins)
}


fn part_1(start_1: usize, start_2: usize)  -> usize {
    let mut pos_1 = start_1;
    let mut pos_2 = start_2;
    let mut score_1 = 0;
    let mut score_2 = 0;
    let mut dice = Dice::new();
    loop {
        pos_1 = new_pos(pos_1, dice.roll() + dice.roll() + dice.roll());
        score_1 += pos_1;
        if score_1 >= 1000 {
            break
        }
        pos_2 = new_pos(pos_2, dice.roll() + dice.roll() + dice.roll());
        score_2 += pos_2;
        if score_2 >= 1000 {
            break
        }
    }
    dice.n_rolls * score_1.min(score_2)
}

fn main() {
    let (start_1, start_2) = (8, 3);
    println!("Answer part 1: {}", part_1(start_1, start_2));
    println!("Answer part 2: {}", part_2(start_1, start_2));
}

mod test {
    use super::*;

    #[test]
    fn p1() {
        let (start_1, start_2) = (4, 8);
        assert_eq!(
            part_1(start_1, start_2),
            739785
        )
    }

}
