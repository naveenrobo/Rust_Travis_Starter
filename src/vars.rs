/*
 Variables hold primitive data or reference to data
 Variables are immutable by default
 Rust is a block scoped language
*/

pub fn run() {
    let name = "Naveen";
    let mut age = 37;
    println!("My name is {} and I'm {}", name, age);
    age = 38;
    println!("My name is {} and I'm {}", name, age);

    // Define constant
    const PI: f32 = 3.14;
    println!("Pi is {}", PI);

    // Multiple variables at once
    let (my_name, my_age) = ("Naveen", 26);
    println!("{} is {}",my_name, my_age )
}
