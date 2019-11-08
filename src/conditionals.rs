// Conditionals

pub fn run() {
    let age = 18;

    // if/else
    if age > 18 {
        println!("what you would like to drink")
    } else {
        println!("you need to leave the bar")
    }

    // Shorthand if
    let is_of_age = if age > 21 { true } else { false };
    println!("is of age ? {}", is_of_age);
}
