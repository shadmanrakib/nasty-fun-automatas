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
    #[wasm_bindgen(constructor)]
    pub fn new(str: String) -> Self {
        Regex {
            nfa: nfa::NFA::from_regex(&str),
        }
    }
    #[allow(non_snake_case)]
    pub fn isMatch(&self, input: String) -> bool {
        self.nfa.is_match(&input)
    }
}
