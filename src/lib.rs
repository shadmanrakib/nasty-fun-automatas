use wasm_bindgen::prelude::*;

mod nfa;
mod parse;

// a bit unconventional, but the tests are in a separate file from code
#[cfg(test)]
mod tests;

#[wasm_bindgen]
pub struct Regex {
    nfa: nfa::NFA,
}

#[wasm_bindgen]
impl Regex {
    pub fn new(str: String) -> Option<Regex> {
        let nfa = nfa::NFA::from_regex(&str)?;
        Some(Regex { nfa })
    }
    #[allow(non_snake_case)]
    pub fn isMatch(&self, input: String) -> bool {
        self.nfa.is_match(&input)
    }
}
