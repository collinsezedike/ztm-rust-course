// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colour {
    Red,
    Blue,
    Green,
    Black,
    White,
}


fn print_colour(colour: Colour) {
    match colour {
        Colour::Red => println!("colour is red"),
        Colour::Blue => println!("colour is blue"),
        Colour::Green => println!("colour is green"),
        Colour::Black => println!("colour is black"),
        Colour::White => println!("colour is white"),
    };
}


fn main() {
    let my_colour = Colour::Black;
    print_colour(my_colour);
}
