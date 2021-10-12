fn evenly_divisible(x: i32, until: i32) -> bool {
    let mut result = true;

    for d in 2..until+1 {
        if x % d != 0 {
            result = false;
            break;
        }
    }

    result
}

fn main() {
    for x in 10..1000000000 {
        let result = evenly_divisible(x, 20);
        if result {
            println!("{}", x);
            break;
        }
    }
}