
fn compare(n1: i64, n2: i64) -> i64 {
    if n1 > n2 {
        -1
    } else if n1 == n2 {
        0
    } else {
        1
    }
}

pub fn p1(init_pos : Vec<(i64, i64, i64)>, steps: usize) -> i64 {
    let mut vel = vec![(0,0,0), (0,0,0), (0,0,0), (0,0,0)];
    let mut pos = init_pos.clone();
    
    for _ in 0..steps {
        // update velocities
        for i in 0..4 {
            for j in (i+1)..4 {
                let v1 = vel[i];
                let v2 = vel[j];
                let p1 = pos[i];
                let p2 = pos[j];
                let res = (compare(p1.0, p2.0), compare(p1.1, p2.1), compare(p1.2, p2.2));
                vel[i] = (v1.0 + res.0, v1.1 + res.1, v1.2 + res.2);
                vel[j] = (v2.0 - res.0, v2.1 - res.1, v2.2 - res.2);

            }
        }
        // update positions
        for i in 0..4 {
            let p = pos[i];
            let v = vel[i];
            pos[i] = (p.0 + v.0, p.1 + v.1, p.2 + v.2);

        }
    }
    // get total energy 
    let mut total = 0;
    for i in 0..4 {
        let kin =  vel[i].0.abs() + vel[i].1.abs() + vel[i].2.abs();    
        let pot =  pos[i].0.abs() + pos[i].1.abs() + pos[i].2.abs(); 
        total += kin * pot
    }
    total
}

#[cfg(test)]
mod tests {
   
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(vec![(-8,-10,0),(5,5,10),(2,-7,3),(9,-8,-3)], 100), 1940);
        assert_eq!(p1(vec![(-1,0,2),(2,-10,-7),(4,-8,8),(3,5,-1)], 10), 179);
        assert_eq!(p1(vec![(-2,9,-5),(16,19,9),(0,3,6),(11,0,11)], 1000), 12053);
    }
}   
