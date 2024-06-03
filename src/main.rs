#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
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
const NONGROUPING_OPERATORS: [char; 4] = ['|', '*', '.', '?'];
const TWO_OPERAND_OPERATORS: [char; 1] = ['|'];

fn parse_re_to_tokens(re: String) -> Vec<Token> {
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

// Shunting Yard Algorithm
fn calc_postfix(tokens: Vec<Token>) -> Vec<Token> {
    let mut operators = vec![];
    let mut postfix: Vec<Token> = vec![];

    for token in tokens.iter() {
        match token {
            Token::OpenParenthesis => {
                operators.push(Token::OpenParenthesis);
            }
            Token::CloseParenthesis => {
                while operators.len() > 0 && *operators.last().unwrap() != Token::OpenParenthesis {
                    postfix.push(operators.pop().unwrap());
                }
                // pop off open parenthesis
                operators.pop();
            }
            Token::Union | Token::Concatenation => {
                while operators.len() > 0
                    && *operators.last().unwrap() != Token::OpenParenthesis
                    && operators.last().unwrap().has_greater_precedence(*token)
                {
                    postfix.push(operators.pop().unwrap());
                }
                operators.push(*token);
            }
            c => {
                postfix.push(c.clone());
            }
        }
    }

    while operators.len() > 0 {
        postfix.push(operators.pop().unwrap());
    }

    postfix
}

fn main() {
    // operations
    // | - union, . - wildcard, * - kleene, () - groups, ? - optional
    // concat is implicit
    let regex = "a(b\\\\b)|a(s)*s*?".to_string();
    println!("tokens: {:?}", parse_re_to_tokens(regex.clone()));
    println!("postfix: {:?}", calc_postfix(parse_re_to_tokens(regex)));
    println!("Hello, world!");
}
