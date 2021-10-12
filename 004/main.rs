fn is_palindrome(x: i32) -> bool {
    let xs = x.to_string();
    let n = xs.len()/2;

    let mut result = true;
    
    for i in 0..n {
        if xs.chars().nth(i).unwrap() != xs.chars().rev().nth(i).unwrap() {
            result = false;
        }
    }

    result
} 

fn main() {
    let start = 100;
    let stop = 999;
    let mut max_result = 0;

    for a in start..stop+1 {
        for b in start..stop+1 {
            let x = a*b;
            let result = is_palindrome(x);
            if result && x > max_result {
                println!("{}", x);
                max_result = x;
        }
        }
    }
}