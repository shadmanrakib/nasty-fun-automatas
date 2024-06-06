// =================
// PARSING
// =================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Letter(char),
    Wildcard,
    OpenParenthesis,
    CloseParenthesis,
    Concatenation,
    Union,
    KleeneQuantifier,
    PositiveQuantifier,
    OptionalQuantifier,
}

#[allow(dead_code)]
#[derive(PartialEq)]
enum Associativity {
    Left,
    Right,
}

impl Token {
    const PRECEDENCES: [(Token, u8, Associativity); 6] = [
        (Token::KleeneQuantifier, 3, Associativity::Left),
        (Token::PositiveQuantifier, 3, Associativity::Left),
        (Token::OptionalQuantifier, 3, Associativity::Left),
        (Token::Wildcard, 3, Associativity::Left),
        (Token::Concatenation, 2, Associativity::Left),
        (Token::Union, 1, Associativity::Left),
    ];
    fn precedence(&self) -> (u8, Associativity) {
        for (token, score, associativity) in Self::PRECEDENCES {
            if *self == token {
                return (score, associativity);
            }
        }
        return (4, Associativity::Left);
    }
    fn has_greater_precedence(&self, other: Token) -> bool {
        let (precedence, _) = self.precedence();
        let (other_precedence, other_associativity) = other.precedence();
        return (precedence > other_precedence)
            | (precedence == other_precedence && other_associativity == Associativity::Left);
    }
}

// const RESERVED = ['\\', '(', ')', '|', '*', '.', '?'];
const NONGROUPING_OPERATORS: [char; 4] = ['|', '*', '?', '+'];
const TWO_OPERAND_OPERATORS: [char; 1] = ['|'];

pub fn parse_re_to_tokens(re: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut escaped = false;
    let chars: Vec<char> = re.chars().collect();
    for i in 0..chars.len() {
        // add implicit concat if no operators between characters,
        // ignore if escaped since it would get handled once before
        // also do not add after two operand operators and before
        // other operators
        if i > 0
            && !escaped
            && !TWO_OPERAND_OPERATORS.contains(&chars[i - 1])
            && !NONGROUPING_OPERATORS.contains(&chars[i])
            && chars[i - 1] != '('
            && chars[i] != ')'
        {
            tokens.push(Token::Concatenation);
        }

        match (chars[i], escaped) {
            ('\\', false) => {
                escaped = true;
            }
            ('(', false) => {
                tokens.push(Token::OpenParenthesis);
            }
            (')', false) => {
                tokens.push(Token::CloseParenthesis);
            }
            ('|', false) => {
                tokens.push(Token::Union);
            }
            ('*', false) => {
                tokens.push(Token::KleeneQuantifier);
            }
            ('?', false) => {
                tokens.push(Token::OptionalQuantifier);
            }
            ('+', false) => {
                tokens.push(Token::PositiveQuantifier);
            }
            ('.', false) => {
                tokens.push(Token::Wildcard);
                escaped = false;
            }
            (c, _) => {
                tokens.push(Token::Letter(c));
                escaped = false;
            }
        }
    }

    tokens
}

fn str_count_diff(op: &Token) -> i32 {
    match op {
        // increases count
        Token::Letter(_) => 1,
        Token::Wildcard => 1,
        Token::CloseParenthesis => 1, // should be 1 valid string if inside of () is regex
        // consumes 2, produces one
        Token::Concatenation => -1,
        Token::Union => -1,
        // keeps count the same
        Token::KleeneQuantifier => 0,
        Token::PositiveQuantifier => 0,
        Token::OptionalQuantifier => 0,
        Token::OpenParenthesis => 0,
    }
}

// Modified Shunting Yard Algorithm
// TODO: Validate regex
pub fn calc_postfix(tokens: Vec<Token>) -> Option<Vec<Token>> {
    let mut operators = vec![];
    let mut postfix: Vec<Token> = vec![];

    let mut num_strs: i32 = 0;
    let mut preservation_stack: Vec<i32> = vec![];

    for token in tokens.iter() {
        match token {
            Token::OpenParenthesis => {
                // we need to perserve the num of strs before the parentheses
                // to validate the larger regex and reset count to validate
                // the regex inside of the parentheses
                preservation_stack.push(num_strs);
                num_strs = 0;

                operators.push(Token::OpenParenthesis);
            }
            Token::CloseParenthesis => {
                // nothing to close, malformed parentheses group
                if preservation_stack.len() == 0 {
                    return None;
                }

                while operators.len() > 0 && *operators.last().unwrap() != Token::OpenParenthesis {
                    let op = operators.pop().unwrap();
                    postfix.push(op);
                    num_strs += str_count_diff(&op);
                }

                // a regex should only result in one string
                if num_strs != 1 {
                    return None;
                }

                // pop off open parenthesis
                operators.pop();

                // we need to restore the prev string count
                if let Some(s) = preservation_stack.pop() {
                    num_strs = s;
                }
                num_strs += str_count_diff(token);
            }
            // operators
            Token::Union
            | Token::Concatenation
            | Token::KleeneQuantifier
            | Token::OptionalQuantifier
            | Token::PositiveQuantifier => {
                // these operators require at least one str before them
                if num_strs <= 0 {
                    return None;
                }

                while operators.len() > 0
                    && *operators.last().unwrap() != Token::OpenParenthesis
                    && operators.last().unwrap().has_greater_precedence(*token)
                {
                    let op = operators.pop().unwrap();
                    postfix.push(op);
                    num_strs += str_count_diff(&op);
                }
                operators.push(*token);
            }
            // char matches
            Token::Letter(_) | Token::Wildcard => {
                // for letters and wildcards it should increment by 1
                num_strs += str_count_diff(token);
                postfix.push(token.clone());
            }
        }
    }

    while operators.len() > 0 {
        let op = operators.pop().unwrap();
        postfix.push(op);
        num_strs += str_count_diff(&op);
    }

    // a regex should only result in one string and no malformed parenthesis should work
    if preservation_stack.len() > 0 || num_strs != 1 {
        return None;
    }

    Some(postfix)
}
