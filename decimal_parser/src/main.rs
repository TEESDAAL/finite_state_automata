enum NextChar {
    Digit(u8),
    DecimalPoint,
    Invalid,
}

impl NextChar {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => NextChar::Digit(c as u8 - '0' as u8),
            '.' => NextChar::DecimalPoint,
            _ => NextChar::Invalid,
        }
    }
}

enum DecimalParser {
    Start,
    Zero,
    Integer,
    DecimalPoint,
    FloatingPoint,
    Failed,
}

impl DecimalParser {
    fn new() -> DecimalParser {
        DecimalParser::Start
    }
    fn progress_parse(&mut self, next_char: NextChar) {
        *self = match self {
            Self::Start => match next_char {
                NextChar::Digit(0) => Self::Zero,
                NextChar::Digit(_) => Self::Integer,
                _ => Self::Failed,
            },
            Self::Zero => match next_char {
                NextChar::DecimalPoint => Self::DecimalPoint,
                _ => Self::Failed,
            },
            Self::Integer => match next_char {
                NextChar::DecimalPoint => Self::DecimalPoint,
                NextChar::Digit(_) => Self::Integer,
                _ => Self::Failed,
            },
            Self::DecimalPoint | Self::FloatingPoint => match next_char {
                NextChar::Digit(_) => Self::FloatingPoint,
                _ => Self::Failed,
            },
            Self::Failed => Self::Failed,
        }
    }
}
fn main() {
    let nums = vec![
        "1", "1.0", "45.6", "3.14", "0.866", ".0", "0..", "3.1.4", "abc", "3.ad", "01", "00", "",
    ];
    for num in nums {
        let mut decimal_parser = DecimalParser::new();
        for c in num.clone().to_string().chars().collect::<Vec<char>>() {
            decimal_parser.progress_parse(NextChar::from(c));
        }
        match decimal_parser {
            DecimalParser::Zero | DecimalParser::Integer | DecimalParser::FloatingPoint => {
                println!("✅ \"{}\" is a valid number", num)
            }
            _ => println!("❌\"{}\" is not a valid number", num),
        }
    }
}
