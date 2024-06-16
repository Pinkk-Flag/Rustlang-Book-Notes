fn main() {
    let mut counter = 0;

    let cheese = "Yummy!";

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result in {}", result);
    println!("{}", cheese);
}