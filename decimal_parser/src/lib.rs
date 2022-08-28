pub enum NextChar {
    Digit(char),
    DecimalPoint,
    Invalid,
}

impl NextChar {
    pub fn from(c: char) -> Self {
        match c {
            '0'..='9' => NextChar::Digit(c),
            '.' => NextChar::DecimalPoint,
            _ => NextChar::Invalid,
        }
    }
}

pub enum DecimalParser {
    Start,
    Zero,
    Integer,
    DecimalPoint,
    FloatingPoint,
    Failed,
}

impl DecimalParser {
    pub fn new() -> Self {
        DecimalParser::Start
    }
    pub fn progress_parse(&mut self, next_char: NextChar) {
        *self = match self {
            Self::Start => match next_char {
                NextChar::Digit('0') => Self::Zero,
                NextChar::Digit(_) => Self::Integer,
                NextChar::Invalid | NextChar::DecimalPoint => Self::Failed,
            },
            Self::Zero => match next_char {
                NextChar::DecimalPoint => Self::DecimalPoint,
                NextChar::Invalid | NextChar::Digit(_) => Self::Failed,
            },
            Self::Integer => match next_char {
                NextChar::DecimalPoint => Self::DecimalPoint,
                NextChar::Digit(_) => Self::Integer,
                NextChar::Invalid => Self::Failed,
            },
            Self::DecimalPoint | Self::FloatingPoint => match next_char {
                NextChar::Digit(_) => Self::FloatingPoint,
                NextChar::DecimalPoint | NextChar::Invalid => Self::Failed,
            },
            Self::Failed => Self::Failed,
        }
    }
    pub fn fully_parse(&mut self, input: String) {
        for c in input.chars().collect::<Vec<char>>() {
            self.progress_parse(NextChar::from(c));
        }
    }
    pub fn valid_number(&self) -> bool {
        match self {
            DecimalParser::Zero | DecimalParser::Integer | DecimalParser::FloatingPoint => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_integers() {
        let values = vec![
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "31415926",
        ];
        for value in values {
            let mut final_result = DecimalParser::new();
            final_result.fully_parse(value.to_string());
            assert_eq!(final_result.valid_number(), true)
        }
    }
    #[test]
    fn valid_floats() {
        let values = vec!["0.1", "1.2", "3.1415926", "0.001"];
        for value in values {
            let mut final_result = DecimalParser::new();
            final_result.fully_parse(value.to_string());
            assert_eq!(final_result.valid_number(), true)
        }
    }

    #[test]
    fn invalid_numbers() {
        let values = vec!["0.1.1", ".2", "7.", ".", "01"];
        for value in values {
            let mut final_result = DecimalParser::new();
            final_result.fully_parse(value.to_string());
            assert_eq!(final_result.valid_number(), false)
        }
    }

    #[test]
    fn invalid_misc() {
        let values = vec!["", "abc", "3.abc", "a.3", "0.a"];
        for value in values {
            let mut final_result = DecimalParser::new();
            final_result.fully_parse(value.to_string());
            assert_eq!(final_result.valid_number(), false)
        }
    }
}
