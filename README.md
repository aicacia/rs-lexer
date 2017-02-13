rs-lexer [![Build Status](https://travis-ci.org/nathanfaucett/rs-lexer.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-lexer)
=====

lexer for basic token parsing

```rust
extern crate lexer;


use lexer::Lexer;


fn main() {
  let mut lexer = Lexer::new("
    let template = \"
        Count: $1
    \";

    fn get_count(count) {
        template('$' + count)
    }
  ");

  for token in lexer {
    println!("{:?}", token);
  }
}
```
