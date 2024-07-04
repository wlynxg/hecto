use std::io;
use std::io::Read;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
   enable_raw_mode().unwrap();
    for byte in io::stdin().bytes() {
        let b = byte.unwrap();
        let c = b as char;

        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        }

        if c=='q' {
            disable_raw_mode().unwrap();
            break
        }
        println!("{}",c);
    }
}
