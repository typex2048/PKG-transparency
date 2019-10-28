
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
//''

use std::fmt::Display;

use crate::merkle_tree::ProofNode;
use crate::merkle_hash::{create_leaf_hash, create_node_hash};

#[derive(Debug)]
pub struct Proof<T: Display> {
    pub root_hash: String,
    pub value: T,
    pub path: Vec<ProofNode>,
}

impl<T> Proof<T>
    where T: Display
{
    pub fn new(root_hash: String, value: T, path: Vec<ProofNode>) -> Self {
        Proof {
            root_hash: root_hash,
            value: value,
            path: path,
        }
    }

    pub fn validate(&self, root_hash: &str) -> bool {
        let mut hash = create_leaf_hash(&self.value);
        for node in &self.path {
            hash = match node {
                &ProofNode::Left(ref proof_hash) => create_node_hash(proof_hash, &hash),
                &ProofNode::Right(ref proof_hash) => create_node_hash(&hash, proof_hash),
            };
        }
        hash == root_hash
    }
}
