//! Rule 118 CA — Bart Massey 2025
//!
//! Solution to <https://github.com/MasseyRustTraining/rule-110>.

/// Row width.
const N: usize = 8;

/// A row is an array of bits.
type Row = [bool; N];

/// Given a starting row description string, make and return
/// the corresponding row.
fn make_start(row: &str) -> Row {
    if row.chars().count() != N {
        panic!("make_start: string length not equal to {}", N);
    }
    let mut bits = Row::default();
    for (i, c) in row.chars().enumerate() {
        match c {
            '*' => bits[i] = true,
            '.' => bits[i] = false,
            _ => panic!("bad char {}", bits[i]),
        }
    }
    bits
}

/// CA Rule 110. <https://en.wikipedia.org/wiki/Rule_110>
fn rule_110(bits: [bool; 3]) -> bool {
    match bits {
        [true, true, true] => false,
        [true, true, false] => true,
        [true, false, true] => true,
        [true, false, false] => false,
        [false, true, true] => true,
        [false, true, false] => true,
        [false, false, true] => true,
        [false, false, false] => false,
    }
}

/// Produce the renderable string for the current row.
fn row_string(cur: Row) -> String {
    cur.into_iter().map(|b| if b { '*' } else { '.' }).collect()
}

/// Use CA Rule 110 to make a new row.
fn next_row(cur: Row) -> Row {
    let mut next = [false; N];
    for i in 0..N {
        let bits = [cur[i], cur[(i + 1) % N], cur[(i + 2) % N]];
        let next_bit = rule_110(bits);
        next[(i + 1) % N] = next_bit;
    }
    next
}

#[test]
fn test_second_row() {
    let cur = make_start("*.*..*..");
    let next = next_row(cur);
    assert_eq!(next, make_start("***.**.*"));
}

/// Make the first 10 rows of the required output.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nrows: usize = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        10
    };
    let row0 = if args.len() > 2 {
        &args[2]
    } else {
        "*.*..*.."
    };
    let mut cur = make_start(row0);
    for _ in 0..nrows {
        println!("{}", row_string(cur));
        cur = next_row(cur);
    }
}
