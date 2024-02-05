fn main() {
    let mut counter = 0;

    let result = loop {  // the basic loop is infinite
        counter += 1;

        if counter == 10 {
            break counter * 2;  // the (optional) expression is the return value
        }
    };

    assert_eq!(result, 20);
}