fn main() {

    // * Shows the general *gist* of how references work and how they are kind of like "readonly"
    {
        let s1: String = String::from("John");
        let s2: &String = &s1;
        println!("Hello, {s1}!");
        println!("Hello, {s2}!");
    }

}