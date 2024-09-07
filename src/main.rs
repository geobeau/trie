use std::{cell::RefCell, rc::Rc};


type NodePointer = Rc<RefCell<Node>>;

struct Node {
    // Prefix store the common bits until reaching offset
    prefix: u8,
    // Offset points at the bit that is different
    offset: u8,
    // Left child is the source child (with prefix)
    left_child: Option<NodePointer>,
    // Right child is the one with a different bit
    right_child: Option<NodePointer>
}

// 3 cases
// leaf -> No child
// One child -> sub tree continuation (offset=u8::MAX)
// two child -> intermediary node


impl Node {

    pub fn new(value: &[u8]) -> Self {
        assert!(!value.is_empty(), "Value should not be empty");
        let child = if value.len() > 1 {
            // Recursively create child nodes
            Some(Rc::new(RefCell::new(Node::new(&value[1..]))))
        } else {
            None
        };
        Node { prefix: value[0], offset: u8::MAX, left_child: child, right_child: None }
    }

    pub fn insert(&mut self, value: &[u8]) {
        let diff = value[0] ^ self.prefix;
        let new_offset = diff.leading_zeros();
        if diff == 0 {
            // Both value[0] and self.prefix are equal
            let mut left_child_mut = self.left_child.unwrap().borrow_mut();
            return left_child_mut.insert(&value[1..])
        }
        if new_offset > (self.offset as u32) {
            if self.right_child.is_some() {
                self.split();
            }
            return 
        } else if new_offset == (self.offset as u32) {
            let mut right_child_mut = self.right_child.unwrap().borrow_mut();
            return right_child_mut.insert(&value[1..])
        } else {
            self.split_before()
        }


    }

}



struct Patricia {
    root: Option<NodePointer>
}

impl Patricia {
    pub fn new() -> Self {
        Patricia{ root: None }
    }

    pub fn insert(&mut self, value: Vec<u8>) {
        match &mut self.root {
            Some(root) => root.as_ref().borrow_mut().insert(&value),
            None => self.root = Some(Rc::new(RefCell::new(Node::new(&value)))),
        }
    }

    pub fn exists(&self, _: Vec<u8>) -> bool {
        return false
    }
}

fn main() {
    let data1 = vec![10u8; 3];
    let data2 = vec![11u8; 3];
    let data3 = vec![12u8; 3];

    let mut patricia = Patricia::new();
    patricia.insert(data1);
    patricia.insert(data2);
    patricia.insert(data3);

    assert!(patricia.exists(data1));
}
