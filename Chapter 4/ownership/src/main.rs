fn main() {
    // String's that can be mutated: Strings

    let mut s = String::from("Hello");

    s.push_str(", world! I'm Dario Gerald.");

    println!("{s}");
    // An example of a string literal: let c = "Hello world!"

    // Going into pointers and errors with scope
    let s1 = String::from("hello");
    let s2 = s1.clone()

    println!("{s1}");
}