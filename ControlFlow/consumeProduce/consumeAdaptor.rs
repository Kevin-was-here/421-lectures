fn main() {
    let a = [10.0, 20.0, 30.0, 40.0, 50.0];

    let mean = a.iter().sum::<f32>()/(a.len() as f32);
    println!("idiomatic average: {}", mean);
}