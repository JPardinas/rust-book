fn main() {
    /*
        Compile error because variable is not mutable

        let x = 5;
        println!("The value of x is: {}", x);
        x = 6;
        println!("The value of x is: {}", x);
    */

    // mutable example
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constant example
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    
    // shadowing example
    let a = 5;

    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of x in the inner scope is: {}", a);
    }

    println!("The value of x is: {}", a);


    // changing type example
    let spaces = "   ";
    let spaces = spaces.len();

    /*
    Error example

    let mut spaces = "   ";
    spaces = spaces.len();
    */
}
