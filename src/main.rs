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

    // * Struct
    // Structure is a type that contains multiple pieces of data
    // Each of piece of data within struct must be populated
    // Each piece of data is called a "field"
    // Makes working with data easier, similar data can be grouped together

    // Example One
    // struct ShippingBox {
    //     depth: i32,
    //     width: i32,
    //     height: i32,
    // }
    // let my_box = ShippingBox {
    //     depth: 3,
    //     width: 2,
    //     height: 5
    // };
    // // Access inidividual field (by a dot ".")
    // let tall = my_box.height;
    // println!("the box is {:?} units tall", tall);

    // Example Two
    // struct GroceryItem {
    //     stock: i32,
    //     price: f32
    // }
    // let cereal = GroceryItem {
    //     stock: 10,
    //     price: 2.99
    // };
    // println!("stock: ${:?}", cereal.stock);
    // println!("price: ${:?}", cereal.price);

    // Example Three
    // enum Flavor {
    //     Sparkling,
    //     Sweet,
    //     Fruity
    // }
    // struct Drink {
    //     flavor: Flavor,
    //     fluid_oz: f64
    // }
    // fn print_drink (drink: Drink) {
    //     match drink.flavor {
    //        Flavor::Sparkling => println!("sparkling"),
    //        Flavor::Sweet => println!("sweet"),
    //        Flavor::Fruity => println!("fruity")
    //     }
    //     println!("oz: {:?}", drink.fluid_oz);
    // }
    // let sweet_drink = Drink {
    //     flavor: Flavor::Sweet,
    //     fluid_oz: 6.0
    // };
    // let fruity_drink = Drink {
    //     flavor: Flavor::Fruity,
    //     fluid_oz: 4.0
    // };
    // print_drink(sweet_drink);
    // print_drink(fruity_drink);

    // * Tuples
    // A type of "record", to store data anonymously
    // There is no need for name fields
    // Useful to return pairs of data from functions
    // Can be "destructed" easily into variables

    // Examples
    // enum Access {
    //     Full,
    // }
    // // Returning tuple
    // fn one_two_three() -> (i32, i32, i32) {
    //     (1, 2, 3)
    // }
    // let numbers = one_two_three();
    // // Destructuring
    // let (x, y, z) = one_two_three();
    // // Printing
    // println!("{:?}, {:?}", x, numbers.0); // 1
    // println!("{:?}, {:?}", y, numbers.1); // 2
    // println!("{:?}, {:?}", z, numbers.2); // 3
    // // Creating tuple
    // let (employee, access) = ("Jake", Access::Full);
    // println!("{:?} has full access!", employee);

    // Examples 
    // fn cartesian_coordinate() -> (i32, i32) {
    //     return (10, 10)
    // }
    // let (x, y) = cartesian_coordinate();
    // if y > 5 {
    //     println!("greater than 5")
    // } else if y < 5 {
    //     println!("less than 5")
    // } else {
    //     println!("equal to 5")
    // }

    // * Expressions
    // Rust is an expression based language
    // Most things are evaluated and return some value
    // Expression values coalesce to a single point
    // Can be used for nested logic

    // Example
    // let my_num = 3;
    // Expression
    // let is_lt_5 = if my_num < 5 {
    //     true
    // } else {
    //     false
    // };
    // Shorter Expression
    // let is_lt_5 = my_num < 5;
    // println!("{:?}", is_lt_5);

    // Example
    // let my_num = 3;
    // let message = match my_num {
    //     1 => "Hello!",
    //     _ => "Undefined"
    // };
    // println!("{:?}", message);

    // * Memory, Adresses, Offsets
    // Memory is stored using binary bits: 0 or 1
    // Computer optimized for bytes: 1 byte - 8 contiguous bits
    // All data in memory has an "address"

    // Address is used to locate data
    // Address is always the same, only data changes
    // Usually we don't utilize addresses directly, variables handle most of the work

    // Items can be located at an address using an "offset"
    // "Offset" begin at 0, represents the number of bytes away from the original address,
    // usually we don't deal with "offsets" directly, we deal with the indexes instead,
    // and compiler will calculate for us how many bytes are we away from the original address

    // Recap:
    // Memory uses addresses & offsets
    // Addresses are permanent, only data differs
    // Offsets can be used to "index" into some data

    // * Ownership
    // Ownership is what allows Rust to execute code in a performant manner,
    // and helps to ensure that compiled code executes correctly under various circumstances
    
    // Managing Memory
    // Programs must track memory, if they fail to do so, a "leak" occurs
    // Rust utilizes "Owernship" model to manage memory, where the "owner"
    // of memory is responsible for cleaning up the memory
    // Memory can be either "moved" or "borrowed" from the owner

    // Example (memory moved)
    // enum Light {
    //     Bright,
    //     Dull
    // }

    // Error with moving "dull" into function twice
    // fn display_light(light: Light) {
    // -- Borrow instead of moving
    // fn display_light(light: &Light) {
    //     match light {
    //         Light::Bright => println!("Light is bright"),
    //         Light::Dull => println!("Light is dull"),
    //     }
        // Invisibly remove "dull" from memory
    // }

    // let dull = Light::Dull;
    // Since we call display_light twice, the program will not compile
    // display_light(dull);
    // display_light(dull);
    // -- Borrow instead of moving
    // display_light(&dull);
    // display_light(&dull);

    // Recap
    // Memory must be managed in some way to prevent leaks
    // Rust uses "ownership" model to accomplish memory management
    // - The "owner" of data must clean up the memory
    // - This occurs automatically at the end of the scope
    // Default behaviour is to "move" memory to a new owner
    // - Use an ampersand (&) to allow code to "borrow" memory

    // Code Example
    // struct Book {
    //     pages: i32,
    //     rating: i32,
    // }
    // fn display_page_count(book: &Book) {
    //     println!("pages = {:?}", book.pages);
    // }
    // fn display_page_rating(book: &Book) {
    //     println!("rating = {:?}", book.rating);
    // }
    // let book = Book {
    //     pages: 200,
    //     rating: 5
    // };
    // display_page_count(&book);
    // display_page_rating(&book);

    // Example
    // struct GroceryItem {
    //     quantity: i32,
    //     id: i32
    // }
    // fn display_quantity(item: &GroceryItem) {
    //     println!("quantity = {:?}", item.quantity);
    // }
    // fn display_id(item: &GroceryItem) {
    //     println!("id = {:?}", item.id);
    // }
    // let tomato: GroceryItem = GroceryItem {
    //     quantity: 100,
    //     id: 42
    // };
    // display_quantity(&tomato);
    // display_id(&tomato);


}
