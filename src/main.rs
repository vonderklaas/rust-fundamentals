

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

    // * impl, self keywords (implementation)
    // Allows you to implement functionality on specific enumerations and structs
    // Grealy ehnances organization of your code and makes programs easier to follow

    // Example
    // struct Temperature {
    //     degrees_f: f64
    // }
    // impl Temperature {
    //     // Self === Temperature
    //     fn freezing() -> Self {
    //         Self {
    //             degrees_f: 32.0
    //         }
    //     }
    //     fn show_temp(&self) {
    //         println!("{:?} degrees F", self.degrees_f)
    //     } 
    // }
    // let hot = Temperature { degrees_f: 99.9 };
    // hot.show_temp();
    // let cold = Temperature::freezing();
    // cold.show_temp();

    // Example
    // enum Color {
    //     Brown,
    //     Red
    // }
    // impl Color {
    //     fn print(&self) {
    //         match self {
    //             Color::Brown => println!("Color is brown"),
    //             Color::Red => println!("Color is red"),
    //         }
    //     }
    // }
    // struct Dimensions {
    //     width: f64,
    //     height: f64,
    //     depth: f64
    // }
    // impl Dimensions {
    //     fn print(&self) {
    //         println!("width: {:?}", self.width);
    //         println!("height: {:?}", self.height);
    //         println!("depth: {:?}", self.depth);
    //     }
    // }
    // struct ShippingBox {
    //     color: Color,
    //     weight: f64,
    //     dimensions: Dimensions
    // }
    // impl ShippingBox {
    //     fn new(weight: f64, color: Color,dimensions: Dimensions) -> Self {
    //         Self {
    //             weight,
    //             color,
    //             dimensions
    //         }
    //     }
    //     fn print(&self) {
    //         self.color.print();
    //         self.dimensions.print();
    //     }
    // }

    // let small_dimensions: Dimensions = Dimensions {
    //     width: 1.0,
    //     height: 2.0,
    //     depth: 3.0
    // };
    // let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    // small_box.print();
    
    // * Vectors (Data Structure)
    // Multiple pieces of data, data of the same type
    // Used for lists of information
    // Can add, remove, and traverse the entries

    // Examples

    // Alternatives
    // let my_numbers = vec![1, 2, 3];
    // let mut my_numbers = Vec::new();
    // my_numbers.push(1);
    // my_numbers.push(2);
    // my_numbers.push(3);
    // my_numbers.pop();
    // my_numbers.len(); // 2
    // let two = my_numbers[1];
    // println!("{:?}", two);

    // Iterating
    // let my_numbers = vec![1, 2, 3];
    // for num in my_numbers {
    //     println!("{:?}", num);
    // }

    // Recap
    // Vectors contain multiple pieces of similar data
    // Data can be added or removed
    // The vec! macro can be used to create vectors
    // Use for..in to iterate through items of a vector

    // Example
    // struct Test {
    //     score: i32
    // }
    // let my_scores = vec![
    //     Test {
    //         score: 90
    //     },
    //     Test {
    //         score: 77
    //     },
    //     Test {
    //         score: 83
    //     }
    // ];
    // let mut total: i32 = 0;
    // for score in my_scores {
    //     total += score.score;
    // }
    // println!("Total is {:?}", total);
    // println!("Average is {:?}", total / 3);

    // Example
    // let numbers = vec![10, 20, 30, 40];
    // for number in numbers {

    //     // "if", "else"
    //     // if number == 30 {
    //     //     println!("Thirty");
    //     // } else {
    //     //     println!("{:?}", number);
    //     // }

    //     // "match"
    //     match number {
    //         30 => println!("Thirty"),
    //         _ => println!("{:?}", number)
    //     }
    // }

    // * Strings
    // Strings are used to store text information
    // Two commonly string types: String (owned) and &str (borrowed)
    // Must use an owned String to store in a struct
    // Use &str for function parameters

    // Example (pass it to a function)
    // fn print_it(data: &str) {
    //     println!("{:?}", data);
    // }
    // // Automaticly borrowed
    // print_it("Hello World");
    // // Create our own string
    // let owned_string = "owned_string".to_owned();
    // let another_string = String::from("Hello World");
    // print_it(&owned_string);
    // print_it(&another_string);

    // Example (struct)
    // struct Employee {
    //     // Doesn't work
    //     // name: &str,
    //     name: String
    // }
    // let emp_name = String::from("Nick");
    // let emp = Employee {
    //     name: emp_name
    // };

    // Recap
    // Strings are automaticly borrowed
    // Use .to_owned() or String::from() to create an owned String
    // Use &str for function parameters
    // Use String for struct fields 

    // Example
    // struct Person {
    //     age: i32,
    //     name: String,
    //     color: String
    // }

    // let people = vec![
    //     Person {
    //         age: 5,
    //         name: String::from("Nick"),
    //         color: String::from("Blue")
    //     },
    //     Person {
    //         age: 7,
    //         name: String::from("John"),
    //         color: String::from("Red")
    //     },
    //     Person {
    //         age: 22,
    //         name: String::from("Sally"),
    //         color: String::from("Green")
    //     }
    // ];
    // fn print_name(name: &str) {
    //     println!("name = {:?}", name);
    // }
    // fn print_color(color: &str) {
    //     println!("color = {:?}", color);
    // }
    // for person in people {
    //     if person.age <= 10 {
    //         print_name(&person.name);
    //         print_color(&person.color);
    //     }
    // }

    // * Type Annotations 
    // They are required for funciton signatures
    // Types are usually inferred by the compiler, but can also be specified in code, by explicit type annotation

    // Example of type annotation for vectors
    // let letters = vec!['a', 'b', 'c'];
    // let numbers = vec![1, 2, 3];

    // * Advanced Match
    // enum Discount {
    //     Percent(i32),
    //     Flat(i32)
    // }

    // struct Ticket {
    //     event: String,
    //     price: i32
    // }

    // let n = 3;
    // match n {
    //     3 => println!("Three"),
    //     // _ => println!("Not Three")
    //     other  => println!("number: {:?}", other)
    // }

    // let flat = Discount::Flat(2);
    // match flat {
    //     Discount::Flat(2) => println!("flat 2"),
    //     Discount::Flat(amount) => println!("flat discount of {:?}", amount),
    //     _ => println!("no discount")
    // }

    // let concert = Ticket {
    //     event: "concert".to_owned(),
    //     price: 50
    // };
    // match concert {
    //     Ticket {
    //         price, ..
    //     } => println!("price = {:?}", price)
    // }

    // Example
    // enum Ticket {
    //     Backstage(f64, String),
    //     Standard(f64),
    //     Vip(f64, String),
    // }

    // let tickets = vec![
    //     Ticket::Backstage(50.0, "Billy".to_owned()),
    //     Ticket::Standard(25.0),
    //     Ticket::Vip(100.0, "Nick".to_owned())
    // ];

    // for ticket in tickets {
    //     match ticket {
    //         Ticket::Backstage(price, name) => println!("Backstage ticket for {:?} costs {:?}", name, price),
    //         Ticket::Standard(price) => println!("Standard ticket costs {:?}", price),
    //         Ticket::Vip(price, name) => println!("VIP ticket for {:?} costs {:?}", name, price)
    //     }
    // }

    // * Option
    // Option is a type that represents a value that may or may not exist
    // Option<T> is either Some(T) or None
    // Used in scenarios where data may not be required or is unavailable
    // - Unable to find something
    // - Ran out of items in a list
    // - Form field not filled out

    // Examples
    // Definition of Option
    // enum Option<T> {
    //     Some(T),
    //     None
    // }

    // Example
    // struct Customer {
    //     age: Option<i32>,
    //     email: String
    // }
    // let mark = Customer {
    //     age: Some(30),
    //     email: String::from("mark@gmail.com")
    // };
    // let becky = Customer {
    //     age: None,
    //     email: String::from("becky@gmail.com")
    // };
    // match becky.age {
    //     Some(age) => println!("age = {:?}", age),
    //     None => println!("No age")
    // }

    // * Documentation
    // enum Color {
    //     Red,
    //     Blue
    // }
    // /// Represents a person
    // struct Mail {
    //     address: String
    // }
    // /// Sends an email to the specified address
    // fn add(a: i32, b: i32) -> i32 {
    //     return a + b
    // }


}
