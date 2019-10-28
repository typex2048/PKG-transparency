
//'''
//* based on Merkle tree, Merkle, 
//  R. C. (1988). "A Digital Signature Based on a Conventional Encryption Function". CRYPTO '87. 
//* code reference: 
//  https://github.com/oasislabs/tutorials
//  https://github.com/shahn/merkle-rs
//  https://github.com/BlockTechCert/BTCert/blob/master/src
//    /main/java/org/bham/btcert/utils/merkle/MerkleTree.java  
//  https://github.com/aleksuss/merkle-tree
//:Authors:    Anonymous works
//:Date:       10/2019
//'

#![feature(test)]
use std::collections::{BTreeMap, VecDeque};
use std::collections::vec_deque::Iter;
use std::fmt::Display;
use std::rc::Rc;

use crate::element::Element;
use crate::merkle_hash::*;
use crate::merkle_proof::Proof;

#[derive(Debug)]
pub enum ProofNode {
    Left(String),
    Right(String),
}

#[derive(Debug)]
pub struct MerkleTree<T: ToString + Display + Clone> {
    root: Element<T>,
    height: usize,
    count: usize,
    storage: VecDeque<Rc<T>>,
    nodes: BTreeMap<usize, VecDeque<Element<T>>>,
}

impl<T: ToString + Display + Clone> MerkleTree<T> {

    pub fn new() -> Self {
        MerkleTree {
            root: Element::empty(),
            height: 0,
            count: 0,
            storage: VecDeque::new(),
            nodes: BTreeMap::new(),
        }
    }

    pub fn from_vec(data: Vec<T>) -> Self {
        if data.is_empty() {
            Self::new()
        } else {
            let elements = data.into_iter()
                .map(|e| Rc::new(e))
                .collect::<VecDeque<Rc<T>>>();
            let mut result = MerkleTree {
                root: Element::empty(),
                height: 0,
                count: 0,
                storage: elements,
                nodes: BTreeMap::new(),
            };
            //result.calculate_tree();
            result
        }
    }

    
    pub fn push(&mut self, value: T) {
        self.storage.push_back(Rc::new(value));
        self.count = self.storage.len();
        //self.calculate_tree();
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if let Some(v) = self.storage.get(index) {
            Some(v.as_ref())
        } else {
            None
        }
    }

    pub fn get_values(&self) -> Option<Vec<T>> {
        if self.storage.is_empty() {
            None
        } else {
            let values = self.storage
                .iter()
                .map(|v| v.as_ref().clone())
                .collect::<Vec<T>>();
            Some(values)
        }
    }

    
    pub fn len(&self) -> usize {
        self.count
    }
    
    pub fn height(&self) -> usize {
        self.height
    }

    
    pub fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }


    
    pub fn root_hash(&self) -> Option<&String> {
        self.root.hash()
    }
 
    pub fn iter(&self) -> Iter<Rc<T>> {
        self.storage.iter()
    }
  
    pub fn get_proof(&self, value: T) -> Proof<T> {
        let path = self.get_needed_hashes_for_proof(&value);
        Proof::new(self.root_hash().unwrap().clone(), value.clone(), path)
    }

    pub fn calculate_tree(&mut self) {
        self.count = self.storage.len();
        self.height = calculate_height(self.count);
        self.root = Element::empty();
        self.nodes.clear();
        let mut current_level = self.height;

        if !self.storage.is_empty() {
            let mut leaves = VecDeque::new();
            for value in &self.storage {
                let e = Element::create_leaf(value.clone());
                leaves.push_back(e);
            }
            self.nodes.insert(current_level, leaves);
            while current_level > 0 {
                let above_level = current_level - 1;
                let above_row = {
                    let mut row = VecDeque::new();
                    let current_row = self.nodes.get(&current_level).unwrap();
                    for i in (0..current_row.len()).step_by(2) {
                        let left = current_row.get(i).unwrap();
                        let right = current_row.get(i + 1).unwrap_or(left);
                        let node = Element::create_node(left.clone(), right.clone());
                        row.push_back(node);
                    }
                    row
                };
                self.nodes.insert(above_level, above_row);
                current_level -= 1;
            }
            assert!(current_level == 0);
            self.root = self.nodes.get(&0).unwrap()[0].clone(); //root_node;
        }
    }

    fn get_needed_hashes_for_proof(&self, value: &T) -> Vec<ProofNode> {
        let mut level = self.height;
        let mut next_hash = create_leaf_hash(&value);
        let mut needed_hashes = Vec::new();

        while level > 0 {
            if let Some(index) = self.get_element_index(level, &next_hash) {
                let nodes = self.nodes.get(&level).unwrap();
                match nodes.get(index) {
                    Some(&Element::Leaf { ref hash, .. }) |
                    Some(&Element::Node { ref hash, .. }) => {
                        if index % 2 == 0 {
                            if let Some(sibling_node) = nodes.get(index + 1) {
                                needed_hashes.push(ProofNode::Right(sibling_node
                                                                        .hash()
                                                                        .unwrap()
                                                                        .clone()));
                                next_hash = create_node_hash(hash, sibling_node.hash().unwrap());
                            } else {
                                needed_hashes.push(ProofNode::Right(hash.clone()));
                                next_hash = create_node_hash(hash, hash);
                            }
                        } else {
                            if let Some(sibling_node) = nodes.get(index - 1) {
                                needed_hashes.push(ProofNode::Left(sibling_node
                                                                       .hash()
                                                                       .unwrap()
                                                                       .clone()));
                                next_hash = create_node_hash(sibling_node.hash().unwrap(), hash);
                            }
                        }
                    }
                    _ => continue,
                };
            }
            level -= 1;
        }
        needed_hashes
    }

    fn get_element_index(&self, level: usize, hash: &String) -> Option<usize> {
        let row_hashes = self.nodes
            .get(&level)
            .unwrap()
            .iter()
            .map(|e| e.hash().unwrap())
            .collect::<Vec<&String>>();
        row_hashes.iter().position(|&s| s == hash)
    }
}



pub fn calculate_height(count: usize) -> usize {
    if count > 0 {
        let height = (count as f64).log2();
        if height - height.floor() > 0.0 {
            (height + 1.0) as usize
        } else {
            height as usize
        }
    } else {
        0
    }
}

/*
#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_empty_tree_hash() {
	    let db: MerkleTree<u32> = MerkleTree::new();
	    assert_eq!(&"5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9".to_string(),
	               db.root_hash().unwrap_or(&"None".to_string()));
	}
}*/
