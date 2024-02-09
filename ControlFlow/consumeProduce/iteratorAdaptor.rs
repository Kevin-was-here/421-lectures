fn main() {
    let a = ["1.0", "two", "NaN", "4.0"];

    let good: Vec<f32> = a.iter()
                          .map(|s| s.parse())
                          .filter(|s| s.is_ok())
                          .map(|result| result.unwrap())
                          .collect();
    println!("parseable: {:?}", good);
}