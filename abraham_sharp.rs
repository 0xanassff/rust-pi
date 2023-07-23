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

fn abraham_sharp() -> f64 {
    // grabbed from https://github.com/TheAlgorithms/Rust/blob/master/src/math/fast_power.rs also
    const MOD: usize = 1000000007;

    // initilize the variable pi
    let mut pi: f64 = 0.;
    
    // the calculation loop will stop on this variable, which maies this a finite loop, and improve accuracy
    let stop_step: i32 = 10000;

    for i in 0..=stop_step {
        // if even return 1, else return -1 instead, optimize a little! instead of -1*-1*-1...
        let y = if let 0=i%2{1.} else{-1.};

        // NOTE: 3^(1/2 - i) = 3^1/2 / 3^i = 1.732050808 / 3^i, since the function below don't accept float, and im lazy to generalize it
        let a: f64 = 2.0 * y * (1.732050808 / fast_power(3, i as usize, MOD) as f64);
        let b: f64 = 2.0 * i as f64 + 1.0;
        
        pi += a / b
    }
    
    pi
}
