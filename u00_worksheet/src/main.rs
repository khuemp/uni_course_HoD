fn main() {
    let name = "Khue";
    let age = "22";
    let fib_value = 14;
    let is_prime_value = 13;
    println!("My name is: {}. I am now {} years old.", name, age);
    println!("Fibonacci number of {} is {}.", fib_value, fib(fib_value));
    println!("Is {} a prime number? {}.", is_prime_value, is_prime(is_prime_value));
}
fn fib(n: usize) -> usize {
    let mut f0 = 0;
    let mut f1 = 1;
    let mut res = 0;
    if n == 0 {
        return f0;
    } else if n == 1 {
        return f1;
    }
    for _i in 2..=n {
        res = f1 + f0;
        f0 = f1;
        f1 = res
    }
    res
}
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n%i == 0 {
            return false;
        }
    }
    true
}
