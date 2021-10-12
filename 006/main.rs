fn sum_sqaures_v1(n: i32) -> i32 {
    let mut sum = 0;
    for x in 1..n+1 {
        sum = sum + x.pow(2);
    }

    sum
}

fn sum_sqaures_v2(n: i32) -> i32 {
    (n*(n+1)*(2*n+1))/6
}

fn square_sum(n: i32) -> i32 {
    ((n*(n+1))/2).pow(2)
}

fn main() {
    let n = 100;
    let a1 = sum_sqaures_v1(n);
    let a2 = sum_sqaures_v2(n);
    let b = square_sum(n);
    
    println!("a1 {} a2 {}", a1, a2);
    println!("b {}", b);
    println!("b-a {}", b-a2);
}
