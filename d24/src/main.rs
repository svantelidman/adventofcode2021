use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Operand {
    Int {val: isize},
    Register {register: char}
}

#[derive(Debug)]
enum Instruction{
    Inp {a: Operand},
    Add {a: Operand, b: Operand},
    Mul {a: Operand, b: Operand},
    Div {a: Operand, b: Operand},
    Mod {a: Operand, b: Operand},
    Eql {a: Operand, b: Operand}
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ALU {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
} 

fn parse_instruction(s: &str) -> Instruction {
    fn get_operand(words: &mut std::str::SplitWhitespace) -> Operand {
        let operand_word  = words.next().unwrap();
        match  operand_word {
            "w" | "x" | "y" | "z" => Operand::Register{ register: operand_word.chars().next().unwrap()},
            _                     => Operand::Int{ val: operand_word.parse::<isize>().unwrap()}
        }
    }
    let mut words = s.split_whitespace();
    let ins_word = words.next().unwrap();
    match ins_word {
        "inp" => Instruction::Inp{ a: get_operand(&mut words) }, 
        "add" => Instruction::Add{ a: get_operand(&mut words), b: get_operand(&mut words) }, 
        "mul" => Instruction::Mul{ a: get_operand(&mut words), b: get_operand(&mut words) }, 
        "div" => Instruction::Div{ a: get_operand(&mut words), b: get_operand(&mut words) }, 
        "mod" => Instruction::Mod{ a: get_operand(&mut words), b: get_operand(&mut words) }, 
        "eql" => Instruction::Eql{ a: get_operand(&mut words), b: get_operand(&mut words) },
        _ => panic!("Illegal instruction: {}", ins_word)
    }
}

fn parse_dig_progs(s: &str) -> Vec<Vec<Instruction>> {
    s.split("\n\n").map(|part| part.lines().map(|line| parse_instruction(line)).collect()).collect()
}

impl ALU {
    fn new() -> Self {
        Self{ w: 0, x: 0, y: 0, z: 0 }
    }

    fn get(&self, operand: Operand) -> isize {
        match operand {
            Operand::Register{ register } =>
                match register {
                    'w' => self.w,
                    'x' => self.x,
                    'y' => self.y,
                    'z' => self.z,
                    _  =>  panic!("Illegal register {}", register)
                },
            Operand::Int{ val } => val
        }
    }

    fn set(&mut self, operand: Operand, val: isize) {
        match operand {
            Operand::Register{ register } =>
                match register {
                    'w' => self.w = val,
                    'x' => self.x = val,
                    'y' => self.y = val,
                    'z' => self.z = val,
                    _  =>  panic!("Illegal register {}", register)
                },
            Operand::Int { val: _ } => panic!("Cannot set an integer literal.")
        }
    }

    fn process_instruction(&mut self, instruction: &Instruction, input_stack: &mut Vec<isize>) {
        match instruction {
            Instruction::Inp {a}    => self.set(*a, input_stack.pop().unwrap()),
            Instruction::Add {a, b} => self.set(*a, self.get(*a) + self.get(*b)),
            Instruction::Mul {a, b} => self.set(*a, self.get(*a) * self.get(*b)),
            Instruction::Div {a, b} => self.set(*a, self.get(*a) / self.get(*b)),
            Instruction::Mod {a, b} => self.set(*a, self.get(*a) % self.get(*b)),
            Instruction::Eql {a, b} => self.set(*a, if self.get(*a) == self.get(*b) { 1 } else {0} )
        }
    }
}

fn run_delta(prog: &Vec<Instruction>, alu: &ALU, input: isize) -> ALU {
    let mut alu = alu.clone();
    for instruction in prog {
        alu.process_instruction(&instruction, &mut vec!(input));
    }
    alu
}

fn part_1(dig_progs: &Vec<Vec<Instruction>>) -> isize {
    let mut memo_0: HashSet<ALU> =  HashSet::new();
    let mut memo_1: HashSet<ALU> =  HashSet::new();
    let mut memo_2: HashSet<ALU> =  HashSet::new();
    let mut memo_3: HashSet<ALU> =  HashSet::new();
    let mut memo_4: HashSet<ALU> =  HashSet::new();
    let mut memo_5: HashSet<ALU> =  HashSet::new();
    let mut memo_6: HashSet<ALU> =  HashSet::new();
    let mut memo_7: HashSet<ALU> =  HashSet::new();
    let mut memo_8: HashSet<ALU> =  HashSet::new();
    let mut memo_9: HashSet<ALU> =  HashSet::new();
    let mut memo_10: HashSet<ALU> =  HashSet::new();
    let mut memo_11: HashSet<ALU> =  HashSet::new();
    let mut memo_12: HashSet<ALU> =  HashSet::new();

    for ind0 in 1..=9 {
        let alu0 = run_delta(&dig_progs[0], &ALU::new(), ind0);
        if memo_0.contains(&alu0) {
            continue
        }
        memo_0.insert(alu0.clone());
        for ind1 in 1..=9 {
            let alu1 = run_delta(&dig_progs[1], &alu0, ind1);
            if memo_1.contains(&alu1) {
                continue
            }
            memo_1.insert(alu1.clone());
            for ind2 in 1..=9 {
                let alu2 = run_delta(&dig_progs[2], &alu1, ind2);
                if memo_2.contains(&alu2) {
                    continue
                }
                memo_2.insert(alu2.clone());
                for ind3 in 1..=9 {
                    let alu3 = run_delta(&dig_progs[3], &alu2, ind3);
                    if memo_3.contains(&alu3) {
                        continue
                    }
                    memo_3.insert(alu3.clone());
                    for ind4 in 1..=9 {
                        let alu4 = run_delta(&dig_progs[4], &alu3, ind4);
                        if memo_4.contains(&alu4) {
                            continue
                        }
                        memo_4.insert(alu4.clone());
                        for ind5 in 1..=9 {
                            let alu5 = run_delta(&dig_progs[5], &alu4, ind5);
                            if memo_5.contains(&alu5) {
                                continue
                            }
                            memo_5.insert(alu5.clone());
                            for ind6 in 1..=9 {
                                let alu6 = run_delta(&dig_progs[6], &alu5, ind6);
                                if memo_6.contains(&alu6) {
                                    continue
                                }
                                memo_6.insert(alu6.clone());

                                for ind7 in 1..=9 {
                                    let alu7 = run_delta(&dig_progs[7], &alu6, ind7);
                                    if memo_7.contains(&alu7) {
                                        continue
                                    }
                                    memo_7.insert(alu7.clone());
                                    for ind8 in 1..=9 {
                                        let alu8 = run_delta(&dig_progs[8], &alu7, ind8);
                                        if memo_8.contains(&alu8) {
                                            continue
                                        }
                                        memo_8.insert(alu8.clone());
                                        for ind9 in 1..=9 {
                                            let alu9 = run_delta(&dig_progs[9], &alu8, ind9);
                                            if memo_9.contains(&alu9) {
                                                continue
                                            }
                                            memo_9.insert(alu9.clone());
                                            for ind10 in 1..=9 {
                                                let alu10 = run_delta(&dig_progs[10], &alu9, ind10);
                                                if memo_10.contains(&alu10) {
                                                    continue
                                                }
                                                memo_10.insert(alu10.clone());
                                                for ind11 in 1..=9 {
                                                    let alu11 = run_delta(&dig_progs[11], &alu10, ind11);
                                                    if memo_11.contains(&alu11) {
                                                        continue
                                                    }
                                                    memo_11.insert(alu11.clone());
                                                    for ind12 in 1..=9 {
                                                        let alu12 = run_delta(&dig_progs[12], &alu11, ind12);
                                                        if memo_12.contains(&alu12) {
                                                            continue
                                                        }
                                                        memo_12.insert(alu12.clone());
                                                        for ind13 in 1..=9 {
                                                            let alu13 = run_delta(&dig_progs[13], &alu12, ind13);
                                                            if alu13.z == 0 {
                                                                return 
                                                                ind13 * 1 + 
                                                                ind12 * 10 + 
                                                                ind11 * 100 + 
                                                                ind10 * 1000 + 
                                                                ind9  * 10000 + 
                                                                ind8  * 100000 + 
                                                                ind7  * 1000000 + 
                                                                ind6  * 10000000 + 
                                                                ind5  * 100000000 + 
                                                                ind4  * 1000000000 + 
                                                                ind3  * 10000000000 + 
                                                                ind2  * 100000000000 + 
                                                                ind1  * 1000000000000 + 
                                                                ind0  * 10000000000000;  
                                                            }
                                                        }                       
                                                    }                       
                                                }            
                                            }            
                                        }            
                                    }            
                                }            
                            }            
                        }            
                    }            
                }        
            }
        }
    }
    panic!("No solution found!")
}

fn main() {
    let progs = parse_dig_progs(include_str!("../input"));
    println!("Answer part 2: {}", part_1(&progs))
}
