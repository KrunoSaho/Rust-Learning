use rayon::prelude::*;


fn is_a_divisor(n: i64, m: i64) -> bool {
    if n <= 1 {
        return true;
    }
    if m < 1 {
        return true;
    }

    n.rem_euclid(m) == 0
}


fn count_divisors(n: i64) -> i64 {
    let mut divs = 0;

    for i in 1..=n { //RAW!111
        if is_a_divisor(n, i) { 
            divs += 1;
        }
    }

    divs
}


fn main() {
    let data: std::vec::Vec<i64> = (1..=100_000).into_par_iter().map(|n| { 
        (0_i64..=(n as i64)).into_par_iter().sum() 
    }).collect();


    let div = data.par_iter().find_first(|x| count_divisors(**x) > 500);

    dbg!(div);
}
