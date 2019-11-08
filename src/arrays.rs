// Arrays are fixed length and have same data type

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("{}", numbers[0]);

    // Change values
    numbers[2] = 20;
    println!("{:?}", numbers);

    // Get length
    println!("Length : {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32]= &numbers[0..2];
    println!("Slice : {:?}",slice )
}
