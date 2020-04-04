# Coding style

These are the coding style requirements for this repository, not including those
enforced by `clippy`, `rustfmt`, and `rustc`.

- [Declaration order](#declaration-order)
- [Forbidden](#forbidden)
- [General guidelines](#general-guidelines)
- [Line wrapping](#line-wrapping)
- [`use` paths](#use-paths)

## Declaration order

The order of [`use` declarations] and `mod` imports should be the following,
each section separated by a newline:

1. `use`s of standard library items

2. `use`s of external crate items

3. `mod` imports

4. `use`s of local crate items

Example:

```rust
use std::io;
use std::fs::File;

use proc_macro::TokenStream;

mod foo;
mod bar;

use crate::bar::Baz;

// --snip--
```

## Forbidden

The following should not be used whatsoever, except under the listed exceptions.

- Explicit returning of the [unit type].

- `extern crate`, with [exceptions]

- [`return` expressions]

- TODO and FIXME comments. This is enforced by `clippy`, but we encourage you
to not just remove the comment to pass CI; create a GitHub issue instead.

## General guidelines

As a general rule, all code and documentation in this repository conforms to the
[Rust API Guidelines] and [Rust RFC 1574]. We won't check every commit against
them; that would be impractical. But please at least read through [this
checklist] to get a feel for what's expected. If you find something in the
codebase that doesn't conform to the guidelines, feel free to [submit an issue].

## Line wrapping

Line wrapping should be done primarily by `rustfmt` (i.e. don't manually wrap
long-ish lines). Of course, if your code is significantly harder to read with
manual wrapping, by all means, wrap it. Just make sure it's consistent with
the code around it.

## Use paths

We always use idiomatic `use` paths according to [those in the book].


[exceptions]: https://doc.rust-lang.org/reference/items/extern-crates.html#extern-prelude
[`return` expressions]: https://doc.rust-lang.org/reference/expressions/return-expr.html
[Rust API Guidelines]: https://rust-lang.github.io/api-guidelines/
[Rust RFC 1574]: https://rust-lang.github.io/rfcs/1574-more-api-documentation-conventions.html
[submit an issue]: https://github.com/lberrymage/accrescent/issues/new
[this checklist]: https://rust-lang.github.io/api-guidelines/checklist.html
[those in the book]: https://doc.rust-lang.org/stable/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
[unit type]: https://doc.rust-lang.org/std/primitive.unit.html
[`use` declarations]: https://doc.rust-lang.org/reference/items/use-declarations.html
