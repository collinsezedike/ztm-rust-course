// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let name: &str = "jonathan WILLIAM";
    // println!("Name in lowercase: {:?}", name.to_ascii_lowercase());
    // println!("Name in uppercase: {:?}", name.to_ascii_uppercase());
    println!("Name in lowercase: {:?}", name.to_lowercase());
    println!("Name in uppercase: {:?}", name.to_uppercase());
}
