use linkedlist::util::linked_list::to_list;
use linkedlist::*;

fn main() {
    println!("has cycle: {:?}", has_cycle(to_list(vec![1, 2, 3, 4, 5])));
    println!("merge: {:?}", merge_two_lists(to_list(vec![1, 3, 4]),
        to_list(vec![1, 2, 4])));

    println!("midle: {:?}", middle_node(to_list(vec![1, 5, 7, 8])));
}
