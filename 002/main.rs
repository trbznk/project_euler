fn fib(target: i32) -> i32 {
    let mut a = 1;
    let mut b = 2;
    let mut c;
    let mut s = 0;

    while b < target {
        if b % 2 == 0 {
            s = s+b;
        }
        c = b;
        b = a+b;
        a = c;
    }

    s
}


fn main() {
    let target = 4000000;

    println!("{}", fib(target));
}