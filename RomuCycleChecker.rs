use std::num::Wrapping;

#[inline]
fn rotate_left(d: u64, left_rotate: u64) -> u64 {
    let a = d << left_rotate;
    let b = d >> (64 - left_rotate);
    a | b
}

#[inline]
fn rand_next(state: u64) -> (u64, i32) {
    let result = state as i32;
    let next_state = Wrapping(15241094284759029579 as u64)
        .0
        .overflowing_mul(rotate_left(state, 32));
    (next_state.0, result)
}

fn check_sequences(numbers: &[i32]) {
    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2];
    let d = numbers[3];

    let positions = numbers
        .iter()
        .enumerate()
        .position(|x| x.1 == &a)
        .filter(|x| x != &0)
        .into_iter()
        .collect::<Vec<usize>>();

    positions
        .into_iter()
        .map(|i| {
            let aa = numbers[i.clone()];
            let bb = numbers[i + 1];
            let cc = numbers[i + 2];
            let dd = numbers[i + 3];

            (
                i,
                (aa, bb, cc, dd),
                aa == a && bb == b && cc == c && dd == d,
            )
        })
        .filter(|x| x.2)
        .for_each(|x| println!("Index at {}: {:?} vs {:?}", x.0, x.1, (a, b, c, d)));
}

fn main() {
    let seed = 731971696;
    let mut state = seed;

    let rands = (0..seed)
        .into_iter()
        .map(|_| {
            let (next_state, result) = rand_next(state);
            state = next_state;
            result
        })
        .collect::<Vec<i32>>();

    let cloned_rands = rands.clone();
    let slice = cloned_rands.as_slice();

    println!("Checking...");
    check_sequences(slice.clone());

    println!("Done!");
}
