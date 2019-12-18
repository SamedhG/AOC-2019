use std::collections::HashSet;
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

fn find_coords(wires: Box<Vec<Box<Wire>>>) -> Box<HashSet<(i64, i64)>> {
    let mut points = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for wire in *wires {
        match (*wire).0 {
            Direction::Right => {
                for _ in 0..wire.1 {
                    x += 1;
                    points.insert((x, y));
                }
            }
            Direction::Left => {
                for _ in 0..wire.1 {
                    x -= 1;
                    points.insert((x, y));
                }
            }
            Direction::Up => {
                for _ in 0..wire.1 {
                    y += 1;
                    points.insert((x, y));
                }
            }
            Direction::Down => {
                for _ in 0..wire.1 {
                    y -= 1;
                    points.insert((x, y));
                }
            }
        }
    }
    Box::new(points)
}

fn manhattan_distance(point: (i64, i64)) -> i64 {
    let (x, y) = point;
    x.abs() + y.abs()
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

pub fn p1() -> i64 {
    let input = get_input();
    let wire1 = find_coords(input.0);
    let wire2 = find_coords(input.1);
    wire1
        .intersection(&wire2)
        .map(|x| manhattan_distance(*x))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 209);
    }
}
