use std::cell::RefCell;
use std::rc::Rc;

struct EnterExitCounts { enter: u32, exit: u32 }

impl EnterExitCounts {
    fn new() -> Self {
        Self { enter: 0, exit: 0 }
    }
}

struct House {
    doors: [EnterExitCounts; 2] // Front and back doors in this house
}

fn main() {

    //multithreaded version uses mutex as a wrapper around the refcell

    // Immutable house_visitors owns the House on the heap
   let house_visitors = Rc::new(RefCell::new(
        House { doors: [EnterExitCounts::new(), EnterExitCounts::new()] }
   ));

   // The outside watcher needs to record what they see
   let outside_watcher = Rc::clone(&house_visitors);

   outside_watcher.borrow_mut().doors[0].enter = 1; // See a person enter the front
   outside_watcher.borrow_mut().doors[1].exit = 1; // and leave by the back

   let visitors = house_visitors.borrow();
   let inside = visitors.doors[0].enter - visitors.doors[0].exit
              + visitors.doors[1].enter - visitors.doors[1].exit;
   println!("{} people currently inside", inside);
}