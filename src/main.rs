const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // if we remove "mut" we will get a compile-time error.
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("Three hours: {}", THREE_HOURS_IN_SECONDS);

    let x = x + 1;
    println!("Value = {}", x);
    
    {
        let x = x * 2;
        println!("Value = {}", x);
    }
    
    println!("The value = {}", x);
    println!("The value = {x}");
}
