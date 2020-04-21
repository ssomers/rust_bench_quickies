## What is this?

Simple benchmarks of Rust (standard library) features.

* option_cardinality: various ways to convert a boolean to an integer (i.e. Option::is_some converted to the number of elements in the Option container)
* btreeset_binary_contains vs btreeset_binary_contains_if_not_empty: when sets are quite likely to be empty, `!set.is_empty() && set.contains(elementt)` is faster than `set.contains(element)` on its own.

## How to run this?

    rustup install nightly
    cargo +nightly bench
