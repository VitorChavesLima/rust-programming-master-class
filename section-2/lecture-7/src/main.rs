// ----------------------------
// - Variables
// - Scalar Data Types
// ----------------------------

fn main() {
    // -------------------------------------------
    //  Rules for naming variables
    //  - Only letters, digits and underscores
    //  - Should begin with letter or underscore
    //  - Case sensitive
    // -------------------------------------------

    // -------------------------------------------
    //  Scalar Type
    //  - Integers ( signed and unsigned )
    //      - Signed
    //          - i8, i16, i32, i64
    //      - Unsigned
    //          - u8, u16, u32, u64
    // -------------------------------------------

    println!("Signed Integer Maximum Values");
    println!("The maximum number in i8 = {}", std::i8::MAX);
    println!("The maximum number in i16 = {}", std::i16::MAX);
    println!("The maximum number in i32 = {}", std::i32::MAX);
    println!("The maximum number in i64 = {}", std::i64::MAX);

    println!("\nUnsigned Integer Maximum Values");
    println!("The maximum number in u8 = {}", std::u8::MAX);
    println!("The maximum number in u16 = {}", std::u16::MAX);
    println!("The maximum number in u32 = {}", std::u32::MAX);
    println!("The maximum number in u64 = {}", std::u64::MAX);

    // -------------------------------------------
    //  Scalar Type
    //  - Floats
    //      - f32, f64
    // -------------------------------------------

    println!("\nFloat Maximum Values");
    println!("The maximum number in f32 = {}", std::f32::MAX);
    println!("The maximum number in f64 = {}", std::f64::MAX);

    // -------------------------------------------
    //  Scalar Type
    //  - Bool
    // -------------------------------------------

    let bool_value: bool = false;
    println!("\nThe value of the bool var is {}", bool_value);

    // -------------------------------------------
    //  Scalar Type
    //  - Characters
    //      - Represente single letters,
    //      - digit,
    //      - emoji's or
    //      - unicode scalar values
    // -------------------------------------------

    let c1: char = 'a';
    let c2: char = '3';
    let c3: char = '+';
    let c4: char = '\u{1F600}';
    let c5: char = '\"';

    println!(
        "\nThe value of c1 is {}, c2 is {}, c3 is {}, c4 is {} and c5 is {}",
        c1, c2, c3, c4, c5
    );
}
