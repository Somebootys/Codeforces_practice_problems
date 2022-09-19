#![allow(unused)]
#![allow(unused)]

fn is_even(x: u8) -> bool {
    x % 2 == 0
}

fn main() {
    let mut i: u8 = 1;
    loop {
        if is_even(i) {
            println!("YES");
        } else {
            println!("NO");
        }
        i += 1;
        if i > 10{
            break;
        }
    }
}
