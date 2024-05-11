impl Node {
    pub fn new(value: i32) -> Node {
        Node {
            value: value,
            next: None,
        }
    }
    pub fn add_node(&mut self, value: i32) {
        if self.next.is_none() {
            let n = Box::new(Node::new(value));
            let new = Some(n);
            self.next = new;
        } else {
            let child = &mut self.next.as_mut().unwrap();
            child.add_node(value);
        }
    }
}
#[derive(Clone, Debug)]
pub struct Node {
    pub value: i32,
    pub next: Option<Box<Node>>,
}
