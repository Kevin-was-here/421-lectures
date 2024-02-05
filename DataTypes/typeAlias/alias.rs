type Point = (i32, i32);

use Point as Coordinate;

fn main() {
    let p: Point = (100, 100);
    let northeast: Coordinate = p;

    println!("wind direction today: {:?}", northeast);
}