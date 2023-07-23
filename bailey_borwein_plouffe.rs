// grabbed from https://github.com/TheAlgorithms/Rust/blob/master/src/math/fast_power.rs
fn fast_power(mut base: usize, mut power: usize, modulus: usize) -> usize {
    assert!(base >= 1);

    let mut res = 1;
    while power > 0 {
        if power & 1 == 1 {
            res = (res * base) % modulus;
        }
        base = (base * base) % modulus;
        power >>= 1;
    }
    res
}

fn bailey_borwein_plouffe() -> f64 {
    // grabbed from https://github.com/TheAlgorithms/Rust/blob/master/src/math/fast_power.rs also
    const MOD: usize = 1000000007;

    // initilize the variable pi
    let mut pi: f64 = 0.;

    // the calculation loop will stop on this variable, which maies this a finite loop, and improve accuracy
    let stop_step: i32 = 10000;

    for i in 0..=stop_step {
        let a: f64 = 1.0 / fast_power(16, i as usize, MOD) as f64;
        let b: f64 = 4.0 / ((8 * i) + 1) as f64;
        let c: f64 = 2.0 / ((8 * i) + 4) as f64;
        let d: f64 = 1.0 / ((8 * i) + 5) as f64;
        let e: f64 = 1.0 / ((8 * i) + 6) as f64;
        
        pi += a * (b - c - d - e)
    }

    pi
}