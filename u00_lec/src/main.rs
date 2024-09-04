//-------------------------- Print ------------------------------------------------------------------------------------
/* fn main() {
    println!("Hello, world!");              //no args
    println!("Hello {}", "world");          //simple
    println!("Hello {1} {0}", "world", 1);  //positional
    println!("{value}", value = 4);         //named
    println!("Hello {:?}", ("world", 5));   //debug
    println!("Hello {:#?}", ("world", 5));  //pretty-print
} */

//-------------------------- Fibonacci ------------------------------------------------------------------------------------
/* fn fib(n:u8) -> u16 {
    if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn main(){
    let n = 11;
    let res = fib(n);
    println!("fib({}) = {}", n, res);
} */

//-------------------------- Variables & Mutability & Constants ------------------------------------------------------------------------------------
/* fn main(){
    let mut variable = 3;               // Type could, e.g., be u16, i64, bool, String, ...
    variable = 5;                            // Variables can be immutable (default) or mutable
    const PI: f32 = 3.14;                    // Using const instead of let and specifying the type
} */

//-------------------------- Shadowing i.e. a variable name can be reused in a code block ------------------------------------------------------------------------------------
fn main() {
    let shadow = 1;
    {
        println!("before beign shadowed: {}", shadow);

        let shadow = "abc";
        println!("shadowed in inner block: {}", shadow);
    }
    println!("outside inner block: {}", shadow);

    let shadow = 2;
    println!("shadowed in outer block: {}", shadow);
}

//-------------------------- Functions ------------------------------------------------------------------------------------
/* fn mul(x: i32, y: i32) -> i32 { //MUST declare all parameter types and they are not inferred by the compiler
    x*y
}

fn main(){
    let x = 23;
    let y = 11;
    let res = mul(x,y);
    println!("Result of multiplication of {} and {} is {}", x, y, res);
} */

//-------------------------- Recursion ------------------------------------------------------------------------------------
/* fn gcd(m: i32, n: i32) -> i32 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n%m, m)
    }
}
fn main(){
    let m = 22;
    let n = 11;
    println!("The greatest common divisor {} and {} is {}", m, n, gcd(m, n));
} */

//-------------------------- Namespaces ------------------------------------------------------------------------------------
/* mod math {
    pub fn gcd(m: i32, n: i32) -> i32 {
        if m == 0 {
            n.abs()
        } else {
            gcd(n%m, m)
        }
    }

    pub fn fib(n: u32) -> u32 {
        if n == 0 {
            1
        } else if n == 1 {
            1
        } else {
            fib(n-1) + fib(n-2)
        }
    }
}

fn main() {
    use math::fib;
    let gcd = math::gcd(30, 12);
    let f = fib(3);
    println!("gcd: {}, fib: {}", gcd, f);
} */

//-------------------------- Control Flow ------------------------------------------------------------------------------------
/* fn main() {
    let condition = true;
    let number = if condition {5} else {6};
    print!("The value of number is: {}", number);
} */
//-------------------------- Loop ------------------------------------------------------------------------------------
/* use core::time;
loop {
    let interval = time::Duration::from_secs(1)
    match send_heartbeat() {
        Heartbeat::Success => {
            threat::sleep(interval);                        // On success, the thread waits until one interval has passed
        },
        Heartbreat::Timeout => {
            break;                                          // On timeout, the loop is exited and reconnection happens
        },
        Heartbeat::Error => panic!("unexpected condition"), // On error, the program halts.
    }
} */
//-------------------------- While ------------------------------------------------------------------------------------
/* fn gcd(mut m: i32, mut n: i32) -> i32 {
    while m != 0 {
        let old_m = m;
        m  = n%m;
        n = old_m;
    }
    n.abs()
}
fn main() {
    let m = 32;
    let n = 24;a
    println!("The greatest common divisor of {} and {} is {}", m, n, gcd(m, n));
} */

//-------------------------- For ------------------------------------------------------------------------------------
/* fn fib(n: u8) -> u16 {
    let mut fib = (1, 1);
    for _ in 0..n {
        // Range from 0 to n-1
        fib = (fib.1, fib.0 + fib.1)
    }
    fib.0
}

fn main() {
    let n = 4;
    println!("fib({}) = {}", n, fib(n));
} */
//-------------------------- Worksheet ------------------------------------------------------------------------------------
/* fn fib (n: u8) -> u16 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    let n = 11;
    println!("The Fibonacci number for n = {} is {}", n, fib(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_fib() {
        assert_eq!(fib(11), 89)
    }
}

macro_rules! fib_tests {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, fib(input))
            }
        )*
    };
}

fib_tests! {
    fib_0: (0,0),
    fib_1: (1,1),
    fib_2: (2,1),
    fib_3: (3,2),
    fib_4: (4,3),
    fib_5: (5,5),
    fib_6: (6,8),
    fib_7: (7,13),
    fib_8: (8,21),
    fib_9: (9,34),
} */