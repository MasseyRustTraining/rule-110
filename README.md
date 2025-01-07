# Rule 110
Bart Massey 2025

In this exercise you will create an ASCII rendering of the
[Rule 110](https://en.wikipedia.org/wiki/Rule_110)
[Elementary Cellular
Automaton](https://en.wikipedia.org/wiki/Elementary_cellular_automaton)
(CA). Not as scary as it sounds, and a nice simple Rust
thing to play with.


A CA starts with a random row of bits. We will represent 1
bits with `*` and 0 bits with `.`. Our starting
row will specifically be for now.

    *.*.**..

To produce the next row, take bits three at a time,
"wrapping around" if a boundary is hit.  A group of three
bits in row *n* will determine the center bit position in
row *n + 1* according to Rule 110:

    111 → 0
    110 → 1
    101 → 1
    100 → 0
    011 → 1
    010 → 1
    001 → 1
    000 → 0

So for our start, the first two rows will be

    *.*..*..
    ***.**.*

(Notice the wraparound at the beginning and end.)

Write a program that prints these two rows and then keeps
going.

## Hints

* You probably should use either `u8` or `[bool; 8]` to represent a row. 
  The second thing is easier.

* If you print out as you go, you only need to keep track of
  two rows: the current row and the next row.

* You should probably write a function `rule110()` with a
  signature something like
  ```rust
  fn rule110(bits: [bool; 3]) -> bool {
      todo!()
  }
  ```
  in the array case, or 
  ```rust
  fn rule110(bits: u8) -> u8 {
      todo!()
  }
  ```
  in the bit case.

## Challenges

* Make your program take a command-line argument for the
  starting row. You may limit the length of the argument.
  You can get at the argument with this mystic rune that we
  will explain later:

  ```rust
  let arg = std::env::args().nth(1).unwrap();
  ```

* Time your program. See how fast you can make it
  go. Compare performance with a C or C++ implementation.
