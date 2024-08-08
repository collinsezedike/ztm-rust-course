// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum Ticket {
    Backstage(String, i32), 
    Standard(i32),
    Vip(String, i32), 
}

fn main() {
    let names: Vec<&str> = vec!["Grace", "James"];
    let tickets: Vec<Ticket> = vec![
        Ticket::Standard(10), 
        Ticket::Vip(names[0].to_owned(), 40), 
        Ticket::Backstage(names[1].to_owned(), 100)
    ];
    
    for ticket in tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket is ${:?}", price),
            other => println!("{:?}", other)
        }   
    }

}
