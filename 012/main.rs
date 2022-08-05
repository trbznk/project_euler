fn triangle_number(n: u32) -> u32 {
    (n*(n+1))/2
}

fn count_divisors(x: u32) -> u32 {
    let mut count: u32 = 0;
    let n: u32 = (x as f32).sqrt() as u32;
    for d in 1..n+1 {
        if x % d == 0 {
            if x / d == d {
                count += 1
            } else {
                count += 2;
            }
        }
    }
    count
}

fn main() {
    // SOLUTION: 12375: 76576500  576
    const N: u32 = 500;
    let mut n = 0;
    loop {
        let x = triangle_number(n);
        let mut d = 0;
        d = count_divisors(x);
        println!("{:4}: {:4} {:4}", n, x, d);
        if d > N {
            break;
        } else {
            n += 1;
        }
    }
}

