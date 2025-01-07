const N: usize = 8;
type Row = [bool; N];

fn make_start(row: &str) -> Row {
    let row = row.as_bytes();
    let mut bits = [false; N];
    for i in 0..N {
        match row[i] {
            b'*' => bits[i] = true,
            b'.' => bits[i] = false,
            _ => panic!("bad char {}", bits[i]),
        }
    }
    bits
}

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

fn row_string(cur: Row) -> String {
    let mut row = String::new();
    for i in 0..N {
        if cur[i] {
            row += "*";
        } else {
            row += ".";
        }
    }
    row
}

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

fn main() {
    let mut cur = make_start("*.*..*..");
    for _ in 0..10 {
        println!("{}", row_string(cur));
        cur = next_row(cur);
    }
}

