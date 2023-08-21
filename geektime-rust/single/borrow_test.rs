use std::borrow::Borrow;

fn main() {
    let s = "Hello world!".to_owned();

    let r1: &String = s.borrow();
    let r2: &str = s.borrow();

    println!("r1: {:p}, r2: {:p}, s: {:p}", r1, r2, &s);
}
