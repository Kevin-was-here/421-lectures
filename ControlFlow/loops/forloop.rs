fn main() {
    let a = [10.0, 20.0, 30.0, 40.0, 50.0];
    let mut sum = 0.0;

    for element in a {
        sum += element;
    }

    println!("The mean is {:.1}", sum/(a.len() as f32));
}