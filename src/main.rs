//!
//! # Lexical Analyzer
//! #### Diego Eduardo Hernandez Cadena - A00834015
//!  
//! Implements a lexical analyzer using the plex crate. It reads the specified input file, tries to parse it, and prints a list of the resulting tokens.
//!
//! ## Adding more keywords and operators
//! To add a keyword or operator, it has to be added to the `Keyword` or `Operator` enum, and a mapping for it has to be included in the corresponding static map.
//! Additionally, it may be needed to modify the corresponding regex inside the lexer.
//!
//! ## Adding more tokens
//! To add a new token, an entry has to be added to the `Token` enum, and a new rule has to be added to the `lexer!` macro.

use phf::phf_map;
use plex::lexer;
use std::{fs::File, io::Read};

const INPUT_FILE_PATH: &str = "./input/Lex_InputFile.txt";

/// Supported keywords that the lexer will parse
#[derive(Debug, Clone)]
enum Keyword {
    While,
    For,
    Fn,
    If,
}

/// Supported operators that the lexer will parse
#[derive(Debug, Clone)]
enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,

    // Comparison
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Logical
    And,
    Or,
    Not,
}

/// Supported tokens that the lexer will parse
#[allow(dead_code)]
#[derive(Debug)]
enum Token {
    Integer(i64),
    Whitespace,
    Identifier(String),
    Decimal(f64),
    Keyword(Keyword),
    Operator(Operator),
}

/// List of supported keywords and operators
static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "while" => Keyword::While,
    "for" => Keyword::For,
    "fn" => Keyword::Fn,
    "if" => Keyword::If,
};

static OPERATORS: phf::Map<&'static str, Operator> = phf_map! {
    // Arithmetic
    "+" => Operator::Plus,
    "-" => Operator::Minus,
    "*" => Operator::Multiply,
    "/" => Operator::Divide,
    "%" => Operator::Modulo,

    // Comparison
    "==" => Operator::EqualEqual,
    "!=" => Operator::NotEqual,
    "<" => Operator::Less,
    "<=" => Operator::LessEqual,
    ">" => Operator::Greater,
    ">=" => Operator::GreaterEqual,

    // Logical
    "&&" => Operator::And,
    "||" => Operator::Or,
    "!" => Operator::Not,
};

/// Reads the contents of the specified file
fn extract_file_contents(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    contents
}

fn parse_keyword(s: &str) -> Option<Keyword> {
    return KEYWORDS.get(s).cloned();
}

fn parse_operator(s: &str) -> Option<Operator> {
    return OPERATORS.get(s).cloned();
}

lexer! {
    fn take_token(tok: 'a) -> Token;

    r"[ \n\t]+" => Token::Whitespace,
    r"-?[0-9]+\.[0-9]+" => Token::Decimal(tok.parse().unwrap()),
    r"-?[0-9]+" => Token::Integer(tok.parse().unwrap()),
    r"==|!=|<=|>=|\&\&|\|\||[+\\\-*\/%<>!]" => {
        if let Some(op) = parse_operator(tok) {
            Token::Operator(op)
        } else {
            panic!("Unknown operator: {}", tok);
        }
    }
    "[a-zA-Z_][a-zA-Z0-9_]*" => {
        if let Some(keyword) = parse_keyword(tok) {
            Token::Keyword(keyword)
        } else {
            Token::Identifier(String::from(tok))
        }
    }
}

fn extract_tokens(input: String) -> Vec<Token> {
    let mut remaining = input.as_str();
    let mut tokens: Vec<Token> = Vec::new();

    while let Some((token, new_remaining)) = take_token(remaining) {
        if let Some(prev_token) = tokens.last() {
            if !matches!(prev_token, Token::Whitespace) && !matches!(token, Token::Whitespace) {
                panic!(
                    "Missing separator between tokens {:?} and {:?}",
                    prev_token, token
                )
            }
        }

        tokens.push(token);
        remaining = new_remaining;
    }

    return tokens;
}

fn main() {
    let s = extract_file_contents(INPUT_FILE_PATH);

    let tokens = extract_tokens(s);

    for tok in tokens {
        if matches!(tok, Token::Whitespace) {
            continue;
        }
        println!("Token: {:?}", tok);
    }
}
