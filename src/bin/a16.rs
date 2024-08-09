// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student1 = Student { name: "James".to_owned(), locker: Some(12) };
    let student2 = Student { name: "Zoey".to_owned(), locker: None };
    let student3 = Student { name: "Chris".to_owned(), locker: Some(9) };

    let students = vec![student1, student2, student3];

    for student in students {
        match student.locker {
            Some(num) => println!("{:?} was assigned to locker {:?}.", student.name, num),
            None => println!("No locker assigned to {:?}.", student.name)
        };
    };
    
}
