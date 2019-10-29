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
use typex::MerkleTree;
use typex::Proof;

use map_vec::Map; // Provides a Map-like API but with smaller constant factors.
use oasis_std::{Address, Context};
//use std::fmt::Display;
use std::string::ToString;
use test::Bencher;

extern crate time;
use time::PreciseTime;


#[derive(oasis_std::Service)]
struct TypeX{
    user_ids: Vec<String>,
    admin: Address,
    public_parameters: Vec<String>,
    users_proof_map: Map<String, String>,
}

type Result<T> = std::result::Result<T, String>; 


impl TypeX{

    // new construction
    pub fn new(ctx: &Context) -> Self {
        Self {
            user_ids: Vec::new(),
            admin: ctx.sender(),
            public_parameters: vec![], 
            users_proof_map: Map::new(),
        }
    }

    // register the public_parameters 
    // p,q,n are in the public_parameters.
    pub fn register_param(&mut self, ctx: &Context, public_parameters: Vec<String>) -> Result<()> {
        if self.admin != ctx.sender() {
            return Err("Only the amdin can register the public_parameters.".to_string());
        }
        self.public_parameters = public_parameters;
        Ok(())
    }

    // obtain the public_parameters 
    pub fn get_param(&self, _ctx: &Context) -> Vec<&str> {
        self.public_parameters.iter().map(String::as_ref).collect()
    }

    // register the public_parameters 
    pub fn insert_user(&mut self, _ctx: &Context, usedid: String) -> Result<Vec<String>> {
        // ensure there is no ambiguous id
        if self.user_ids.contains(&usedid){
			return Err("The user has already registered.".to_string());
        }
	    self.user_ids.push(usedid);
        Ok(self.user_ids.clone())
    }

    // register the public_parameters 
    pub fn batch_insert_user(&mut self, _ctx: &Context, usedids: Vec<String>) -> Result<()> {
	    self.user_ids = usedids;
        Ok(())
    }

    // the assumption: there are already 10000 users in the system
    pub fn prepare_assumption_data(&mut self, _ctx: &Context) -> Result<()> {
       let n = 35;
       for i in 1..n {
            self.user_ids.push(i.to_string());
       }
       Ok(())
    }

     // register a new user
    pub fn register_user(&mut self, _ctx: &Context, usedid: String) -> Result<String> {
        
        let mut usedid_clone = usedid.clone();

        // insert a new user
        if self.user_ids.contains(&usedid){
			return Err("The user has already registered.".to_string());
        }
        self.user_ids.push(usedid);
        let uids = self.user_ids.clone();
        
        //let mut db = MerkleTree::from_vec(self.user_ids.collect::<Vec<_>>());
        // get the proof
        let mut db = MerkleTree::new();
        for uid in uids.iter(){
	       db.push(uid);
        }
        db.calculate_tree();
        let userid_proof = db.get_proof(&usedid_clone);
        let pfs = format!("{:?}",userid_proof).to_string(); 
        let pfs_clone = pfs.clone();
        self.users_proof_map.insert(usedid_clone, pfs);
        Ok(pfs_clone)
    }

    // register the public_parameters 
    pub fn get_all_users(&mut self, _ctx: &Context) -> Result<Vec<String>> {
        if self.admin != _ctx.sender() {
            return Err("Only the amdin can get all the users.".to_string());
        }
        Ok(self.user_ids.clone())
    }
    
    // get the proofs
    pub fn get_proof(&mut self, _ctx: &Context, usedid: String) -> Result<String> {
        //println!("11111111111 {:?}",self.users_proof_map);
        let pfs = self.users_proof_map.get(&usedid).map(String::as_str).unwrap_or("default string").to_string();
        //let pfs = format!("{:?}",userid_proof).to_string(); .map(String::as_str) 
        Ok(pfs)
    }
}

fn main() {
    oasis_std::service!(TypeX);
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
       let mut typex = TypeX::new(&admin_ctx); 

      /*
        prepare the public_parameters [batch]
        the assumption: there are already 100000000 users in the system
       */
       println!("[batch] prepare the data [start]");
       let mut uids = Vec::new();
       let n = 100;
       for i in 1..n {
            uids.push(i.to_string());   
            println!("[batch] insert {} user without calculating the proof",i);
       }
       typex.batch_insert_user(&admin_ctx,uids);
       println!("[batch] prepare the data [end]");
 
       /*
        prepare the public_parameters
        the assumption: there are already 100000000 users in the system
       
       println!("prepare the data [start]");
       let n = 100;
       for i in 1..n {
            typex.insert_user(&admin_ctx, i.to_string());
            println!("insert {} user without calculating the proof",i);
       }

       println!("prepare the data [end]");
       */

       /*
       test case 1 for register_param
       */
       let start_0 = PreciseTime::now();
       // code start
       let public_parameters = vec!["1F1tAaz5x1HUXrCNLbtMDqcw6o5GNn4xqX".to_string(), "3P3QsMVK89JBNqZQv5zMAKG8FK3kJM4rjt".to_string()];
       typex.register_param(&admin_ctx,public_parameters);
       //code end
       let end_0 = PreciseTime::now();
       println!("{} seconds for registering the public parameters", start_0.to(end_0));

       /*
       test case 2 for register_param
       */
       //test cased
       let start_1 = PreciseTime::now();
       // code start
       let pp = typex.register_user(&admin_ctx, "test".to_string());
       //println!("the proof is {:?}",pp);
       //code end
       let end_1 = PreciseTime::now();
       println!("{} seconds for register one user, [ current registered user numbers: {} ]", start_1.to(end_1), n);

       /*
       test case 3 for get_proof
       */
       let start_2 = PreciseTime::now();
       // code start
       //let proof = typex.get_proof(&admin_ctx, "4".to_string());
       //println!("the proof is {:?}", proof);
       let proof1 = typex.get_proof(&admin_ctx, "test".to_string());
       //println!("the proof is {:?}", proof1);
       //code end
       let end_2 = PreciseTime::now();
       println!("{} seconds for getting the proof for a random user, [ current registered user numbers: {} ]", start_2.to(end_2), n);
    }

    //#[bench]
    //fn benchmark_creation_from_vec_with_1000_elements(b: &mut Bencher) {
        // let (_admin, admin_ctx) = create_account();
        //b.iter(|| {
               //for i in 1..2 {
                //let mut typex = TypeX::new(&admin_ctx);
                //typex.register_user(&admin_ctx, "2".to_string());
                //println!("{:?}", i);
               //}
        //});
    //}
}

