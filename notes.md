Sebastian
- works on https://helmholtz.cloud/ in desy IT


build project folder: `cargo init hello`

- `cargo run --release` for optimised target
- `cargo run` for debug target

println! is a macro, not a function. note `!` at the end.
Q: where is the macro defined?
A:
missing vscode codelese to auto infere type of variable
\Q: Can you have emaojis in variable names?

- use `_` in numbers for seperation


# ruslings
```shell
git@github.com:rust-lang/rustlings.git
cd
 4917  git checkout tags/5.5.1
 4918  cargo install --force --path .
 4919  rustlings watch
```

work on intro and variables sections

# Functions
`;` are used to suprress output
if you dont have it the last line is returned
even good idea with if
eg:
```rust
fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}
```
and
```rust
pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    if a>b {
        a
    } else {
        b
    }
}
```
work on functios if and types in rustlings

# Ownership
Like RAAI fro C++ but with a twist
- only one owner at a time


`*.clone()` is a deep copy
`*.copy()` is a shallow copy (only for types that implement `Copy` trait)
Q: diff between str: = "asdf" and String::from("asdf")
A: str is a string literal, String is a heap allocated string

## borrowing
- only one mutable borrow at a time
- almost like the address of operator from C ( `&` )
- read only references are passed so no write operations are possible with the reference
-eg:
```rust
fn main() {
    s1 =String::from( "asf" );
    len = calculate_length(&s1) ;
    println!("The length of '{}' is {}.", s1, len);

}
fn calculate_length(s: &String) -> usize {
    s.len()
}
```
- `&mut` for mutable references ( variable must also be mutable)
-" function consumes the variable unless you use a reference ( borrow) it"
Q: why can you have two references to the same variable?

- can have mutable and immutable references at the same time ( lol why?)
eg:
```rust
fn main(){
    s = String::from("asdf");
    let r1 = &s;
    let r2 = &s;
    s.push_str("jkl;"); // error here
    println!("{}, {}", r1, r2);
}
```
![Alt text](image.png)
