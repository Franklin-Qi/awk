# uutils AWK

This is a human, WIP, and clean implementation of an AWK interpreter, written in Rust and compatible with GNU's AWK (`gawk`) bug-for-bug. Expected to be production-ready before Ubuntu 26.10. Made with love.

## State of the Repo

The lexer is pretty much done; however, it should track the span of the tokens so we can produce contextual error messages in the parser, and it is also lacking a POSIX-compatibility mode (trivial). The parser is mostly done, although it probably has some rough edges, and the preprocessor is TBD; also, the Pratt routine is a bit spaghetti and should be refactored (trivial). The interpreter is also TBD; work on it will be started once we get good testing on the parser. Expect this to be a fast-paced repo.

## Contributing

See [this](https://github.com/uutils/coreutils/blob/main/CONTRIBUTING.md).

## License

This is licensed under either the MIT License or the Apache License v2.0. See the `LICENSE-MIT` and `LICENSE-APACHE` files for details.
