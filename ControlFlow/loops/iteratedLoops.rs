fn main() {
    let a = [10.0, 20.0, 30.0, 40.0, 50.0];
    let mut index = 0;
    let mut sum = 0.0;

    while index < 5 {
        sum += a[index];
        index += 1;
    }

    println!("The mean is {:.1}", sum/(index as f32));
}