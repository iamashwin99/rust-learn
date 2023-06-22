
const kk: u8 = 9; // only const can be global
// let superconst = 9; // error: expected item, found keyword `let`
fn main() {
    let a: u8 =b'a';

    let b: u8 = 9_0;
    let f1=0.1;
    let f2 = 0.2;


    println!("Hello, {a} {b}!");
    let k = f1+f2;
    // f2+=f1;  // error : cannot assign twice to immutable variable
    println!("{k}");

    let mut f3 = 0.2;
    f3+=f1;
    println!("{f3}");


    // shadowing
    let f4 = 0.2;
    let f4 = f4 + f1;
    println!("shadowing : {f4}");
    // this works because we are creating a new variable with the same name as the previous one
    // and the previous one is no longer accessible after this point
    // this is called shadowing

}
