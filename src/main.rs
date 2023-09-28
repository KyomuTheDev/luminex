use args::{Command, LuminexCLIArgs};
use clap::Parser;
use std::fs::read_to_string;

mod args;
mod ast;
mod lexer;
mod utils;

fn main() -> Result<(), String> {
    let args = LuminexCLIArgs::parse();

    match args.cmd {
        Command::Build => {}

        Command::Fix => {}

        Command::Analyze => {}

        Command::Init => {}

        Command::Test { test_file } => {
            let data = match read_to_string(test_file) {
                Ok(data) => data,
                Err(e) => return Err(e.to_string()),
            };

            // let tree = {
            //     let mut tokenizer = tokenizer::Tokenizer::new(&data);
            //     let mut data: Vec<Token> = vec![];
                
            //     loop {
            //         let token = tokenizer.next();

            //         if token.kind == Kind::EOF { break; }

            //         data.push(token);
            //     }

            //     data
            // };



            let tree = ast::Parser::new(&data).expr().unwrap();

            println!("{:#?}", tree);
        }
    }

    Ok(())
}