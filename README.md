Simple order book
=================

This is a small sample crate implementing a simple order book in Rust.

In the heart there is the `OrderBook` type, with methods to add and cancel
orders.

The order book is implemented on top of a `BTreeSet`, where the head contains
the top of the book for selling orders and the tail contains the top of the
book for buying orders.

Directory structure
===================

* `src/`: implementation files
* `test/`: integration tests

Building and running
====================

To build this project you need rust nightly:

```sh
$ rustup install nighly
$ rustup default nighly
```

Then just enter the project directory and run the tests:

```sh
$ cargo test
```
