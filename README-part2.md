# Rule 110 Library
Bart Massey 2025

In [Part 1](README-part1.md) you built a Rule 110
command-line program. In this exercise you will turn it into
a library with a sample application.

* The library, in `src/lib.rs`, should contain a sensible
  implementation of a `Rule110` struct and methods for
  working with it. Provide an API that would make sense for
  general exploration of Rule 110.
  
  This would be a good time to go ahead and allow
  constructing the starting row from a formatted input
  string of arbitrary length.

  Your methods should not panic: you will need to define a
  `Rule110Error` type that implements `std::error::Error`
  and use it to return error reports as needed.
  
  Hide things that should be private. Use modules as needed
  to isolate code.

Both library and demo code should have Rustdoc comments,
be formatted according to Rustfmt standards, and build
without compiler or Clippy warnings.

## Bonus

* The demo application, in `examples/rule110.rs`, should
  accept at least a command-line argument for the number of
  rows to be printed. If you allow arbitrary input, that
  should also be a command-line argument.
  
  Use `[dev-dependencies]` in `Cargo.toml` for dependencies
  needed by the demo application but not the library.

* Remove the dependency on Rule 110. Allow the user to
  specify an arbitrary CA rule for both the program and the
  library.
