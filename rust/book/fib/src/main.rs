use std::io;

fn main() {
    println!("Which fibonacci number doth master crave?");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("You must enter a number");

    let n: i32 = n.trim().parse().expect("Failed to parse input");
    if n < 1 {
        println!("Invalid number");
    } else if n == 1 {
        println!("fib(1) = 0");
    } else if n == 2 {
        println!("fib(2) = 1");
    } else {
        let mut prev: i128 = 0;
        let mut acc: i128 = 1;
        for _ in 3..=n {
            let tmp = prev + acc;
            prev = acc;
            acc = tmp;
        }
        println!("fib({n}) = {acc}");
    }
}
