### Rust Tutorial

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
