use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;

fn calculate_fuel(f: i64) -> i64 {
    f/3 - 2
}

fn cumulative_fuel(f: i64) -> i64 {
    if f <= 0 {
        0
    } else {
        f + cumulative_fuel(calculate_fuel(f))
    }
}

fn get_input() ->  Box<Vec<i64>> { 
    let mut file = File::open("resources/day1.txt").unwrap();
    let mut input =  String::new();
    file.read_to_string(&mut input).unwrap();
    let iter = input.split_whitespace();
    Box::new(iter.map(|x| x.parse::<i64>().unwrap()).collect())
}

pub fn p1() -> i64 {
    let arr = get_input();
    arr.iter().fold(0, |acc, x| acc + calculate_fuel(*x))
}

pub fn p2() -> i64 {
    let arr = get_input();
    arr.iter().fold(0, |acc, x| acc + cumulative_fuel(calculate_fuel(*x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(calculate_fuel(10), 1);
        assert_eq!(calculate_fuel(9), 1);
    }

    #[test]
    fn test_cumulative_calc() {
        assert_eq!(cumulative_fuel(2), 2);
        assert_eq!(cumulative_fuel(654), 966);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 3150224);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(), 4722484);
    }
}
