use std::collections::HashSet;
#[derive(Debug)]
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
    fn progress_parse(&mut self, next_char: char) {
        *self = match self {
            Self::Start => {
                if next_char == '0' {
                    Self::Zero
                } else if HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9'])
                    .contains(&next_char)
                {
                    Self::Integer
                } else {
                    Self::Failed
                }
            }
            Self::Zero => {
                if next_char == '.' {
                    Self::DecimalPoint
                } else {
                    Self::Failed
                }
            }
            Self::Integer => {
                if next_char == '.' {
                    Self::DecimalPoint
                } else if HashSet::from(['1', '2', '3', '4', '5', '6', '7', '8', '9'])
                    .contains(&next_char)
                {
                    Self::Integer
                } else {
                    Self::Failed
                }
            }
            Self::DecimalPoint | Self::FloatingPoint => {
                if HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
                    .contains(&next_char)
                {
                    Self::FloatingPoint
                } else {
                    Self::Failed
                }
            }
            Self::Failed => Self::Failed,
        }
    }
}
fn main() {
    let nums = vec!["1", "1.0", "45.6", "3.14", ".0", "0..", "3.1.4", "abc", "3.ad", "01"];
    for num in nums {
        let mut decimal_parser = DecimalParser::new();

        for character in num.clone().to_string().chars().collect::<Vec<char>>() {
            decimal_parser.progress_parse(character);
        }
        match decimal_parser {
            DecimalParser::Zero | DecimalParser::Integer | DecimalParser::FloatingPoint => {
                println!("{} is a valid number", num)
            }
            _ => println!("{} is not a valid number", num),
        }
    }
}
