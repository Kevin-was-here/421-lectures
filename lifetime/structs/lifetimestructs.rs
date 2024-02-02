#[derive(Debug)]
// The struct containing the reference has to live at least as long as the reference
struct MenuItem<'item> {
     name: &'item str,
     price: f32,
}

// We're just referencing name, so an anonymous lifetime is sufficient
impl MenuItem<'_> {
     fn display(&self) {
         println!("Enjoy {} for only ${:.2}", self.name, self.price);
     }
}

fn main() {

    //heap is used as vec bang of making string.
    //vec is an array of string objects
     let items = vec![String::from("Steak"), String::from("Hot Dog")];
     let steak = MenuItem { name: &items[0], price: 5.0E1 };

     steak.display();
}