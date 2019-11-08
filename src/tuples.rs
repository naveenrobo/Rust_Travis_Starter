// Tuples can be of different types
// Max 12 elements
pub fn run() {
    let person:(&str, &str, i8) = ("Naveen","Sakthivel",26);

    println!("{:?}",person);
    println!("{} {} is {}",person.0, person.1, person.2 );
}