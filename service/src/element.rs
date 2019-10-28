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

use std::fmt::Display;
use std::rc::Rc;

use crate::merkle_hash::*;

#[derive(Clone, Debug)]
pub enum Element<T: ToString + Display> {
    Node {
        left_node: Box<Element<T>>,
        right_node: Box<Element<T>>,
        hash: String,
    },
    Leaf { data: Rc<T>, hash: String },
    Empty { hash: String },
}

impl<T: Display> Element<T> {
    pub fn empty() -> Self {
        Element::Empty { hash: empty_hash() }
    }

    pub fn hash(&self) -> Option<&String> {
        match *self {
            Element::Node { ref hash, .. } |
            Element::Leaf { ref hash, .. } |
            Element::Empty { ref hash } => Some(hash),
        }
    }

    pub fn create_leaf(value: Rc<T>) -> Element<T> {
        let leaf_hash = create_leaf_hash(value.as_ref());

        Element::Leaf {
            data: value,
            hash: leaf_hash,
        }
    }

    pub fn create_node(left: Element<T>, right: Element<T>) -> Element<T> {
        let combined_hash = create_node_hash(left.hash().unwrap(), right.hash().unwrap());
        Element::Node {
            hash: combined_hash,
            left_node: Box::new(left),
            right_node: Box::new(right),
        }
    }
}
