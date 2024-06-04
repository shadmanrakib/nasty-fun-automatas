mod nfa;
mod parse;

// a bit unconventional, but the tests are in a separate file from code
#[cfg(test)]
mod tests;

fn main() {
    // operations
    // | - union, . - wildcard, * - kleene, () - groups, ? - optional
    // concat is implicit
    // let regex = "pens?".to_string();
    // println!("tokens: {:?}", parse_re_to_tokens(regex.clone()));
    // println!("postfix: {:?}", calc_postfix(parse_re_to_tokens(regex)));
}
