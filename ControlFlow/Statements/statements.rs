fn main() {
    let x = {  // a block expression
        let x = 3;
        x + 1  // results in this value
    };

    // macro evaluation is an expression, but the result is ignored by making it a statement
    println!("The value of x is: {x}");
}