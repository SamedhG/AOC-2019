use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

// This assumes that three are no cycles
fn count_orbits(map: &HashMap<String, String>, body: &String) -> usize {
    match (*map).get(body) {
        None => 0,
        Some(x) => 1 + count_orbits(map, x),
    }
}

// This assumes that there are no cycles
fn get_chain(map: &HashMap<String, String>, body: &String) -> Vec<String> {
    let mut ancestors = Vec::new();
    let mut curr = body;
    loop {
        match (*map).get(curr) {
            None => break,
            Some(x) => {
                curr = x;
                ancestors.push(x.clone());
            }
        }
    }
    ancestors
}

// Returns a child->parent map and a set of all the bodies
fn get_input() -> (HashMap<String, String>, HashSet<String>) {
    let mut file = File::open("resources/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for line in input.lines() {
        let orbit: Vec<String> = line.split(')').map(|x| String::from(x)).collect();
        map.insert(orbit[1].clone(), orbit[0].clone());
        set.insert(orbit[0].clone());
        set.insert(orbit[1].clone());
    }
    (map, set)
}
pub fn p1() -> usize {
    let (map, set) = get_input();
    let mut total = 0;
    for body in set {
        total += count_orbits(&map, &body);
    }
    total
}

pub fn p2() -> usize {
    let (map, _) = get_input();
    let my_anc = get_chain(&map, &String::from("YOU"));
    let santa_anc = get_chain(&map, &String::from("SAN"));
    let mut dist_to_santa = 0;
    for ancestor in my_anc {
        dist_to_santa += 1;
        match santa_anc.iter().position(|x| *x == ancestor) {
            None => continue,
            Some(x) => return x + dist_to_santa - 1,
        }
    }
    panic!("no cmmon ancestor found");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 268504);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(), 409);
    }
}
