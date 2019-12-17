use std::fs::File;
use std::io::prelude::*;

fn intcode(input: &mut Vec<usize>) {
    let mut x = 0;
    loop {
        if x > input.len() {
            panic!("Pragram didnt have an end signal");
        }
        match input[x] {
            1 => {
                let arg_1 = input[x + 1];
                let arg_2 = input[x + 2];
                let update_index = input[x + 3];
                input[update_index] = input[arg_1] + input[arg_2];
                x += 4;
            }
            2 => {
                let arg_1 = input[x + 1];
                let arg_2 = input[x + 2];
                let update_index = input[x + 3];
                input[update_index] = input[arg_1] * input[arg_2];
                x += 4;
            }
            99 => {
                break;
            }
            _ => panic!("Unknown Operator"),
        }
    }
}

// Day 2 uses comma seperated numbers
fn get_input() -> Box<Vec<usize>> {
    let mut file = File::open("resources/day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let iter = input.lines().next().unwrap().split(',');
    Box::new(iter.map(|x| x.parse::<usize>().unwrap()).collect())
}

pub fn p1() -> usize {
    let mut input = get_input();
    input[1] = 12;
    input[2] = 2;
    intcode(&mut *input);
    input[0]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 2842648)
    }
}
