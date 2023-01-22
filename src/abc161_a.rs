use proconio::input;

fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
    }
    let ab = b;
    let ba = a;
    let ac = c;
    let ca = ab;
    println!("{} {} {}", ac, ba, ca);
}