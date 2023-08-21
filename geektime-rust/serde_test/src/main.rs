use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    age: u8,
}

fn main() {
    let input = r#"{ "name": "BurningKernel", "age": 18}"#;
    let user: User = serde_json::from_str(input).unwrap();

    match user.name {
        Cow::Owned(x) => println!("owned {}", x),
        Cow::Borrowed(x) => println!("borrowed {}", x),
    }
}
