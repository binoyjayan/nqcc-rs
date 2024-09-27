use super::*;

mod minimal;

#[cfg(test)]
struct ExpectedToken<'a>(TokenType, &'a str, usize);

#[cfg(test)]
fn run_scanner_tests(input: &str, tests: Vec<ExpectedToken>) {
    let mut scanner = Scanner::new(input);
    for (n, tt) in tests.iter().enumerate() {
        println!("[{}] Scanner Test", n);
        let token = scanner.scan_token();
        if token.ttype != tt.0 {
            panic!(
                "tests[{}] - tokentype wrong. expected='{}', got='{}[{}]'",
                n, tt.0, token.ttype, token.lexeme
            );
        }
        if token.lexeme != tt.1 {
            panic!(
                "tests[{}] - literal wrong. expected='{}', got='{}'",
                n, tt.1, token.lexeme
            );
        }
        if token.line != tt.2 {
            panic!(
                "tests[{}] - [lexeme:{}] line wrong. expected='{}', got='{}'",
                n, tt.1, tt.2, token.line
            );
        }
    }
}
