// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    quantity: i32,
    id: i32,
}


fn display_quantity(grocery_item: &GroceryItem) {
    println!("Quantity: {:?}", grocery_item.quantity);
}

// If I make choose not to make GroceryItem a borrowed argument, 
// the code would still work provided I call this function last.
fn display_id(grocery_item: &GroceryItem) {
    println!("ID number: {:?}", grocery_item.id);
}


fn main() {
    let chocolate = GroceryItem{ quantity: 2, id: 3 };
    display_quantity(&chocolate);
    display_id(&chocolate);
}
