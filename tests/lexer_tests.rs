use compiler_project_tc3002_b::*;

#[test]
fn test_parse_keyword() {
    assert_eq!(parse_keyword("while"), Some(Keyword::While));
    assert_eq!(parse_keyword("for"), Some(Keyword::For));
    assert_eq!(parse_keyword("fn"), Some(Keyword::Fn));
    assert_eq!(parse_keyword("if"), Some(Keyword::If));
    assert_eq!(parse_keyword("unknown_keyword"), None);
}

#[test]
fn test_parse_operator() {
    assert_eq!(parse_operator("+"), Some(Operator::Plus));
    assert_eq!(parse_operator("!="), Some(Operator::NotEqual));
    assert_eq!(parse_operator("<="), Some(Operator::LessEqual));
    assert_eq!(parse_operator("||"), Some(Operator::Or));
    assert_eq!(parse_operator("not_an_op"), None);
}

#[test]
fn test_extract_tokens_basic() {
    let input = "fn myFunc 42 + 3.14 while".to_string();
    let tokens = extract_tokens(input)
        .into_iter()
        .filter(|t| !matches!(t, Token::Whitespace))
        .collect::<Vec<_>>();

    assert_eq!(
        tokens,
        vec![
            Token::Keyword(Keyword::Fn),
            Token::Identifier("myFunc".to_string()),
            Token::Integer(42),
            Token::Operator(Operator::Plus),
            Token::Decimal(3.14),
            Token::Keyword(Keyword::While),
        ]
    );
}

#[test]
#[should_panic(expected = "Missing separator between tokens")]
fn test_missing_separator_panic() {
    // Should panic due to missing whitespace between Integer and Operator
    let input = "42+3".to_string();
    let _ = extract_tokens(input);
}

#[test]
#[should_panic(expected = "Unrecognized token starting at position")]
fn test_unsupported_token_panic() {
    // Should panic due to missing whitespace between Integer and Operator
    let input = "print $x + 3".to_string();
    let _ = extract_tokens(input);
}

#[test]
fn test_token_with_multiple_whitespace() {
    let input = "if    x   !=  10".to_string();
    let tokens = extract_tokens(input)
        .into_iter()
        .filter(|t| !matches!(t, Token::Whitespace))
        .collect::<Vec<_>>();

    assert_eq!(
        tokens,
        vec![
            Token::Keyword(Keyword::If),
            Token::Identifier("x".to_string()),
            Token::Operator(Operator::NotEqual),
            Token::Integer(10),
        ]
    );
}

#[test]
fn test_unrecognized_identifier_is_treated_as_identifier() {
    let input = "foobar".to_string();
    let tokens = extract_tokens(input)
        .into_iter()
        .filter(|t| !matches!(t, Token::Whitespace))
        .collect::<Vec<_>>();

    assert_eq!(tokens, vec![Token::Identifier("foobar".to_string())]);
}

#[test]
fn test_integer_and_decimal_literals() {
    let input = "100 -42 3.1415".to_string();
    let tokens = extract_tokens(input)
        .into_iter()
        .filter(|t| !matches!(t, Token::Whitespace))
        .collect::<Vec<_>>();

    assert_eq!(
        tokens,
        vec![
            Token::Integer(100),
            Token::Integer(-42),
            Token::Decimal(3.1415),
        ]
    );
}
