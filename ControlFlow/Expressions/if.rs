fn gcd(n: i32) -> i32 {
    let d = if n % 4 == 0 {  // Must be a boolean expression
                4
            } else if n % 3 == 0 {
                3
            } else if n % 3 == 0 {
                2  // all branches must return the same type
            } else {
                1
            };  // semi-colon required when used as an expression
    d
}

fn main() {
    println!("The GCD of 6 is {}", gcd(6));
}