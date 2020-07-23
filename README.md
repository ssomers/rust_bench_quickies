## What is this?

Simple benchmarks of Rust (standard library) features.

* option_cardinality: various ways to convert a boolean to an integer listed under a [stackoverflow question](https://stackoverflow.com/questions/55461617/how-do-i-convert-a-boolean-to-an-integer-in-rust), in particular for converting `Option::is_some` into the number of elements in an Option container.
* option_unwrap: various ways to unwrap an option when you are sure it cannot be empty.
* btreeset_binary_contains vs btreeset_binary_contains_if_not_empty: when sets are quite likely to be empty, `!set.is_empty() && set.contains(elementt)` used to be faster than `set.contains(element)` on its own.
* btreeset_general_contains vs btreeset_general_contains_if_not_empty: found the equilibrium where this trick evens out.
* btreeset_iter vs btreeset_range: creating an iterator takes longer than creating a range, that is kind of a pair of iterators. Which is solved in [issue #62924](https://github.com/rust-lang/rust/issues/62924), except that the numbers say it isn't!? To be continued...

## How to run this?

    rustup install nightly
    rustup run nightly cargo bench
