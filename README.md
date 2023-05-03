### Rust Fundamentals

_Rust_ helps developers write fast software that’s memory-efficient. It’s a modern replacement for languages like C++ or C with a focus on code safety and concise syntax.

### Why Rust?

-   High-level language features without performance penalties
-   Program behaviours can be enforced at compile time
-   Built-in dependency management, similar to NPM
-   Quickly growing ecosystem of libraries
-   Friendly & welcoming community

### Technical Rust Goodies

-   First-class multithreading
-   Type tystem
-   Module system makes code separation simple
-   Adding a dependency is 1 line in config file
-   Tooling: generate docs, lint code, auto format

### Topics Covered

-   Why Rust?
-   Data Types
-   Variables
-   Functions
-   Macros
-   Control Flow (if, else)
-   Loops (loop, while)
-   Match Expressions
-   Enum
-   Struct
-   Tuples (destructuring)
-   Expressions
-   Memory, Addresses, Offsets
-   Ownership Model (memory management)
-   impl, self Keywords
-   Vectors
-   Strings

<br/>

### Additional Info

<br/>

Memory, Addresses, Offsets

![MEMORY](https://github.com/garbalau-github/rust-tutorial/blob/main/screenshots/MEMORY.png?raw=true)

Borrowing

The reason we need to do this, is for efficiency and memory management. If we had a data structure
that is large (like several MB) and we would transfer ownership of that structure to different functions
it would require copying all that data each time we use a function, but if you let function borrow
it instead, then it performs much quicker, because data stays in one place and the function can
simply borrow the data and then give it back. There is huge performance implications to having
borrow vs transfering ownership.

```rust
struct Book {
    pages: i32,
    rating: i32,
}
fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}
fn display_page_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}
let book = Book {
    pages: 200,
    rating: 5
};
display_page_count(&book);
display_page_rating(&book);
```

Rust is quite different than JavaScript. JavaScript tries to find variables or objects not in use and automatically clears them from memory. This is called garbage collection. The language abstracts the developer from thinking about manual memory management. With Rust, developers have more control over memory allocation without it being as painful as C++.

Rust uses a relatively unique memory management approach that incorporates the idea of memory ‘ownership’. Basically, Rust keeps track of who can read and write to memory. It knows when the program is using memory and immediately frees the memory once it is no longer needed. It enforces memory rules at compile-time, making it virtually impossible to have runtime memory bugs. You do not need to manually keep track of memory. The compiler takes care of it.

Implementation

Example 1

```rust
struct Temperature {
    degrees_f: f64
}
impl Temperature {
    // Self === Temperature
    fn freezing() -> Self {
        Self {
            degrees_f: 32.0
        }
    }
    fn show_temp(&self) {
        println!("{:?} degrees F", self.degrees_f)
    }
}
let hot = Temperature { degrees_f: 99.9 };
hot.show_temp();
let cold = Temperature::freezing();
cold.show_temp();
```

Example 2

```rust
enum Color {
    Brown,
    Red
}
impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Color is brown"),
            Color::Red => println!("Color is red"),
        }
    }
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64
}
impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}
impl ShippingBox {
    fn new(weight: f64, color: Color,dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
    }
}

let small_dimensions: Dimensions = Dimensions {
    width: 1.0,
    height: 2.0,
    depth: 3.0
};
let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
small_box.print();
```

Strings

```rust
struct Person {
    age: i32,
    name: String,
    color: String
}

let people = vec![
    Person {
        age: 5,
        name: String::from("Nick"),
        color: String::from("Blue")
    },
    Person {
        age: 7,
        name: String::from("John"),
        color: String::from("Red")
    },
    Person {
        age: 22,
        name: String::from("Sally"),
        color: String::from("Green")
    }
];

fn print_name(name: &str) {
    println!("name = {:?}", name);
}

fn print_color(color: &str) {
    println!("color = {:?}", color);
}

for person in people {
    if person.age <= 10 {
        print_name(&person.name);
        print_color(&person.color);
    }
}
```
