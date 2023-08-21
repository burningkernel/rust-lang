use std::collections::HashMap;

fn main() {
    let map = HashMap::new();
    let mut map = explain("empty", map);

    map.insert('a', 1);
    let mut map = explain("added 1", map);
    map.insert('b', 2);
    map.insert('c', 3);
    let mut map = explain("added 3", map);
    map.insert('d', 4);
    let mut map = explain("added 4", map);
    map.remove(&'a');
    explain("final", map);
}

fn explain<K, V>(name: &str, map: HashMap<K, V>) -> HashMap<K, V> {
    let arr: [usize; 6] = unsafe { std::mem::transmute(map) };
    println!(
        "{}: bucket_mask 0x{:x}, ctrl 0x{:x}, growth_left: 0x{:x}, items: 0x{:x}, array[0]: 0x{:x}, array[1]: 0x{:x}",
        name, arr[2], arr[3], arr[4], arr[5], arr[0], arr[1]
    );
    unsafe { std::mem::transmute(arr) }
}
