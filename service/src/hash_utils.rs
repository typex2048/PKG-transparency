
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

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::string::ToString;

pub fn empty_hash() -> String {
    create_leaf_hash(&0)
}

pub fn create_leaf_hash<T: ToString>(input: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(&input.to_string().as_ref());
    let result = hasher.result_str();
    result
}

pub fn create_node_hash<T: ToString>(left: &T, right: &T) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(left.to_string().as_ref());
    hasher.input_str(right.to_string().as_ref());
    let result = hasher.result_str();
    result
}
