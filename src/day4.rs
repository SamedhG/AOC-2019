fn check_password(password: i64) -> bool {
    let mut has_double = false;
    let num_digits = (password as f64).log10().floor() as u32;
    let mut i : u32 = num_digits;
    loop {
        if i == 0 { break }
        let d1 = (password / 10_i64.pow(i)) % 10;
        let d2 = (password / 10_i64.pow(i-1)) % 10;
        i-=1;
        if d1 == d2 { has_double = true }
        if d1 > d2 { return false }
    }
    has_double
}

pub fn p1(start_idx: i64, end_idx: i64) -> i64 {
    let mut counter = 0;
    for i in start_idx..end_idx {
        if check_password(i) { counter +=1 }
    }
    counter
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(193651, 649729), 1605)
    }
}
