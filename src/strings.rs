// Primitive str = Immutable fixed-length string somewhere in memory
// String = growable, heap-allocated data structure

pub fn run() {
    let mut hello = String::from("Hello ");

    // length
    let len = hello.len();
    println!("{} is {} char",hello, len );

    // Push
    hello.push('W');
    hello.push_str("orld");
    println!("{}",hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // is empty
    println!("Is empty {}",hello.is_empty());

    //Contains
    println!("Contains 'World' -> {}",hello.contains("World"));

    // reply
    println!("Replace 'world' with 'naveen', {}",hello.replace("World", "Naveen!") );

    // Loop through stirng by white space
    for word in hello.split_whitespace() {
        println!("{}",word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');

    assert_eq!(1, s.len());
    assert_eq!(10, s.capacity());
    println!("String with capacity : {}",s );
}