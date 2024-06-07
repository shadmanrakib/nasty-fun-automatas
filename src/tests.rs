use super::*;

#[test]
fn valid_regex_test() {
    let valid_cases = [
        (
            "pens?",
            vec![
                ("", false),
                ("hey", false),
                ("p", false),
                ("pe", false),
                ("pen", true),
                ("pens", true),
                ("pens?", false),
                ("spens", false),
                ("pencil", false),
            ],
        ),
        (
            ".*",
            vec![("", true), ("a", true), ("sdads", true), ("+__Sd*sd", true)],
        ),
        (
            "a|b",
            vec![
                ("", false),
                ("a", true),
                ("sdads", false),
                ("+__Sd*sd", false),
                ("b", true),
            ],
        ),
        (
            "ab",
            vec![
                ("", false),
                ("a", false),
                ("sdads", false),
                ("+__Sd*sd", false),
                ("b", false),
                ("ab", true),
            ],
        ),
        (
            "a+b",
            vec![
                ("", false),
                ("a", false),
                ("sdads", false),
                ("aaab", true),
                ("aaabb", false),
                ("b", false),
                ("ab", true),
            ],
        ),
        (
            "a.*|b",
            vec![
                ("", false),
                ("a", true),
                ("b", true),
                ("sdads", false),
                ("+__Sd*sd", false),
                ("absd", true),
                ("bsd", false),
                ("afeoriobsdsada", true),
                ("ab", true),
                ("bafeoriobsdsada", false),
                ("ba", false),
            ],
        ),
        (
            "a(bb)*|b",
            vec![
                ("", false),
                ("a", true),
                ("b", true),
                ("sdads", false),
                ("+__Sd*sd", false),
                ("abbsd", false),
                ("bsd", false),
                ("afeoriobsdsada", false),
                ("ab", false),
                ("abb", true),
                ("abbb", false),
                ("abbbbbbbb", true),
                ("bafeoriobsdsada", false),
                ("ba", false),
            ],
        ),
        (
            "a(bb)+|b",
            vec![
                ("", false),
                ("a", false),
                ("b", true),
                ("sdads", false),
                ("+__Sd*sd", false),
                ("abbsd", false),
                ("bsd", false),
                ("afeoriobsdsada", false),
                ("ab", false),
                ("abb", true),
                ("abbb", false),
                ("abbbbbbbb", true),
                ("bafeoriobsdsada", false),
                ("ba", false),
            ],
        ),
        (
            "a(bb)?|b",
            vec![
                ("", false),
                ("a", true),
                ("b", true),
                ("sdads", false),
                ("+__Sd*sd", false),
                ("abbsd", false),
                ("bsd", false),
                ("afeoriobsdsada", false),
                ("ab", false),
                ("abb", true),
                ("abbb", false),
                ("abbbbbbbb", false),
                ("bafeoriobsdsada", false),
                ("ba", false),
            ],
        ),
        (
            ".*a.*",
            vec![
                ("", false),
                ("a", true),
                ("b", false),
                ("sdads", true),
                ("+__Sd*sd", false),
                ("abbsd", true),
                ("bsd", false),
                ("afeoriobsdsada", true),
                ("ab", true),
                ("abb", true),
                ("abbb", true),
                ("abbbbbbbb", true),
                ("bafeoriobsdsada", true),
                ("ba", true),
            ],
        ),
        (
            ".+@.+\\.com?", // emails ending with com or co
            vec![
                ("", false),
                ("a", false),
                ("b", false),
                ("@.com", false),
                ("hi@gmail.com", true),
                ("@gmail.com", false),
                ("hi@.com", false),
                ("sd@gmail.co", true),
                ("asdjk@sd.c", false),
                ("abb", false),
                ("sd@hotmail.co", true),
                ("abbbbbb.co", false),
                ("ooof", false),
                ("hmm", false),
            ],
        ),
    ];
    for (re, cases) in valid_cases {
        println!("re: {}", re);
        if let Some(nfa) = nfa::NFA::from_regex(&re.to_string()) {
            for (input, expected) in cases {
                let result = nfa.is_match(&input.to_string());
                if result != expected {
                    println!("re {re}, case: {input}, result: {result}, expected: {expected}");
                }
                assert_eq!(result, expected);
            }
        } else {
            panic!("re {re} expected to be valid, but no NFA returned");
        }
    }
}

#[test]
fn invalid_regex_test() {
    let invalid_cases = [
        // empty languages are not accepted
        "",
        "()()((()))",
        // malformed parentheses
        "(",
        "())",
        "()()(",
        "a+(a",
        "(a|)b",
        // using operators without char matchers
        "+",
        "*",
        "?",
        "+",
        "|",
        // only one string for 2 string operator
        "a|",
        "|a",
        "a||",
        "a||b",
        "a|(b|x|)",
    ];
    for re in invalid_cases {
        println!("re: {}", re);
        if let Some(_) = nfa::NFA::from_regex(&re.to_string()) {
            panic!("re {re} expected to be invalid, but NFA returned");
        }
    }
}
