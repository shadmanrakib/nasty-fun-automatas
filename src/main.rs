// =================
// PARSING
// =================

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
const NONGROUPING_OPERATORS: [char; 4] = ['|', '*', '?', '+'];
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
// TODO: Validate regex
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

// =================
// NFA
// =================

#[derive(Debug)]
enum TransitionLabel {
    Letter(char),
    Wildcard,
    Epsilon,
    None,
}
#[derive(Debug)]
struct Transition {
    label: TransitionLabel,
    to: usize,
}
impl Transition {
    const NONE: Transition = Transition {
        label: TransitionLabel::None,
        to: 0,
    };
}
#[derive(Debug)]
struct State {
    // thompson const branches at most
    num_transitions: usize,
    transitions: [Transition; 2],
    accepting: bool,
}

impl State {
    fn new() -> State {
        let transitions: [Transition; 2] = [Transition::NONE, Transition::NONE];
        State {
            num_transitions: 0,
            transitions,
            accepting: false,
        }
    }
    fn with_transition(mut self, transition: Transition) -> Self {
        self.transitions[self.num_transitions] = transition;
        self.num_transitions += 1;
        self
    }
    fn with_accepting(mut self, accepting: bool) -> Self {
        self.accepting = accepting;
        self
    }
    fn add_transition(&mut self, transition: Transition) {
        self.transitions[self.num_transitions] = transition;
        self.num_transitions += 1;
    }
    fn set_accepting(&mut self, accepting: bool) {
        self.accepting = accepting;
    }
}

struct NFAFragement {
    start_id: usize,
    out_id: usize,
}

struct NFA {
    start_id: usize,
    states: Vec<State>,
}

impl NFA {
    fn from_regex(re: String) -> NFA {
        let tokens = parse_re_to_tokens(re);
        let postfix = calc_postfix(tokens);

        let mut states: Vec<State> = vec![];
        let mut fragments: Vec<NFAFragement> = vec![];

        for token in postfix {
            match token {
                Token::Letter(c) => {
                    fragments.push(NFA::add_single_transition_fragment(
                        &mut states,
                        TransitionLabel::Letter(c),
                    ));
                }
                Token::Wildcard => {
                    fragments.push(NFA::add_single_transition_fragment(
                        &mut states,
                        TransitionLabel::Wildcard,
                    ));
                }
                Token::Concatenation => {
                    let end_fragment = fragments.pop().unwrap();
                    let start_fragment = fragments.pop().unwrap();

                    fragments.push(NFA::add_concat_fragment(
                        &mut states,
                        start_fragment,
                        end_fragment,
                    ));
                }
                Token::Union => {
                    let frag_a = fragments.pop().unwrap();
                    let frag_b = fragments.pop().unwrap();

                    fragments.push(NFA::add_union_fragment(&mut states, frag_a, frag_b));
                }
                Token::KleeneQuantifier => {
                    let frag = fragments.pop().unwrap();
                    fragments.push(NFA::add_quantifier_fragment(&mut states, frag, true, true));
                }
                Token::PositiveQuantifier => {
                    let frag = fragments.pop().unwrap();
                    fragments.push(NFA::add_quantifier_fragment(&mut states, frag, true, false));
                }
                Token::OptionalQuantifier => {
                    let frag = fragments.pop().unwrap();
                    fragments.push(NFA::add_quantifier_fragment(&mut states, frag, false, true));
                }
                // parentheses should not be in the postfix
                _ => unreachable!(),
            }
        }

        // turn fragment to NFA
        let start_id = fragments[0].start_id;
        // make last node accepting
        states[fragments[0].out_id].accepting = true;
        // we have all the info we need to create NFA
        NFA { start_id, states }
    }
    fn add_single_transition_fragment(
        states: &mut Vec<State>,
        label: TransitionLabel,
    ) -> NFAFragement {
        let start_id = states.len();
        let out_id = states.len() + 1;

        let start = State::new().with_transition(Transition { label, to: out_id });
        let out = State::new();

        states.push(start);
        states.push(out);

        NFAFragement { start_id, out_id }
    }
    fn add_concat_fragment(
        states: &mut Vec<State>,
        start_fragment: NFAFragement,
        end_fragment: NFAFragement,
    ) -> NFAFragement {
        // add epsilon transition to from end of start_fragment
        // that jumps to start of end_fragment
        let transition = Transition {
            label: TransitionLabel::Epsilon,
            to: end_fragment.start_id,
        };
        states[start_fragment.out_id].add_transition(transition);

        let start_id = start_fragment.start_id;
        let out_id = end_fragment.out_id;

        NFAFragement { start_id, out_id }
    }

    fn add_union_fragment(
        states: &mut Vec<State>,
        frag_a: NFAFragement,
        frag_b: NFAFragement,
    ) -> NFAFragement {
        let start_id = states.len();
        let out_id = states.len() + 1;

        let start = State::new()
            .with_transition(Transition {
                label: TransitionLabel::Epsilon,
                to: frag_a.start_id,
            })
            .with_transition(Transition {
                label: TransitionLabel::Epsilon,
                to: frag_b.start_id,
            });
        let out = State::new();

        states[frag_a.out_id].add_transition(Transition {
            label: TransitionLabel::Epsilon,
            to: out_id,
        });
        states[frag_b.out_id].add_transition(Transition {
            label: TransitionLabel::Epsilon,
            to: out_id,
        });

        states.push(start);
        states.push(out);

        NFAFragement { start_id, out_id }
    }

    fn add_quantifier_fragment(
        states: &mut Vec<State>,
        frag: NFAFragement,
        repeat: bool,
        optional: bool,
    ) -> NFAFragement {
        let start_id = states.len();
        let out_id = states.len() + 1;

        // connect to frag
        let mut start = State::new().with_transition(Transition {
            label: TransitionLabel::Epsilon,
            to: frag.start_id,
        });
        let out = State::new();

        // have the result of frag go to out
        states[frag.out_id].add_transition(Transition {
            label: TransitionLabel::Epsilon,
            to: out_id,
        });

        if optional {
            start.add_transition(Transition {
                label: TransitionLabel::Epsilon,
                to: out_id,
            });
        }

        if repeat {
            states[frag.out_id].add_transition(Transition {
                label: TransitionLabel::Epsilon,
                to: frag.start_id,
            })
        }

        states.push(start);
        states.push(out);

        NFAFragement { start_id, out_id }
    }
}

fn main() {
    // operations
    // | - union, . - wildcard, * - kleene, () - groups, ? - optional
    // concat is implicit
    // let regex = "pens?".to_string();
    // println!("tokens: {:?}", parse_re_to_tokens(regex.clone()));
    // println!("postfix: {:?}", calc_postfix(parse_re_to_tokens(regex)));
    for re in [
        "pens?".to_string(),
        ".*".to_string(),
        "a.*|b".to_string(),
        "a(bb)*|a".to_string(),
    ] {
        println!("re: {}", re);
        let nfa = NFA::from_regex(re);
        println!("start: {}", nfa.start_id);
        for (i, s) in nfa.states.iter().enumerate() {
            println!("{:?} {:?}", i, s);
        }
    }
}
