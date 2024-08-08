// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i64,
    name: String,
    favourite_colour: String,
}

fn print_info(person: &Person) {
    println!("Hello, I am {:?}. I am {:?} years old and my favourite colour is {:?}.", 
        person.name, person.age, person.favourite_colour)
}

fn main() {
    let person1 = Person { 
        age: 15, 
        name: String::from("Goodness"), 
        favourite_colour: String::from("Yellow"), 
    };

    let person2 = Person { 
        age: 25, 
        name: String::from("Paul"), 
        favourite_colour: String::from("Black"),
    };

    let person3 = Person { 
        age: 55,
        name: "Martin".to_owned(), 
        favourite_colour: "Blue".to_owned(), 
    };

    let people = vec![person1, person2, person3];

    for person in people {
        if person.name == "Goodness" {
            print_info(&person);
        }
    }
}
