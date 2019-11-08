pub fn run(){
    // Print to console
    println!("Hello from print.rs file");

    // Basic Formating
    println!("Number is {}",1);
    println!("{} is from {}","Naveen","Salem");

    // Positional Arguments
    println!("{0} is form {1} and {0} like to {2}","Naveen","Salem","code" );

    // Named Arguments
    println!("{name} likes to {hobby}",name="Naveen", hobby="code" );

    // Placeholder traits
    println!("Binary is {:b}\t Hex is {:x}\t Octal {:o}",10,10,10 );

    // Placeholder for debug traits
    println!("{:?}",(10,true,"naveen") );

    // Basic math
    println!("10+10 = {}",10+10 );

}