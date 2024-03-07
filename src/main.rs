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

    let theName = "Leon";
    let s = theName;
    let s = s.len();

    println!("Value = {}", s);

    // The following will throw an error:
    // let mut spaces = "    ";
    // spaces = spaces.len();
    //
    //
    let something: isize = 23;


    // Integer literals:
    let aDec: i32 = 32;
    let aHex: i32 = 0xff;
    let aOct: i32 = 0o32;
    let aBin: i32 = 0b101;

    println!("Some value: {}", aBin);
    
    let aByte: u8 = 5;
    
    println!("A byte: {}", aByte);

    let someX = 2.0;        // f64
    let someY: f32 = 3.0;   // f32
                            //
    let t = true;
    let f: bool = false;

    // Character types...
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("Cat: {}", heart_eyed_cat);
}
