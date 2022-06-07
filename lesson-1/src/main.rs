
use std::env;
use lesson_1::{
    dungeon_data::{
        Access, 
        Identity, 
        Items, 
        Money,
        Purse,
        Conversion
    },
    command_parser::{
        Token,
        tokenize
    }
};

fn main() {
    let command = "dig up; dig down; dig north {something goes here}; dig south; if mary then give 10 gold! to mary Otherwise! if @go west; then if go east then if go south then spend 10 gold";

    let tokens = tokenize(command);
    match tokens {
        Some(toks) => println!("{:#?}", toks),
        None => {}
    }

    

}





