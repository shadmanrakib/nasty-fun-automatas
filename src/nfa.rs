// =================
// NFA
// =================

use std::collections::{HashSet, VecDeque};

use crate::parse::{calc_postfix, parse_re_to_tokens, Token};

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
    // thompson NFAs branches at most
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

pub struct NFA {
    start_id: usize,
    states: Vec<State>,
}

impl NFA {
    pub fn from_regex(re: &String) -> Option<NFA> {
        let tokens = parse_re_to_tokens(re);

        // if the postfix is invalid (None), we cannot construct
        // an NFA because we we're provided with an invalid regex
        // so we propogate the None
        let postfix = calc_postfix(tokens)?;

        // when we have an empty regex, treat it as an empty language
        // so never matches
        if postfix.len() == 0 {
            return Some(NFA::empty_language());
        }

        // we will liberally use unwraps since we know an NFA can
        // be constructed since we validated the input regex when
        // constructing the NFA

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
        states[fragments[0].out_id].set_accepting(true);
        // we have all the info we need to create NFA
        Some(NFA { start_id, states })
    }
    fn empty_language() -> NFA {
        let mut states = Vec::<State>::with_capacity(2);
        let start_id = states.len();
        let start = State::new();
        let mut out = State::new();
        out.set_accepting(true);
        states.push(start);
        states.push(out);
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

impl NFA {
    pub fn is_match(&self, input: &String) -> bool {
        let chars: Vec<char> = input.chars().collect();

        // hashset entry: (idx of input, state visited)
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = VecDeque::<(usize, usize)>::new();

        // push start on to queue
        queue.push_back((0, self.start_id));

        while let Some((idx, state_id)) = queue.pop_front() {
            // mark visited
            visited.insert((idx, state_id));

            // if we consumed all chars and ended up on a accepting state
            // we can end, return true
            if idx >= chars.len() {
                if self.states[state_id].accepting {
                    return true;
                }
            }

            // enqueue all
            for transition in &self.states[state_id].transitions {
                match transition.label {
                    TransitionLabel::Epsilon => {
                        let next = (idx, transition.to);
                        if !visited.contains(&next) {
                            queue.push_back(next);
                        }
                    }
                    TransitionLabel::Wildcard => {
                        let next = (idx + 1, transition.to);
                        if !visited.contains(&next) && idx < chars.len() {
                            queue.push_back(next);
                        }
                    }
                    TransitionLabel::Letter(c) => {
                        let next = (idx + 1, transition.to);
                        if idx < chars.len() && chars[idx] == c {
                            queue.push_back(next);
                        }
                    }
                    _ => {}
                }
            }
        }

        false
    }
}
