use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;


fn count_orbits(map: &HashMap<String, String>, body: &String) -> usize {
    match (*map).get(body) {
        None => 0,
        Some(x) => 1 + count_orbits(map, x),
    }
}

// Returns a child->parent map and a set of all the bodies
fn get_input() -> (HashMap<String, String>, HashSet<String>){
    let mut file = File::open("resources/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut map = HashMap::new();
    let mut set = HashSet::new();
    for line in input.lines() {
        let orbit : Vec<String> = line.split(')').map(|x| String::from(x)).collect();
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

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 268504);
    }

}
