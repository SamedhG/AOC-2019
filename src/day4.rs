fn check_password(password: i64) -> bool {
    let mut has_double = false;
    let num_digits = (password as f64).log10().floor() as u32;
    let mut i: u32 = num_digits;
    loop {
        if i == 0 {
            break;
        }
        let d1 = (password / 10_i64.pow(i)) % 10;
        let d2 = (password / 10_i64.pow(i - 1)) % 10;
        i -= 1;
        if d1 == d2 {
            has_double = true
        }
        if d1 > d2 {
            return false;
        }
    }
    has_double
}

fn check_password_2(password: i64) -> bool {
    let mut has_double = false;
    let mut last_double = false;
    let mut double_ok = false;
    let num_digits = (password as f64).log10().floor() as u32;
    let mut i: u32 = num_digits;
    loop {
        if i == 0 {
            break;
        }
        let d1 = (password / 10_i64.pow(i)) % 10;
        let d2 = (password / 10_i64.pow(i - 1)) % 10;
        i -= 1;
        if d1 > d2 {
            return false;
        }
        if !double_ok && d1 == d2 {
            if last_double {
                has_double = false;
            } else {
                has_double = true;
            }
            last_double = true;
        } else {
            if has_double {
                double_ok = true
            }
            last_double = false;
        }
    }
    double_ok || has_double
}

pub fn p1(start_idx: i64, end_idx: i64) -> i64 {
    let mut counter = 0;
    for i in start_idx..end_idx {
        if check_password(i) {
            counter += 1
        }
    }
    counter
}

pub fn p2(start_idx: i64, end_idx: i64) -> i64 {
    let mut counter = 0;
    for i in start_idx..end_idx {
        if check_password_2(i) {
            counter += 1
        }
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

    #[test]
    fn test_p2() {
        assert_eq!(p2(193651, 649729), 1102)
    }
}
