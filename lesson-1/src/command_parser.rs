
use regex_lexer::*;

use crate::dungeon_data::{
    Items,
    Money,
    Identity
};

use std::{
    result::Result,
    error::Error 
};



#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn from_string(dir: String) -> Option<Direction> {
        match dir.to_lowercase().as_str() {
            "north" => Some(Self::North),
            "south" => Some(Self::South),
            "east" => Some(Self::East),
            "west" => Some(Self::West),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Go,
    Dig,
    Take,
    Drop,
    Spend,
    On,
    With,
    Use,
    Attack,
    Direction(Direction),
    Num(i32),
    Copper,
    Bronze,
    Silver,
    Gold,
    Then,
    If,
    When,
    Where,
    Give,
    To,
    Otherwise,
    Ident(String),
    Punctuation(char)
}

pub fn tokenize(command: &str) -> Option<Vec<Token>> {
    let params: Vec<&str> = command.split(char::is_whitespace).filter(|c| *c != "").collect();
    let lexer = regex_lexer::LexerBuilder::new()
        .token(r"[0-9]+", |num| Some(Token::Num(num.parse().unwrap())))
        .token(r"[a-zA-Z]+[0-9_]*", |s| Some(Token::Ident(s.to_owned())))
        .token(r"go",|_| Some(Token::Go))
        .token(r"dig",|_| Some(Token::Dig))
        .token(r"spend", |_| Some(Token::Spend))

        .token(r"north", |_| Some(Token::Direction(Direction::North)))
        .token(r"south", |_| Some(Token::Direction(Direction::South)))
        .token(r"east", |_| Some(Token::Direction(Direction::East)))
        .token(r"west", |_| Some(Token::Direction(Direction::West)))
        .token(r"up", |_| Some(Token::Direction(Direction::Up)))
        .token(r"down", |_| Some(Token::Direction(Direction::Down)))

        .token(r"copper", |_| Some(Token::Copper))
        .token(r"bronze", |_| Some(Token::Bronze))
        .token(r"silver", |_| Some(Token::Silver))
        .token(r"gold", |_| Some(Token::Gold))


        .token(r"otherwise", |_| Some(Token::Otherwise))
        .token(r"then", |_| Some(Token::Then))
        .token(r"if", |_| Some(Token::If))
        .token(r"when", |_| Some(Token::When))
        .token(r"where", |_| Some(Token::Where))
        .token(r"give", |_| Some(Token::Give))
        .token(r"to", |_| Some(Token::To))


        .token(r"[[:punct:]]{1}", |s| Some(Token::Punctuation(s.to_owned().chars().nth(0).unwrap())))
        .token(r"\s+", |_| None)
        .build();

    match lexer {
        Ok(lex) => {
            let toks = lex.tokens(command);
            println!("{:#?}",toks);
            Some(toks.collect())
        },
        Err(err) => None
        
    }

    
}
