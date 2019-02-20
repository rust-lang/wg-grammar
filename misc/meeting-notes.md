# 2019-01-23

We're making slow, but steady progress!
PRs from @CAD97, @cramertj and @ehuss!

1. lykenware/gll maintenance.
  - The overall consensus is that we need to wrap our brains around it ASAP, and reduce the bus factor.
  - @cramertj started refactoring/cleaning up some bits, we should probably do the same until we get more clarity around how gll works.
  - It's clear that we all miss @eddyb :D
1. Removing the existing grammar from rust-lang/rust: https://github.com/rust-lang-nursery/wg-grammar/issues/9
  - We punted the topic for the next meeting, there was a suggestion of just moving the old grammar into our repo and linking to it.


# 2019-01-09

## tentative agenda

- deal with the in-flight PRs
- setup travis for the repo
- mini-roadmap discussion

## action items

- @qmx parallelization with rayon, travis fixes, blacklist => toml.
- @CAD %% and |=
- @cramertj  inline unit tests
- @centril  removing grammar redundancies + spicing up repo structure
- @ehuss  Making a PR to close issue #1 (charter), cleaning up + enhancing lexical spec in reference, use that to close #3 (lexical spec).

# 2018-11-28

Summary for today's meeting

* @eddyb#0426  had nothing to report since no progress has been made
* @Centril#2862  worked on an RFC [rust-lang/rfcs#2602](https://github.com/rust-lang/rfcs/pull/2602) and tested out the GLL syntax, it felt good, https://github.com/Centril/rfcs/blob/rfc/attributes-galore/text/0000-attributes-galore.md#grammar particularly with the `Rule |= Additions;` formatting. @qmx#4460  agreed it was pleasant and that it was "BNF done right".
* We call the notation in GLL: "lyg".
  * [x]  Change file ending to `.lyg` after merging PR.
* @Centril#2862 felt that parameterized grammars could reduce some duplication with e.g. `Bounds<P> = elems:P* % "+" "+"?;` and similar things.
* [ ]  Cargo features should be tried out for nightly features
* [ ]  @qmx#4460  needs to: "setup the initial test harness with rust-lang/rust vendored into the grammar repo as a submodule"
* [ ]  @eddyb#0426  needs to do some stuff on the GLL side;
* [ ]  @qmx#4460  will work on adding `%%` to GLL
* [ ]  Test harness is top prio;
* [x]  @Centril#2862  will work on improving @eddyb#0426's PR wrt. style and such things.
* [ ]  @ehus might split some work with @qmx.

# 2018-11-14

Last wednesday the wg-grammar team had it's biweekly meeting - here are the principal points:

1. Progress report on @eddyb's [PR](https://github.com/rust-lang-nursery/wg-grammar/pull/13)
  - how to deal with unstable features
    - We're going to go with separate files and using `|=` for defining alternate rules.
    - when to merge @eddyb's PR?
    - There's a blocker where GLL uses a lot of ram for parsing a single file - once @eddyb fixes this we'll merge the PR.
2. testing
  - inline unit tests
    - We've decided to go with tests in separate files, mimicking the grammar file tree structure, since this brings us the advantage of being able to even had AST/parse tree expectation files
  - The initial proposed test file name scheme is `<Production>.<random_description>.<ext>`
  - testing against external corpus
     - The initial testing corpus will be rust-lang/rust, once we get to a good place we'll extend this to maybe top 100 crates.io (stable)
3. how to deal with ambiguity (declarative...)
  - eddyb thinks that's too early to discuss this, we're punting this to the near future
4. GLL
  - formatting
    - we asked for small changes on the overall feel for GLL grammars, [GLL PR](https://github.com/lykenware/gll/pull/74) with the proposed changes.

```
FnArgs =
  | Regular:FnArg+ % ","
  | Variadic:"..."
  | RegularAndVariadic:{ args:FnArg+ % "," "," "..." }
  ;
```

We're using leading `|` and moving the other `|` to the start of the line.
  - syntax changes
    - there were some discussions about using BNF-like `::=` for attribution of new rules but this went nowhere at least for now.
    - there were also discussions about having parameterized grammars

