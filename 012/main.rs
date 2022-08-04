fn triangle_number(n: u32) -> u32 {
    (n*(n+1))/2
}

fn count_divisors(x: u32) -> u32 {
    let mut count: u32 = 1; // init with 1 to count the number itself as a divisor
    for d in 1..x/2+1 {
        if x % d == 0 {
            count += 1;
        }
    }
    count
}

fn print_grid(X: Vec<u32>, length: usize) {
    for i in 0..X.len() {
        print!("{:3} ", X[i]);
        if (i+1) % length == 0 {
            print!("\n");
        }
    }
}

fn prime_factors(x: u32, factors: Vec<u32>) -> Vec<u32> {

    vec!()
}

fn main() {
    // println!("{:?}", prime_factors(1, vec!()));
    
    // let n = 14444;
    // let x = triangle_number(n);
    // let nb: f32 = -0.5 + (0.5_f32.powf(2.0)+2.0*(x as f32)).sqrt(); 
    // println!("{}|{}: {}", n, nb, x);
    
    const N: u32 = 500;
    let mut n = 0;
    loop {
        let x = triangle_number(n);
        let mut d = 0;
        if x % 100 == 0 {
            d = count_divisors(x);
            println!("{:4}: {:4} {:4}", n, x, d);
        } 
        if d > N {
            break;
        } else {
            n += 1;
        }
    }

    // print_grid(D, 25);

    // let mut n = 1;
    // loop {
    //     sum += n;
    //     if d > N {
    //         println!("{}: {} divisors={}", n, sum, d);
    //         break;
    //     } else {
    //         if n % 100 == 0 {
    //             let d = count_divisors(sum);
    //             println!("{}: {} divisors={}", n, sum, d);
    //         }
    //         n += 1;
    //     }
    // }
}
