// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavour {
    Tropical,
    Orange,
    Apple,
    Cola,
}


struct Drink {
    flavour: Flavour,
    ounce: f64,
}


fn print_drink(drink: Drink) {
    match drink.flavour {
        Flavour::Tropical => println!("Flavour: Tropical"),
        Flavour::Orange => println!("Flavour: Orange"),
        Flavour::Apple => println!("Flavour: Apple"),
        Flavour::Cola => println!("Flavour: Cola"),
    };
    println!("Ounce: {:?}", drink.ounce)
}


fn main() {
    let my_flavour = Flavour::Cola;
    let my_drink = Drink { 
        flavour: my_flavour, 
        ounce: 0.8 
    };
    print_drink(my_drink);
}
