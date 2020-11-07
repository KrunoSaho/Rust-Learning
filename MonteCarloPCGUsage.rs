/**
Dependencies:
rand="0.7"
rand_pcg = "0.2.1"
time = "0.2.22"
**/
use rand::distributions::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;
use std::sync::mpsc;
use std::thread;
use std::vec::Vec;

fn fx_circle(px: f64, py: f64, xc: f64, yc: f64, r: f64) -> f64 {
    let x = (px - xc).powf(2.0);
    let y = (py - yc).powf(2.0);
    (x + y).sqrt() - r
}

fn monte_carlo(a: i64, b: i64) -> f64 {
    if a >= b {
        panic!("b{} <= a{}: ", b, a);
    }

    let n = (b - a).abs();
    let nf = n as f32;
    let rn = 1.0 / nf;

    let state = (a as u128)
        .overflowing_mul(2_u128.pow(64))
        .0
        .overflowing_add((b as u128).overflowing_mul(2_u128.pow(64_u32)).0)
        .0;
    let mut rng = Pcg64Mcg::new(state);
    let range_gen = Uniform::new(-1.0_f64, 1.0_f64);

    let mut hits = 0_f64;
    for _ in 0..n {
        let y = range_gen.sample(&mut rng);
        let x = range_gen.sample(&mut rng);

        let fxy = fx_circle(x, y, 0.0, 0.0, 1.0);
        if fxy <= 1e-9 {
            hits += 1.0;
        }
    }

    4.0 * hits * (rn as f64)
}

fn main() {
    let t0 = time::Instant::now();
    let no_of_blocks = 20_i64;
    let block_size = 100_000_000_0_i64;
    let mut receivers = vec![];

    for i in 0..no_of_blocks {
        let (a, b) = (i * block_size, (i + 1) * block_size);
        let (tx, rx) = mpsc::channel();

        receivers.push(rx);

        thread::spawn(move || {
            let x = monte_carlo(a, b);
            tx.send(x).unwrap();
        });
    }
    let mut avgs = receivers
        .iter()
        .map(|x| x.recv().unwrap())
        .collect::<Vec<f64>>();
    avgs.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let avg = avgs.iter().fold(0.0_f64, |a, x| a + x) / (no_of_blocks as f64);

    let t1 = time::Instant::now();
    let dt = t1 - t0;
    println!(
        "Sum() = P(x,y): {:?}\n{} seconds.",
        avg,
        dt.as_seconds_f32()
    );
}
