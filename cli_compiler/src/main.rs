use args::{Command, LuminexCLIArgs};
use clap::Parser;

#[macro_use]
extern crate enum_index_derive;

mod args;
mod ast;
mod tokenizer;
mod utils;

fn main() -> Result<(), String> {
    let args = LuminexCLIArgs::parse();

    println!("{}", utils::get_error_message(3, 1, 1, String::from("testing")));

    match args.cmd {
        Command::Build => {}

        Command::Fix => {}

        Command::Analyze => {}

        Command::Init => {}

        Command::Test { test_file } => {}
    }

    Ok(())
}

// struct Test {
//     position: usize,
//     chars: Vec<char>,
// }

// impl Test {
//     pub fn new(input: &str) -> Test {
//         Test {
//             position: 0,
//             chars: input.chars().collect(),
//         }
//     }

//     fn run(&mut self) -> () {
//         loop {
//             let c = match self.peek() {
//                 Some(c) => c,
//                 None => break,
//             };
        
//             if c.is_alphanumeric() {
//                 println!("Character is alphanumeric!");
//             }
        
//             self.position += 1;
        
//             println!("{}", self.position);
        
//             if self.position > 5 {
//                 break;
//             }
//         }
//     }
    
//     fn peek(&self) -> Option<&char> {
//         self.chars.get(self.position)
//     }
// }

// fn main() {
//     let mut t = Test::new("Titties");
    
//     t.run()
// }