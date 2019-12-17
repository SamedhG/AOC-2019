use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Wire(Direction, usize);

fn convert(s: &str) -> Box<Wire> {
    let (dir, len) = s.split_at(1);
    let size = len.parse::<usize>().unwrap();
    let d = match dir {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("unknown direction"),
    };
    Box::new(Wire(d, size))
}

fn get_input() -> (Box<Vec<Box<Wire>>>, Box<Vec<Box<Wire>>>) {
    let mut file = File::open("resources/day3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut wires = input.lines();
    let wire1 = wires.next().unwrap().split(',');
    let wire2 = wires.next().unwrap().split(',');
    let w1 = Box::new(wire1.map(|x| convert(x)).collect());
    let w2 = Box::new(wire2.map(|x| convert(x)).collect());
    (w1, w2)
}

pub fn p1() -> usize {
    let input = get_input();
    println!("{:?}", input);
    3
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 2842648);
    }
}
