fn is_prime(p: i64) -> bool {
    let mut result = true;

    for d in 2..p {
        if p % d == 0 {
            result = false;
            break;
        }
    }

    result
}

fn main() {
    const N: usize = 10000;
    let mut prims = Vec::new();

    //println!("{}", prims);

    for p in 2..N as i64 {
        if is_prime(p.into()) {
            prims.push(p);
        }
    }

    let mut pf = Vec::new();

    let target = 600851475143; //13195;
    let mut current_number = target;
    let mut i = 0;

    loop {
        if current_number % prims[i] == 0 {
            pf.push(prims[i]);
            current_number = current_number/prims[i];
            i = 0;
            if current_number == 1 {
                break
            }
        } else {
            i = i+1;
        }
    }

    println!("{}", pf.iter().max().unwrap());
}