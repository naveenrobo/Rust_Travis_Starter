/*
    Primitive types
    Integer - u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    Float   - f32, f64
    Boolean - bool
    Character- char
    Tuples
    Arrays
*/

pub fn run() {
    // Default is "i32"
    let _x = 1;

    // Default is "f64"
    let _y = 2.5;

    // Explicit type
    let _z: i64 = 2147483648;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max f32: {}", std::f32::MAX);

    // Boolean
    let is_active: bool = false;
    println!("{:?}", (_x, _y, _z, is_active));

    //Get boolean from expression
    let is_greater = 10 < 11;
    println!("Is Greater? {}",is_greater);

    // Character
    let _a1 = 'a';
    let _a2 = '\u{1F600}';
    println!("char is {}",_a1);
    println!("Emoji is {}",_a2);
}
