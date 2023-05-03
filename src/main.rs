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

}
