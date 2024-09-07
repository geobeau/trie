use std::{cell::RefCell, process::Child, rc::Rc};


type NodePointer = Rc<RefCell<Node>>;

struct Node {
    /// Prefix store the common bits until reaching offset
    prefix: u8,
    /// Offset points at the bit that is different
    offset: u8,
    /// Depth is the byte position in the original byte array
    depth: u8,
    /// leaf is true if a record ended there. It might still have
    /// children! (because records have varied length) 
    leaf: bool,
    /// Left child is the source child (with prefix)
    left_child: Option<NodePointer>,
    /// Right child is the one with a different bit
    right_child: Option<NodePointer>
}

// 3 cases
// leaf -> No child
// One child -> sub tree continuation (offset=u8::MAX)
// two child -> intermediary node


impl Node {

    pub fn new(value: &[u8], depth: u8) -> Self {
        let child = if (value.len() - 1) > depth.into() {
            // Recursively create child nodes
            Some(Rc::new(RefCell::new(Node::new(&value, depth+1))))
        } else {
            None
        };
        Node { prefix: value[depth as usize], depth, offset: u8::MAX, leaf: child.is_none(), left_child: child, right_child: None }
    }

    pub fn insert(&mut self, value: &[u8]) {
        let diff = value[self.depth as usize] ^ self.prefix;
        let new_offset = diff.leading_zeros();
        // If there is no diff between the reference and the added value, follow the left child
        if diff == 0 {
            if value.len() -1 == self.depth.into() {
                if self.leaf {
                    // Record already exists in the trie
                    return
                }
                self.leaf = true;
                return;
            }
            let mut left_child_mut = self.left_child.as_ref().unwrap().borrow_mut();
            left_child_mut.insert(&value);
            return
        }

        // If there is a diff but it is on the offset that existed before, follow the right child
        if new_offset == (self.offset as u32) {
            let mut right_child_mut: std::cell::RefMut<'_, Node> = self.right_child.as_ref().unwrap().borrow_mut();
            right_child_mut.insert(&value);
            return
        }

        // At this point, a new branch of the tree is going to be created
        let new_child = Node::new(&value, self.depth);

        if new_offset > (self.offset as u32) {
            self.split_after(new_offset as u8, new_child);
            return
        }  else {
            self.split_before(new_offset as u8, new_child);
            return
        }
    }

    /// This is called if the offset is bigger than the one already existing (implying that a right
    /// child already exists).
    /// 
    /// To add another right child, the current node needs to be duplicated and the new child+offset
    /// added to the duplicata. The duplicata is then added as left child.
    pub fn split_after(&mut self, new_child_offset: u8, new_child: Node) {
        let mut copy_node = Node { prefix: self.prefix, depth: self.depth, offset: self.offset, leaf: self.leaf,
            left_child: self.left_child.clone(), right_child: self.right_child.clone() };
        
        copy_node.offset = new_child_offset;
        copy_node.right_child = Some(Rc::new(RefCell::new(new_child)));
        self.left_child = Some(Rc::new(RefCell::new(copy_node)));
    }

    /// This is called if the current node as an offset bigger than the new one. A right child might
    /// not be present.
    pub fn split_before(&mut self, new_child_offset: u8, new_child: Node) {
        // If there are no right child, just insert the new one here. No need to duplicate the current
        // one.
        if self.right_child.is_none() {
            self.right_child = Some(Rc::new(RefCell::new(new_child)));
            self.offset = new_child_offset;
            return;
        }

        // TODO: would .clone() be similar/better?
        let copy_node = Node { prefix: self.prefix, depth: self.depth, offset: self.offset, leaf: self.leaf,
            left_child: self.left_child.clone(), right_child: self.right_child.clone() };
        
        // Compared to split after, the new child is added to the current node. The old copy of current
        // node is pushed down
        self.offset = new_child_offset;
        self.right_child = Some(Rc::new(RefCell::new(new_child)));
        self.left_child = Some(Rc::new(RefCell::new(copy_node)));
    }

    /// Search if record exists in the trie
    pub fn exists(&self, value: &[u8]) -> bool {
        let diff = value[self.depth as usize] ^ self.prefix;
        let offset = diff.leading_zeros();

        if diff == 0 {
            if value.len() - 1 == self.depth.into() {
                if self.leaf {
                    // Record already exists in the trie
                    return true
                }
                return false;
            }
            if self.leaf {
                return false
            }
            return self.left_child.as_ref().unwrap().borrow().exists(&value)
        }
        // If there is a diff but it is on the offset that existed before, follow the right child
        if offset == (self.offset as u32) {
            return self.right_child.as_ref().unwrap().borrow().exists(&value)
        }

        // Try chance on the next left child
        if offset > (self.offset as u32) {
            return self.left_child.as_ref().unwrap().borrow().exists(&value)
        }
        // if offset < self.offset it means that the record doesn't exists
        return false
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
            None => self.root = Some(Rc::new(RefCell::new(Node::new(&value, 0)))),
        }
    }

    pub fn exists(&self, value: &Vec<u8>) -> bool {
        return match &self.root {
            Some(root) => root.as_ref().borrow().exists(&value),
            None => false,
        }
    }
}

fn main() {
    let data1 = vec![10u8, 11u8, 12u8];
    let data2 = vec![13u8, 14u8, 15u8];
    let data3 = vec![16u8, 17u8, 18u8];
    let data4 = vec![10u8, 11u8, 13u8];
    let data5 = vec![16u8, 11u8, 13u8];

    let mut patricia = Patricia::new();
    patricia.insert(data1.clone());
    assert!(patricia.exists(&data1));
    patricia.insert(data2.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    patricia.insert(data3.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    assert!(patricia.exists(&data3));
    patricia.insert(data4.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    assert!(patricia.exists(&data3));
    assert!(patricia.exists(&data4));
    patricia.insert(data5.clone());
    assert!(patricia.exists(&data1));
    assert!(patricia.exists(&data2));
    assert!(patricia.exists(&data3));
    assert!(patricia.exists(&data4));
    assert!(patricia.exists(&data5));
}
