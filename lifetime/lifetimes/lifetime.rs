#[derive(Debug)]
struct MenuItem {
     name: &'static str,
     price: f32,
}

impl MenuItem {
     fn display(&self) {
         println!("Enjoy {} for only ${:.2}", self.name, self.price);
     }
}

fn main() {
     let steak = MenuItem { name: "Steak", price: 5.0E1 };

     steak.display();
}