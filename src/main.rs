mod node;
use crate::node::Node;
fn main() {
    let mut linked_list = Node::new(0);
    for i in 1..10 {
        linked_list.add_node(i);
    }

    println!("linked_list -> {:?}", linked_list);
    while !linked_list.next.is_none() {
        println!("value -> {}", linked_list.value);
        linked_list = *linked_list.next.unwrap();
    }

    println!("last value -> {}", linked_list.value);
}
