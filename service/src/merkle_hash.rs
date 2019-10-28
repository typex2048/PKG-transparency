//'''
//* based on Merkle tree, Merkle, 
//  R. C. (1988). "A Digital Signature Based on a Conventional Encryption Function". CRYPTO '87. 
//* code reference: 
//  https://github.com/oasislabs/tutorials
//  https://github.com/shahn/merkle-rs
//  https://github.com/BlockTechCert/BTCert/blob/master/src
//    /main/java/org/bham/btcert/utils/merkle/MerkleTree.java  
//  https://github.com/aleksuss/merkle-tree
//  https://docs.rs/blake2/0.8.1/blake2/
//:Authors:    Anonymous works
//:Date:       10/2019
//''

use blake2::{Blake2b, Digest};
use std::string::ToString;

pub fn empty_hash() -> String {
    create_leaf_hash(&0)
}

pub fn create_leaf_hash<T: ToString>(input: &T) -> String {
    let mut hasher = Blake2b::new();
    hasher.input(&input.to_string());
    let result = format!("{:?}", hasher.result());
    result
}

pub fn create_node_hash<T: ToString>(left: &T, right: &T) -> String {
    let mut hasher = Blake2b::new();
    hasher.input(left.to_string());
    hasher.input(right.to_string());
    let result = format!("{:?}", hasher.result());
    result
}
