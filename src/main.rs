fn main() {
    println!("Hello, world!");

    // * Data Types
    // Boolean - true, false
    // Integer - 1, 2, 50, 99, -2
    // Double / Float - 1.1, 5.5, 2.0
    // Character - 'A','$'
    // String - "Hello", "This is string"

    // * Variables
    // Assign data to temporary memory location
    // Can be set to any value & type
    // Immutable by default, but can be mutable

    // Examples
    // let two = 2;
    // let hello = "Hello";
    // let j = 'J';
    // let my_half = 0.5;
    // let mut my_name = "Nick";
    // let quit_program = false;
    // let your_half = my_half;

    // * Functions
    // A way to encapsulate program functionality
    // Optionally accept data
    // Optionally return data
    // Utilized for code organization

    // Examples
    // fn add(a: i32, b: i32) -> i32 {
    //     return a + b;
    // }
    // let total = add(2, 2);
    // print!("Total: {total}\n");

    // * Macros
    // Macros are similar to functions
    // Exclamation symbol "!"" makes difference between macro and function
    // Macros expand into additional code
    // The println macro - "prints" information to the terminal
    // Useful for debugging

    // Examples
    // let life = 42;
    // println!("hello");
    // // Take external value and include in the statement
    // println!("{:?}", life);
    // println!("{:?} {:?}", life, life * 2);
    // println!("The meaning of life is {:?}", life);
    // // Debugging / End User Display
    // println!("{life:?}");
    // println!("{life}");

    // * Control Flow
    // Code is executed line-by-line
    // Actions are performed & control flow may change
    // Specific conditions can change control flow: "if", "else", "else if"
    // Try to always include "else" statement, unless there truly is no alternative case
    
    // Examples
    // let age = 25;
    // if age > 18 {
    //     println!("Good, you can enter!")
    // } else {
    //     println!("Sorry, you can't enter!")
    // }

    // * Loops
    // Repetetion is called looping or "iteration"
    // Multiple types of loops:
    // "loop" - infinite loop
    // "while" - conditional loop

    // Example (Loop)
    // let mut counter = 0;
    // loop {
    //     if counter == 5 {
    //         println!("Hit the stopping condition");
    //         break;
    //     }
    //     println!("{:?}", counter);
    //     counter += 1;
    // }

    // Example (While)
    // while counter != 5 {
    //     println!("{:?}", counter);
    //     counter += 1;
    // }

    // * Match expresions
    // They are used to add logic to your programs
    // Similar to "if", "else"
    // However, the difference is that "Match" all options must be accounted for

    // Example (bool)
    // let some_bool = true;
    // match some_bool {
    //     true => println!("It's true"),
    //     false => println!("It's false")
    // }

    // Example (int)
    // let some_int = 3;
    // match some_int {
    //     1 => println!("It's 1"), 
    //     2 => println!("It's 2"), 
    //     3 => println!("It's 3"),
    //     _ => println!("It's something else"), 
    // }

    // Match vs "if", "else"
    // Match will be checked by the compiler:
    // - if a new posibility is added, you will be notified
    // "if", "else" is NOT checked by the compiler:
    // - if a new posibility is added, your code may contain a bug

    // Example decisions with Match
    // let my_name = "Nick";
    // match my_name {
    //     "Nick" => println!("That is my name"),
    //     "Bob" => println!("That is NOT my name"),
    //     "Alice" => println!("Oh, hello Alice"),
    //     _ => println!("First time here? Welcome!")
    // }

    // * Enum
    // Enumeration to work with data, that can be multiple posibilities
    // Each posibility is called a "variant"
    // Enum provides information about your program to the compiler
    // More robust programs

    // Example
    // enum Color {
    //     Red,
    //     Green,
    //     Blue
    // }
    // fn print_color (color: Color) {
    //     match color {
    //         Color::Red => println!("red"),
    //         Color::Green => println!("green"),
    //         Color::Blue => println!("blue")
    //     }
    // }
    // print_color(Color::Blue);

}
