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

fn bellard() -> f64 {
    // grabbed from https://github.com/TheAlgorithms/Rust/blob/master/src/math/fast_power.rs also
    const MOD: usize = 1000000007;

    // since 1/2^6 = 0.015625
    let x: f64 = 0.015625;

    // this is the part of the equation that dosen't include 1/2^6 in it
    let mut n: f64 = 0.;

    // the calculation loop will stop on this variable, which maies this a finite loop, and improve accuracy
    let stop_step: i32 = 10000;

    for i in 0..=stop_step {
        // if even return 1, else return -1 instead, optimize a little! instead of -1*-1*-1...
        let y = if let 0=i%2{1.}else{-1.};
        let z = 10*i as usize;

        // used a community function for power using bitwise operators
        let n1: f64 = y / fast_power(2, z, MOD) as f64;

        // calculate and convert i32 types to f64
        let n2: f64 = 32. / (4 * i + 1) as f64;
        let n3: f64 = 1. / (4 * i + 3) as f64;
        let n4: f64 = 256. / (10 * i + 1) as f64;
        let n5: f64 = 64. / (10 * i + 3) as f64;
        let n6: f64 = 4. / (10 * i + 5) as f64;
        let n7: f64 = 4. / (10 * i + 7) as f64;
        let n8: f64 = 1. / (10 * i + 9) as f64;
        
        n += n1 * (-n2 - n3 + n4 - n5 - n6 - n7 + n8);
    }

    // no need to initilize a new variable "pi" and return it, instead return it directly
    n * x
}