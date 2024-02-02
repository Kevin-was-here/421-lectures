
//multiple parameters
fn show_max(x: &f32, y: &f32) {
    let op = if x >= y {
        ">="
    } else {
        "<"
    };
    println!("{0} {1} {2}", x, op, y);
}


//walking through the string to find the first word
fn first_word(sentence: &str) -> &str {
    for (i, &c) in sentence.as_bytes().iter().enumerate() {
        if c == b' ' {
           return &sentence[..i];
        }
    }
    &sentence
    //life time of the output is the same as the lifetime of the input
}

fn ref_max<'a>(x: &'a f32, y: &'a f32) -> &'a f32 {
    //returning the reference based on what is passed in
    if x >= y {
        x
    }
    else {
        y
    }

    //lifetime of the output is the same as the lifetime of the input
    //rust gets confused if the lifetimes are not the same
}

fn main() {
    let x = 1.414;
    let y = 3.141;
    let words = String::from("compare x and y");

    show_max(&y, &x);
    println!("first word: {}", first_word(&words));
    println!("max is {}", ref_max(&y, &x));
}