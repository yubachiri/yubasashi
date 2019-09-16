pub struct Token {
    line_number: i32,
    value: i32,
    text: &'static str,
    literal: &'static str,
}

trait StrToken {
    fn new(line: i32, literal: &'static str) -> Token;
    fn EOF() -> Token;
    fn is_string() -> bool;
    fn get_text(&self) -> &'static str;
}

impl StrToken for Token {
    fn new(line: i32, literal: &'static str) -> Token {
        Token { line_number: line, value: 0, text: "dummy", literal: literal }
    }

    fn EOF() -> Token {
        Self::new(-1, "dummy")
    }

    fn is_string() -> bool {
        true
    }

    fn get_text(&self) -> &'static str {
        self.literal
    }
}

pub fn test() {
    println!("this is test");
}
