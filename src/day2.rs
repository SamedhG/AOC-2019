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

fn find(input: &Vec<usize>, to_find: usize) -> (usize, usize) {
    for i in 0..101 {
        for j in 0..101 {
            let mut new_input = input.clone();
            new_input[1] = i;
            new_input[2] = j;
            intcode(&mut new_input);
            if new_input[0] == to_find {
                return (i, j);
            }
        }
    }
    panic!("did not find value in defined range");
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

pub fn p2(search: usize) -> usize {
    let input = get_input();
    let (arg1, arg2) = find(&input, search);
    (100 * arg1) + arg2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 2842648);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(2842648), 1202);
        assert_eq!(p2(19690720), 9074);
    }
}
