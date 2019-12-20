use std::fs::File;
use std::io::prelude::*;

fn get_input() -> String{
    let mut file = File::open("resources/day8.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    input
}

pub fn p1() -> usize {
    let data = get_input();
    //the counts of 0,1,2 in each layer
    let mut counts = Vec::new();
    let mut index = 0;
    for pixel in data.bytes() {
        if pixel == 10 { break; }
        if index % (25 * 6) == 0 {counts.push(vec![0,0,0])}
        let layer = index / (25 * 6);
        counts[layer][(pixel as usize - 48)] += 1;
        index += 1
    }
    let lowest_row = counts.iter().map(|v| v[0] ).min().unwrap();
    let lowest_row_idx = counts.iter().position(|v| v[0] == lowest_row).unwrap() as usize;
    counts[lowest_row_idx][1] * counts[lowest_row_idx][2] 
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(), 1088);
    }

}
