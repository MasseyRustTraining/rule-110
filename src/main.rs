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

fn main() {
    let mut cur = make_start("*.*..*..");
    let mut next = [false; N];
    for _ in 0..10 {
        for i in 0..N {
            let c = if cur[i] {
                '*'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();

        for i in 0..N {
            let bits = [cur[i], cur[(i + 1) % N], cur[(i + 2) % N]];
            let next_bit = rule_110(bits);
            next[(i + 1) % N] = next_bit;
        }
        cur = next;
    }
}
