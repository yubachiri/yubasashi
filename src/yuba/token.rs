pub struct Token {
    line_number: i32,
    value: i32,
    text: &'static str,
    literal: &'static str,
}

trait StrToken {
    fn new(line: i32, literal: String) -> Token;
    fn EOF(&self) -> Token;
    fn is_string() -> bool;
    fn get_text() -> String;
}

impl StrToken for Token {
    fn new(line: i32, literal: String) -> Token {
        Token { line_number: 0, value: 0, text: "dummy", literal: "dummy" }
    }
}

pub fn test() {
    println!("this is test");
}
