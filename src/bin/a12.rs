// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


enum Colour {
    Black,
    White, 
    Brown,
    Biege,
}

struct ShippingBox {
    dimensions: (f64, f64, f64),
    weight: f64,
    colour: Colour
}

impl ShippingBox {
    fn new(dimensions: (f64, f64, f64), weight: f64, colour: Colour) -> Self {
        Self { dimensions, weight, colour }
    }

    fn print_features(&self) {
        let (x, y, z) = self.dimensions;
        let colour = match self.colour {
            Colour::Black => String::from("black"),
            Colour::White => String::from("White"),
            Colour::Brown => String::from("Brown"),
            Colour::Biege => String::from("Biege"),
        };
        
        println!("Weight: {:?}", self.weight);
        println!("Dimensions: {:?}x{:?}x{:?}", x, y, z);
        println!("Colour: {:?}", colour);
    }
}

fn main() {
    let box_colour = Colour::Brown;
    let box_dimensions = (2.0, 3.0, 4.5);
    let box_weight = 1.2;

    let shipping_box = ShippingBox::new(box_dimensions, box_weight, box_colour);
    shipping_box.print_features();
}
