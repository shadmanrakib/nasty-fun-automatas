# nasty-fun-automatas

I wanted to build a Regex library using theory I learned in an Automata course. This also seemed like a good project to mess around with WebAssembly. 

This project implements a Rust library for regular expression matching using Non-deterministic Finite Automaton (NFA). It includes bindings for WebAssembly using `wasm-bindgen`, allowing for use in many JavaScript environments.

You can view a basic demo here: https://nasty-fun-automatas.vercel.app/.

## Features

- Construct NFAs from regular expressions.
- Match strings against the constructed NFA.
- WebAssembly support for easy integration with JavaScript.

## Installation

To use this library, you need to have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Build

To build the project, run:

```sh
wasm-pack build
```

### Test

To run the tests, use:

```sh
cargo test
```

## Regular Expression Language

The regular expression language supported by this library is a subset of typical regex features, allowing for the construction of NFAs that can handle common pattern matching tasks. Here’s a summary of the supported syntax:

### Supported Syntax

- **Literals**: Match exact characters. For example, `a` matches the character 'a'.
- **Concatenation**: Match sequences of characters. For example, `abc` matches the string "abc".
- **Union (|)**: Match either of two patterns. For example, `a|b` matches "a" or "b".
- **Kleene Star (*)**: Match zero or more repetitions of the preceding element. For example, `a*` matches "", "a", "aa", "aaa", etc.
- **Wildcard (.)**: Match any single character. For example, `a.b` matches "aab", "abb", "acb", etc.
- **Positive Quantifier (+)**: Match one or more repetitions of the preceding element. For example, `a+` matches "a", "aa", "aaa", etc.
- **Optional Quantifier (?)**: Match zero or one occurrence of the preceding element. For example, `a?` matches "" or "a".

### White Spaces

In this library, white spaces in the regular expression are treated as literal white spaces. For example, the regex `a b` matches the string "a b" (with a space between 'a' and 'b') and does not match "ab".

### Escaping Reserved Characters

Reserved characters (such as `|`, `*`, `.`, `+`, `?`, and `(`, `)`) can be escaped using a backslash (`\`) to match them literally. For example:
- `\|` matches the character '|'
- `\*` matches the character '*'
- `\.` matches the character '.'

This allows for flexibility when constructing regex patterns that need to include these special characters.

### Non-Empty Languages Only

This library supports only non-empty languages, meaning that every valid regular expression must match at least one string. An empty regular expression is considered invalid, and the `NFA::from_regex` method will return `None` for such inputs. This ensures that constructed NFAs are always capable of performing meaningful matches.

### Examples

Here are a few examples demonstrating the usage of the regex language:

- **Literal Matching**:
  - Regex: `abc`
  - Matches: "abc"
  - Does not match: "ab", "abcd"

- **Union**:
  - Regex: `a|b`
  - Matches: "a", "b"
  - Does not match: "ab", "ba"

- **Kleene Star**:
  - Regex: `a*`
  - Matches: "", "a", "aa", "aaa"
  - Does not match: "b", "ab"

- **Wildcard**:
  - Regex: `a.b`
  - Matches: "aab", "abb", "acb"
  - Does not match: "ab", "a", "abc"

- **Positive Quantifier**:
  - Regex: `a+`
  - Matches: "a", "aa", "aaa"
  - Does not match: "", "b"

- **Optional Quantifier**:
  - Regex: `a?`
  - Matches: "", "a"
  - Does not match: "aa", "b"

- **White Spaces**:
  - Regex: `a b`
  - Matches: "a b"
  - Does not match: "ab", "a  b"

- **Escaped Characters**:
  - Regex: `a\|b`
  - Matches: "a|b"
  - Does not match: "ab", "a", "|b"

## Project Structure

- **src/nfa.rs**: Contains the implementation of the NFA, including state transitions and matching logic.
- **src/parse.rs**: Contains the parsing logic to convert regular expressions into tokens and then into postfix notation.
- **src/lib.rs**: The main library file that exposes the `Regex` struct and its methods via `wasm-bindgen`.
- **src/tests.rs**: Contains the test cases for the library.

## Usage

### Rust

In Rust, you can create a `Regex` object and use it to match strings as follows:

```rust
use wasm_bindgen::prelude::*;
use nazsty_fun_automatas::Regex;

let regex = Regex::new("a*b".to_string()).unwrap();
let is_match = regex.isMatch("aaab".to_string());
assert!(is_match);
```

### JavaScript

After building the project with `wasm-pack`, you can use the generated WebAssembly module in JavaScript:

```js
import init, { Regex } from './nasty_fun_automatas';

async function run() {
    await init();

    const regex = Regex.new("a*b");
    const isMatch = regex.isMatch("aaab");
    console.log(isMatch); // true
}

run();
```

## Regex Module

### `Regex`

#### Methods

- **`new(str: String) -> Option<Regex>`**: Constructs a new `Regex` object from the given regular expression string. Returns `None` if the regex is invalid.
- **`isMatch(&self, input: String) -> bool`**: Checks if the input string matches the regex.

## Internal Structure

### NFA

The NFA (Non-deterministic Finite Automaton) implementation is inspired by Russ Cox's blog post. The following structs and enums are used for the Thompson construction:

- **State**: Represents a state in the NFA with transitions to other states.
- **Transition**: Represents a transition from one state to another, with a label that can be a character, wildcard, epsilon, or none.
- **NFAFragement**: Represents a fragment of an NFA used during construction.
- **NFA**: Represents the entire NFA with methods to construct from a regex and to match strings.

For matches, the library uses Breadth First Search to see if we can consume all the characters in the input and end in an accepting state.

### Parsing

The parsing module uses a modified Shunting Yard algorithm to convert regular expressions into postfix notation, which is used to construct the NFA.

## Acknowledgements

This project uses `wasm-bindgen` for WebAssembly support and is inspired by various resources on regular expression and NFA implementations. The library implements the inductive NFA construction described in:

-  *Introduction to Automata Theory, Languages, and Computation* by John E. Hopcroft, Rajeev Motwani, and Jeffrey D. Ullman
-  Russ Cox's blog post https://swtch.com/~rsc/regexp/regexp1.html
