fn main() {
    const N: usize = 2000000;
    let mut sieve: [bool; N] = [true; N];
    sieve[0] = false;
    sieve[1] = false;
    sieve[2] = true;

    let mut prime_sum: u64 = 0;
    for i in 2..N {
        if sieve[i] {
            prime_sum += i as u64;
            let mut j = 2;
            while j*i < N {
                sieve[j*i] = false;
                j += 1;
            }
        }
    }

    println!("{}", prime_sum);
}


