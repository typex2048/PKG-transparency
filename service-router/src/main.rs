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

//extern crate merkle_tree;
#![feature(test)]
extern crate test;

use map_vec::Map; // Provides a Map-like API but with smaller constant factors.
use oasis_std::{Address, Context};
//use std::fmt::Display;
use std::string::ToString;
//use test::Bencher;

extern crate time;
use time::PreciseTime;


#[derive(oasis_std::Service)]
struct Router{
    users_contract_map: Map<String, String>,
}

type Result<T> = std::result::Result<T, String>; 

impl Router{
	// new construction
	pub fn new(ctx: &Context) -> Self {
		Self {
			users_contract_map: Map::new(),
		}
	}

	// get the user contract address 
	pub fn set_user_contract_address(&mut self, _ctx: &Context, usedid: String, contractId: String ) -> Result<()> {
		self.users_contract_map.insert(usedid, contractId);
		Ok(())
	}

	// get the user contract address 
	pub fn get_user_contract_address(&mut self, _ctx: &Context, usedid: String) -> Result<String> {
		let addr = self.users_contract_map.get(&usedid).map(String::as_str).unwrap_or("no address found").to_string();
		Ok(addr)
	}
}

fn main() {
    oasis_std::service!(Router);
}

#[cfg(test)]
mod tests{
    // This is required even in Rust 2018. If omitted, rustc will not link in the testing
    // library and will produce a giant error message.
    extern crate oasis_test;
    //extern crate test;
    use super::*;

    /// Creates a new account and a `Context` with the new account as the sender.
    fn create_account() -> (Address, Context) {
        let addr = oasis_test::create_account(0 /* initial balance */);
        let ctx = Context::default().with_sender(addr).with_gas(100_000);
        (addr, ctx)
    }

    #[test]
    fn functionality(){
       let (_admin, admin_ctx) = create_account();
       let mut router = Router::new(&admin_ctx); 
	   let rs = router.set_user_contract_address(&admin_ctx, "test".to_string(),"339f05d5d4b3487f52d3d7988d4d9dbb7d0165".to_string());
	   let ad = router.get_user_contract_address(&admin_ctx, "test".to_string());
	   println!("test address is {:?}", ad); 
    }
}

