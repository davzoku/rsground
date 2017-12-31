// simple Hello World

fn main() {
    println!("Hello World");
    
    let age = 66;
    // bindings are immutable by default for safety.
    
    println!("I am {} years old", age);
    
    let mut dog = 1;
    dog = 2;
    
    println!("I have {} dogs", dog);
    
    //if statements
    if age <= 18{
        println!("Go to  school");}
    else if age > 18 && age <= 65{
        println!("Go to work");}
    else {
        println!("Go retire");
    }
    
    //for loops: (0..5) = (x=0; x<5)
    for x in 0..5 {
    println!("{}", x);
    }
    println!("---");
    //while loops
    let mut y= 10;
    let mut done = false;
    
    while !done {
        println!("{}", y);
        y = y-1;
        if y ==0{
            done = true;
        }
    }
    
    //functions
    print_hello("Tom");
}

fn print_hello(name: &str){
    println!("Hello {}", name);
    
}