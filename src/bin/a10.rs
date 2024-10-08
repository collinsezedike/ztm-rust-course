// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_message(is_greater_than_100: bool) {
    let message = match is_greater_than_100 {
        true => "its big",
        false => "its small",
    };
    println!("{:?}", message);
}

fn main() {
    let num = 1000;
    let is_greater_than_100 = num > 100;

    print_message(is_greater_than_100);
}
