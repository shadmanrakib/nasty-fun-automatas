mod nfa;
mod parse;

// a bit unconventional, but the tests are in a separate file from code
#[cfg(test)]
mod tests;

fn main() {
    // operations
    // | - union, . - wildcard, * - kleene, () - groups, ? - optional, + more than zero, \ - escape
    // " " is a space
    // spaces are not ignored
    // concat is implicit
}
