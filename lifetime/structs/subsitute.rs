#[derive(Debug)]
struct MenuItem<'item> {
     name: &'item str,
     price: f32,
}

impl<'item> MenuItem<'item> {
     fn display(&self) {
         println!("Enjoy {} for only ${:.2}", self.name, self.price);
     }


     //subsitute method to replace when the item is out of stock
     // This uses lifetime coercion to say lifetime 'b is at least as long as 'a 
     //'b: 'item means the lifetime of b is at least as long as 'item
     fn substitute<'b: 'item>(& mut self, new_item: &'b str) { 
         //new reference is given to name
        self.name = new_item;
     }
}

//testing the borrow checker
//this works because fastfood is 'static 
fn nice_try<'a>(item: &'a mut MenuItem) { 

    //this will not work because fast_food is not 'static as in it is owned by the function
    //let fast_food = String::from("Fast food");
     
     let fast_food = "Fast food";
     item.substitute(&fast_food);
}

fn main() {
     let items = vec![String::from("Steak"), String::from("Hot Dog")];
     let mut steak = MenuItem { name: &items[0], price: 5.0E1 };

     nice_try(&mut steak);
     
     steak.substitute(&items[1]);
     steak.display();
}