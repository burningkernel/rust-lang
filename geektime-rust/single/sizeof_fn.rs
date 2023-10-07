use std::{collections::HashMap, mem::size_of_val};

fn main() {
    let c1 = || println!("hello world!");
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("Kernel");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");

    let c3 = || println!("hello: {name}");
    let c4 = move || println!("hello: {name1}, {:?}", table);
    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("test");
        println!("hello: {x}, {:?}, {:?}", name2, name3);
    };

    println!(
        "c1: {}, c2: {}, c3: {}, c4: {}, c5: {}, main: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&main),
    );

    let c = move | greeting: String| (greeting, name);
    let result = c("hello".to_string());
    println!("result: {:?}", result);
}
