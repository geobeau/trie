use std::{cell::RefCell, ops::BitXor, rc::Rc};
use bitintr::{self, Pdep, Pext, Popcnt};

const N: usize = 32;

pub struct HotNode {
    /// Number of keys and pointers in the node
    key_count: u8,
    reference: u64,
    mask: u64,
    partial: [u8; N],
    /// Children store a random value at the moment for testing
    children: [u64; N]
}

impl HotNode {
    pub fn new(key: u64, value: u64) -> Self {
        let mut node = HotNode {
            key_count: 1,
            mask: 0,
            reference: key,
            partial: [0; N],
            children: [0; N],
        };
        node.partial[0] = 0;
        node.children[0] = value;  // 2 Means leaf for testing atm
        return node
    }

    pub fn _insert_at_idx(&mut self, key: u64, value: u64) {

    }

    pub fn _update_mask(&mut self, new_mask: u64) {
        println!(">> modified mask: {:#b}", self.mask);
        let discriminating_bits_count = new_mask.popcnt();
        assert!(discriminating_bits_count <= 8);
        println!(">> discriminating_bits_count: {}", discriminating_bits_count);

        for i in 0..self.key_count {
            let old_partial = (self.partial[i as usize] as u64);
            let determinant_bits = old_partial.pdep(self.mask);
            let original_key = determinant_bits ^ self.reference;
            self.partial[i as usize] = original_key.pext(new_mask) as u8;
            println!(">> updating idx {i} with partial {:#b} from {:#b}. {determinant_bits:#b}/{original_key}", self.partial[i as usize], old_partial)
        } 
        println!(">> replaced mask with : {:#b}", new_mask);
        self.mask = new_mask;
    }


    pub fn insert(&mut self, key: u64, value: u64) {
        println!("Inserting with {:#b} ({})", key, key);
        // diff is a mask of bits that are different between the ref the key
        let diff = key ^ self.reference;
        // Check if diff not contained in the mask
        // In this case, the mask needs to be expended
        println!("> mask: {:#b}", self.mask);
        if (self.mask & diff) != diff {
            self._update_mask(diff | self.mask)
        }



        let dense_key = key.pext(diff);
        println!("> insert dense key: {:#b}", dense_key);

        println!("> Setting at {}: {:#b}", self.key_count, dense_key);
        self.partial[self.key_count as usize] = dense_key as u8;
        self.children[self.key_count as usize] = value;
        self.key_count += 1;
    }

    pub fn search(&self, key: u64) -> Option<u64> {
        println!("Searching with {:#b}", key);
        // diff is a mask of bits that are different between the ref the key
        let diff = key ^ self.reference;
        // Check if diff not contained in the mask
        // if it's not, the key is not contained
        if (self.mask & diff) != diff {
            println!("> Not contained within mask");
            return None;
        }

        let dense_key = key.pext(self.mask) as u8;
        println!("> search dense key: {:#b}", dense_key);
        for i in 0..self.key_count {
            println!("> compare with dense key: {:#b}", self.partial[i as usize]);
            if dense_key == self.partial[i as usize] {
                return Some(self.children[i as usize])
            } 
        }

        return None
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut hn = HotNode::new(1, 1);
        hn.insert(2, 2);
        hn.insert(3, 3);
        hn.insert(7, 7);
        hn.insert(4, 4);
        assert_eq!(hn.search(7), Some(7));
        assert_eq!(hn.search(4), Some(4));
        assert_eq!(hn.search(3), Some(3));
        assert_eq!(hn.search(2), Some(2));
        assert_eq!(hn.search(1), Some(1));
        hn.insert(3, 3);
    }


}