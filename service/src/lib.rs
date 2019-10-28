#![feature(test)]
//#[warn(non_camel_case_types)]

extern crate blake2;
//extern crate test;

mod element;
mod merkle_hash;
mod merkle_tree;
mod merkle_proof;

//mod tests;

pub use self::merkle_tree::MerkleTree;
pub use self::merkle_proof::Proof;
//pub use self::test::*;