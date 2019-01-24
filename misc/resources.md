# Resources

This is a collection of various resources that may be helpful to anyone
looking at or working on the grammar.

## Previous/Existing Grammars and Parsers
- The truth: https://github.com/rust-lang/rust/tree/master/src/libsyntax
- The reference: https://github.com/rust-lang-nursery/reference/
- The old grammar: https://doc.rust-lang.org/nightly/grammar.html ([src](https://github.com/rust-lang/rust/blob/master/src/doc/grammar.md))
- The old ANTLR grammar ([removed](https://github.com/rust-lang/rust/pull/41705)): https://github.com/rust-lang/rust/tree/12e76e24cc1466ffb2bd37cc7652a6dd7ba15735/src/grammar
- Old flex/bison grammar ([outdated](https://github.com/rust-lang/rust/issues/32723)): https://github.com/rust-lang/rust/tree/master/src/grammar
- Intellij's parser: https://github.com/intellij-rust/intellij-rust/blob/master/src/main/grammars/RustParser.bnf
- jorendorff's ANTLR grammar: https://github.com/jorendorff/rust-grammar/
- Niko's rustypop: https://github.com/nikomatsakis/rustypop
- Haskell parser: https://github.com/harpocrates/language-rust/blob/master/src/Language/Rust/Parser/Internal.y

## Grammar RFC
The call to canonicalize the grammar.

- RFC: https://github.com/rust-lang/rfcs/blob/master/text/1331-grammar-is-canonical.md
- RFC tracker: https://github.com/rust-lang/rust/issues/30942
- Original RFC discussion with interesting stuff: https://github.com/rust-lang/rfcs/pull/1331

## New Rust Parsing Projects
- Parser used here: https://github.com/rust-lang-nursery/gll
  (Forked from https://github.com/lykenware/gll/)
- Lossless syntax tree: https://github.com/matklad/rowan/
- New front-end rust parser: https://github.com/matklad/rust-analyzer/

## Testing
Some other test suites for Rust parsing.

- https://github.com/brson/ctrs
- The old grammar bot: https://github.com/rust-lang/rust/issues/28592
- Intellij's test suite: https://github.com/intellij-rust/intellij-rust/tree/master/src/test/resources/org/rust/lang/core/parser/fixtures
