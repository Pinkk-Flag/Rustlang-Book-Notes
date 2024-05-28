fn main() {
    let mut number = 2147483646;

    while number > 0 {
        println!("T Minus-{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!")
}
