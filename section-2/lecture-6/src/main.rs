fn main() {
    // This is our first program in this course
    // This is the second line of comment

    /*
        This is
        a
        multi-line
        comment
    */

    println!("Hello From Rust");
    println!("Hello World");
    println!("{}", 10);
    println!(
        "My first name is {} and my last name is {}",
        "Vitor", "Coelho"
    );

    print!("This is a print command");
    print!("This is goint to be written in the same line");

    print!(
        "
        This is going to be
            printed on multiple
            lines
    "
    );

    println!("\n\n This is going to be printed after a line");
    println!("\t This will have some empty space at the beginning");

    println!(
        "I am doing {2} from {1} years and i {0} it",
        "like", 20, "programming"
    );

    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        language = "Rust",
        activity = "Code"
    );

    println!("The summation of 25 + 10 = {}", 25 + 10);
}
