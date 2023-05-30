// ----------------------------
// Variables
//      - Shadowing
//      - Constants
// ----------------------------

fn main() {
    // you can group variables between () to declare multiple values in an unique line
    let (_first_number, _second_number) = (250, 480.22);

    // You can use _ to separate large number for better readability
    let _large_number: i32 = 1_000_000;

    /*
        You can use :o in a print statement to display a value in octal
        :X to display a value in hexadecimal
        :b to display a value in binary
    */
    let x: u8 = 255;
    println!(
        "The value of the variable in octal is {:o}, in hexadecimal is {:X} and {:b} in binary",
        x, x, x
    );

    // -------------------------------------------
    //  Shadowing
    //      The concept of use the same name for
    //      a new variable, replacing it's type
    //      and value
    // -------------------------------------------
    let s: i32 = 5;
    let s: i32 = 5 * 5;
    println!("The value of s is {}", s);

    let q: i32 = 32;
    let q: char = 'A';

    let mut r: i32 = 65;
    {
        let r: i32 = 60;
        println!("Inside the code segment r: {}", r);
    }

    println!("Outside the code segment r: {}", r);

    // -------------------------------------------
    //  CONSTANT
    //      A constant is a fixed value that we
    //      can use in the code
    // -------------------------------------------

    const MAX_SALARY: u32 = 100_000;
}
