fn three_volwels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a'| 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true
                }
            }
            _ => vowel_count = 0
        }
    }
    false
}
fn main() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{ferris}: {}", three_volwels(&ferris));
    println!("{curious}: {}", three_volwels(&curious));
}
