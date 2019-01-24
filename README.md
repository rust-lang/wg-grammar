# wg-grammar

This is the home for the Rust Grammar Working Group. The goal of the working
group is to satisfy [RFC 1331] and produce a testable, canonical grammar for
the Rust language. The primary audiences for the grammar are:

- Rust RFC authors who wish to propose and communicate changes.
- rustc and Rust tool developers who need an authoritative definition of the
  grammar.
- To assist documentation efforts to communicate valid Rust syntax to users,
  and to facilitate the [Rust language specification].

The grammar tools produced here are not intended to be used directly within
rustc, or any other existing tools.

## Links
- [Meeting notes](misc/meeting-notes.md)
- [Resources](misc/resources.md)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[RFC 1331]: https://github.com/rust-lang/rfcs/blob/master/text/1331-grammar-is-canonical.md
[Rust language specification]: https://github.com/rust-lang-nursery/reference/
