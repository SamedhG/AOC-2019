use std::fs::File;
use std::io::prelude::*;

enum Opcode {
    Add(i64, i64, i64),
    Multiply(i64, i64, i64),
    Input,
    Output,
    JumpIfTrue(i64, i64),
    JumpIfFalse(i64, i64),
    LessThan(i64, i64, i64),
    Equals(i64, i64, i64),
    Halt,
}

fn parse_opcode(opcode: i64) -> Opcode {
    match opcode % 100 {
        1 => Opcode::Add(
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
            (opcode / 10000) % 10,
        ),
        2 => Opcode::Multiply(
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
            (opcode / 10000) % 10,
        ),
        3 => Opcode::Input,
        4 => Opcode::Output,
        5 => Opcode::JumpIfTrue(
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
        ),
        6 => Opcode::JumpIfFalse(
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
        ),
        7 => Opcode::LessThan(
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
            (opcode / 10000) % 10,
        ),
        8 => Opcode::Equals(
            (opcode / 100) % 10,
            (opcode / 1000) % 10,
            (opcode / 10000) % 10,
        ),
        99 => Opcode::Halt,
        _ => panic!("Unknown Opcode"),
    }
}

fn intcode(memory: &mut Vec<i64>, input: &mut Vec<i64>, output: &mut Vec<i64>) {
    let mut x: usize = 0;
    loop {
        if x > memory.len() {
            panic!("Pragram didnt have an end signal");
        }
        match parse_opcode(memory[x]) {
            Opcode::Add(m1, m2, m3) => {
                let arg_1 = if m1 == 0 { memory[x + 1] as usize } else { x + 1 };
                let arg_2 = if m2 == 0 { memory[x + 2] as usize } else { x + 2 };
                let update_index = if m3 == 0 { memory[x + 3] as usize } else { x + 3 };
                memory[update_index] = memory[arg_1] + memory[arg_2];
                x += 4;
            }
            Opcode::Multiply(m1, m2, m3) => {
                let arg_1 = if m1 == 0 { memory[x + 1] as usize } else { x + 1 };
                let arg_2 = if m2 == 0 { memory[x + 2] as usize } else { x + 2 };
                let update_index = if m3 == 0 { memory[x + 3] as usize } else { x + 3 };
                memory[update_index] = memory[arg_1] * memory[arg_2];
                x += 4;
            }
            Opcode::Input => {
                let loc = memory[x + 1] as usize;
                memory[loc] = input.pop().unwrap();
                x += 2;
            }
            Opcode::Output => {
                let loc = memory[x + 1] as usize;
                output.push(memory[loc]);
                x += 2;
            }
            Opcode::JumpIfTrue(m1, m2) => {
                let check = if m1 == 0 { memory[x + 1] as usize } else { x + 1 };
                let new_p = if m2 == 0 { memory[x + 2] as usize } else { x + 2 };
                if memory[check] != 0 { x = memory[new_p] as usize } else { x+= 3 }
            }
            Opcode::JumpIfFalse(m1, m2) => {
                let check = if m1 == 0 { memory[x + 1] as usize } else { x + 1 };
                let new_p = if m2 == 0 { memory[x + 2] as usize } else { x + 2 };
                if memory[check] == 0 { x = memory[new_p] as usize } else { x+= 3 }
            }
            Opcode::LessThan(m1, m2, m3) => {
                let arg_1 = if m1 == 0 { memory[x + 1] as usize } else { x + 1 };
                let arg_2 = if m2 == 0 { memory[x + 2] as usize } else { x + 2 };
                let update_index = if m3 == 0 { memory[x + 3] as usize } else { x + 3 };
                let to_set = if memory[arg_1] < memory[arg_2] { 1 } else { 0 };
                memory[update_index] = to_set;
                x += 4;
            }
            Opcode::Equals(m1, m2, m3) => {
                let arg_1 = if m1 == 0 { memory[x + 1] as usize } else { x + 1 };
                let arg_2 = if m2 == 0 { memory[x + 2] as usize } else { x + 2 };
                let update_index = if m3 == 0 { memory[x + 3] as usize } else { x + 3 };
                let to_set = if memory[arg_1] == memory[arg_2] { 1 } else { 0 };
                memory[update_index] = to_set;
                x += 4;
            }
            Opcode::Halt => {
                break;
            }
        }
    }
}

fn get_input() -> Box<Vec<i64>> {
    let mut file = File::open("resources/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let iter = input.lines().next().unwrap().split(',');
    Box::new(iter.map(|x| x.parse::<i64>().unwrap()).collect())
}

pub fn p1() -> i64 {
    let mut memory = get_input();
    let mut input = vec![1];
    let mut output = Vec::new();
    intcode(&mut *memory, &mut input, &mut output);
    output[output.len() - 1]
}

pub fn p2() -> i64 {
    let mut memory = get_input();
    let mut input = vec![5];
    let mut output = Vec::new();
    intcode(&mut *memory, &mut input, &mut output);
    println!("{:?}", output);
    output[output.len() - 1]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 11049715);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(), 2140710);
    }
}
