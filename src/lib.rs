//! # Lexical Analyzer
//! #### Diego Eduardo Hernandez Cadena - A00834015
//!  
//! Implements a lexical analyzer using the `plex` crate. It reads the specified input file, tries to parse it, and prints a list of the resulting tokens.
//!
//! ## Usage
//! To use the lexical analyzer, set the `INPUT_FILE_PATH` constant to the desired location and run `cargo run` to execute the program.
//!
//! ## Adding more keywords and operators
//! To add a keyword or operator:
//! - Add it to the `Keyword` or `Operator` enum.
//! - Add an entry to the corresponding `phf_map!` static map.
//! - If necessary, update the regular expressions in the `lexer!` macro.
//!
//! ## Adding more tokens
//! To add a new token:
//! - Add a new variant to the `Token` enum.
//! - Add a matching rule in the `lexer!` macro that maps input to the new token.

use phf::phf_map;
use plex::lexer;
use std::{fs::File, io::Read};

/// Represents supported keywords that the lexer can recognize
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    While,
    For,
    Fn,
    If,
    Else,
}

/// Represents supported operators in the language
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,
    PlusEqual,
    MinusEqual,
    MultiplyEqual,
    DivideEqual,
    Modulo,
    Equal,

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

/// Represents all possible tokens that can be produced by the lexer
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    /// Integer literals (e.g. `42`)
    Integer(i64),
    /// Whitespace characters (e.g. space, tab, newline)
    Whitespace,
    /// Identifiers (e.g. variable or function names)
    Identifier(String),
    /// Floating point literals (e.g. `3.14`)
    Decimal(f64),
    /// Language keywords (e.g. `while`, `if`)
    Keyword(Keyword),
    /// Operators (e.g. `+`, `!=`)
    Operator(Operator),
}

/// Mapping of keyword strings to `Keyword` enum values
static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "while" => Keyword::While,
    "for" => Keyword::For,
    "fn" => Keyword::Fn,
    "if" => Keyword::If,
    "else" => Keyword::Else,
};

/// Mapping of operator strings to `Operator` enum values
static OPERATORS: phf::Map<&'static str, Operator> = phf_map! {
    // Arithmetic
    "+" => Operator::Plus,
    "+=" => Operator::PlusEqual,
    "-" => Operator::Minus,
    "-=" => Operator::MinusEqual,
    "*" => Operator::Multiply,
    "*=" => Operator::MultiplyEqual,
    "/" => Operator::Divide,
    "/=" => Operator::DivideEqual,
    "%" => Operator::Modulo,
    "=" => Operator::Equal,

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

/// Reads the contents of the file at the specified path
///
/// # Panics
/// Panics if the file cannot be opened or read.
pub fn extract_file_contents(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("Failed to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    contents
}

/// Tries to match a string slice to a known `Keyword`
pub fn parse_keyword(s: &str) -> Option<Keyword> {
    KEYWORDS.get(s).cloned()
}

/// Tries to match a string slice to a known `Operator`
pub fn parse_operator(s: &str) -> Option<Operator> {
    OPERATORS.get(s).cloned()
}

// Lexer definition that converts input strings into tokens
lexer! {
    fn take_token(tok: 'a) -> Token;

    r"[ \n\t]+" => Token::Whitespace,
    r"-?[0-9]+\.[0-9]+" => Token::Decimal(tok.parse().unwrap()),
    r"-?[0-9]+" => Token::Integer(tok.parse().unwrap()),
    r"\+=|-=|\*=|/=|==|!=|<=|>=|\&\&|\|\||[+\\\-*\/%<>!=]" => {
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

/// Extracts all tokens from the input string using the lexer
///
/// # Panics
/// Panics if two non-whitespace tokens are found without a valid separator between them.
pub fn extract_tokens(input: String) -> Vec<Token> {
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

    if !remaining.trim().is_empty() {
        let position = input.len() - remaining.len();
        panic!(
            "Unrecognized token starting at position {}: {:?}",
            position, remaining
        );
    }

    tokens
}

/// Main function: reads input, tokenizes it, and prints each token (excluding whitespace)
pub fn run(input_file: &str) {
    let s = extract_file_contents(input_file);
    let tokens = extract_tokens(s);

    for tok in tokens {
        if matches!(tok, Token::Whitespace) {
            continue;
        }
        println!("Token: {:?}", tok);
    }
}
