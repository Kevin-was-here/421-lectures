use std::cell::Cell;

struct Widget {
    model: String,
    version: (u8, u8, u8),
    in_stock: Cell<bool>,
}

fn main() {
    //deep copy of the struct

    let product = Widget {  // Product is declared immutable here
        model: "Super Thing".to_string(),
        version: (0, 9, 123),
        in_stock: Cell::new(true),
    };

    // Finally sold it!
    product.in_stock.set(false); // Tee-hee! We changed a field anyway!
}