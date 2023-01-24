use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    let result;
    if x == 0{ result = 1 } else { result = 0};
    println!("{}", result);
}
