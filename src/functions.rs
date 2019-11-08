pub fn run() {
    greeting("hello", "naveen");
    println!("Value of 10+10 is {} ", add(10, 10));

    // Closure
    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum:{}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
