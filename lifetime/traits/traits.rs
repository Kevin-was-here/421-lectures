use std::cmp::Ordering;

// Many traits can be given a default implementation by the compiler
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
} //made our own type 


//implement partialORD for point
// PartialOrd requires PartialEq
impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


// requires Eq
impl std::cmp::PartialOrd for Point { 
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = self.x*self.x + self.y*self.y;
        let rhs = other.x*other.x + other.y*other.y;

        //ordering is an enum 
        if lhs < rhs {
            Some(Ordering::Less)
        }
        else if lhs == rhs {
            Some(Ordering::Equal)
        }
        else {
            Some(Ordering::Greater)
        }
    }
}

// You can use "where" to help make the signature more readable
fn print_max<T>(x: T, y: T) where T: std::cmp::PartialOrd + std::fmt::Debug {
    if x >= y {
        println!("{:?} >= {:?}", x, y);
    } else {
        println!("{:?} > {:?}", y, x);
    }
}

fn main() {
    let a = Point { x: 1.0, y: 0.0 };
    let b = Point { x: 0.0, y: 1.0 };

    print_max(a, b);
}