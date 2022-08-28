pub mod lib;
use crate::lib::*;
use std::io;
fn main() {
    loop {
        println!("Please input a number to parse:");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let mut parser = DecimalParser::new();
        number = (number.trim()).to_string();
        parser.fully_parse(number.clone());
        if parser.valid_number() {
            println!("✅ \"{}\" is a valid number", number)
        } else {
            println!("❌ \"{}\" is not a valid number", number)
        }
    }
}
