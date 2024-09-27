use super::*;

#[test]
fn test_multi_digit_constant() {
    let input = r#"
        int main(void) {
            // test case w/ multi-digit constant
            return 100;
        }
    "#;
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 2),
        ExpectedToken(TokenType::Identifier, "main", 2),
        ExpectedToken(TokenType::LeftParen, "(", 2),
        ExpectedToken(TokenType::Void, "void", 2),
        ExpectedToken(TokenType::RightParen, ")", 2),
        ExpectedToken(TokenType::LeftBrace, "{", 2),
        ExpectedToken(TokenType::Return, "return", 4),
        ExpectedToken(TokenType::Integer, "100", 4),
        ExpectedToken(TokenType::Semicolon, ";", 4),
        ExpectedToken(TokenType::RightBrace, "}", 5),
        ExpectedToken(TokenType::Eof, "", 6),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_new_lines() {
    let input = r#"
        int
        main
        (
        void
        )
        {
        return
        0
        ;
        }
    "#;
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 2),
        ExpectedToken(TokenType::Identifier, "main", 3),
        ExpectedToken(TokenType::LeftParen, "(", 4),
        ExpectedToken(TokenType::Void, "void", 5),
        ExpectedToken(TokenType::RightParen, ")", 6),
        ExpectedToken(TokenType::LeftBrace, "{", 7),
        ExpectedToken(TokenType::Return, "return", 8),
        ExpectedToken(TokenType::Integer, "0", 9),
        ExpectedToken(TokenType::Semicolon, ";", 10),
        ExpectedToken(TokenType::RightBrace, "}", 11),
        ExpectedToken(TokenType::Eof, "", 12),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_no_new_lines() {
    let input = r#"int main(void){return 0;}"#;
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 1),
        ExpectedToken(TokenType::Identifier, "main", 1),
        ExpectedToken(TokenType::LeftParen, "(", 1),
        ExpectedToken(TokenType::Void, "void", 1),
        ExpectedToken(TokenType::RightParen, ")", 1),
        ExpectedToken(TokenType::LeftBrace, "{", 1),
        ExpectedToken(TokenType::Return, "return", 1),
        ExpectedToken(TokenType::Integer, "0", 1),
        ExpectedToken(TokenType::Semicolon, ";", 1),
        ExpectedToken(TokenType::RightBrace, "}", 1),
        ExpectedToken(TokenType::Eof, "", 1),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_with_spaces() {
    let input = r#"   int   main    (  void)  {   return  0 ; }"#;
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 1),
        ExpectedToken(TokenType::Identifier, "main", 1),
        ExpectedToken(TokenType::LeftParen, "(", 1),
        ExpectedToken(TokenType::Void, "void", 1),
        ExpectedToken(TokenType::RightParen, ")", 1),
        ExpectedToken(TokenType::LeftBrace, "{", 1),
        ExpectedToken(TokenType::Return, "return", 1),
        ExpectedToken(TokenType::Integer, "0", 1),
        ExpectedToken(TokenType::Semicolon, ";", 1),
        ExpectedToken(TokenType::RightBrace, "}", 1),
        ExpectedToken(TokenType::Eof, "", 1),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_with_tabs() {
    let input = "int\tmain\t(\tvoid)\t{\treturn\t0\t;\t}";
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 1),
        ExpectedToken(TokenType::Identifier, "main", 1),
        ExpectedToken(TokenType::LeftParen, "(", 1),
        ExpectedToken(TokenType::Void, "void", 1),
        ExpectedToken(TokenType::RightParen, ")", 1),
        ExpectedToken(TokenType::LeftBrace, "{", 1),
        ExpectedToken(TokenType::Return, "return", 1),
        ExpectedToken(TokenType::Integer, "0", 1),
        ExpectedToken(TokenType::Semicolon, ";", 1),
        ExpectedToken(TokenType::RightBrace, "}", 1),
        ExpectedToken(TokenType::Eof, "", 1),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_invalid_at_sign() {
    let input = r#"
        int main(void) {
            return 0@1;
        }
    "#;
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 2),
        ExpectedToken(TokenType::Identifier, "main", 2),
        ExpectedToken(TokenType::LeftParen, "(", 2),
        ExpectedToken(TokenType::Void, "void", 2),
        ExpectedToken(TokenType::RightParen, ")", 2),
        ExpectedToken(TokenType::LeftBrace, "{", 2),
        ExpectedToken(TokenType::Return, "return", 3),
        ExpectedToken(TokenType::Integer, "0", 3),
        ExpectedToken(TokenType::Error, "Unexpected character @", 3),
        ExpectedToken(TokenType::Integer, "1", 3),
        ExpectedToken(TokenType::Semicolon, ";", 3),
        ExpectedToken(TokenType::RightBrace, "}", 4),
        ExpectedToken(TokenType::Eof, "", 5),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_invalid_backslash() {
    let input = r#"/* A single backslash is not a valid token. */
    \"#;
    let tests = vec![
        ExpectedToken(TokenType::Error, "Unexpected character \\", 2),
        ExpectedToken(TokenType::Eof, "", 2),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_invalid_backtick() {
    let input = r#"/* A backtick is not a valid token. */
    `"#;
    let tests = vec![
        ExpectedToken(TokenType::Error, "Unexpected character `", 2),
        ExpectedToken(TokenType::Eof, "", 2),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_invalid_identifier() {
    let input = r#"/* '1foo' is not a valid token, because identifier can't start with digits. */
        int main(void) {
        return 1foo;
    }"#;
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 2),
        ExpectedToken(TokenType::Identifier, "main", 2),
        ExpectedToken(TokenType::LeftParen, "(", 2),
        ExpectedToken(TokenType::Void, "void", 2),
        ExpectedToken(TokenType::RightParen, ")", 2),
        ExpectedToken(TokenType::LeftBrace, "{", 2),
        ExpectedToken(TokenType::Return, "return", 3),
        ExpectedToken(TokenType::Error, "Unexpected character f", 3),
        ExpectedToken(TokenType::Identifier, "foo", 3),
        ExpectedToken(TokenType::Semicolon, ";", 3),
        ExpectedToken(TokenType::RightBrace, "}", 4),
        ExpectedToken(TokenType::Eof, "", 4),
    ];
    run_scanner_tests(input, tests);
}

#[test]
fn test_invalid_identifier_2() {
    let input = r#"int main(void) {
        return @b;
    }"#;
    let tests = vec![
        ExpectedToken(TokenType::Int, "int", 1),
        ExpectedToken(TokenType::Identifier, "main", 1),
        ExpectedToken(TokenType::LeftParen, "(", 1),
        ExpectedToken(TokenType::Void, "void", 1),
        ExpectedToken(TokenType::RightParen, ")", 1),
        ExpectedToken(TokenType::LeftBrace, "{", 1),
        ExpectedToken(TokenType::Return, "return", 2),
        ExpectedToken(TokenType::Error, "Unexpected character @", 2),
        ExpectedToken(TokenType::Identifier, "b", 2),
        ExpectedToken(TokenType::Semicolon, ";", 2),
        ExpectedToken(TokenType::RightBrace, "}", 3),
        ExpectedToken(TokenType::Eof, "", 3),
    ];
    run_scanner_tests(input, tests);
}
