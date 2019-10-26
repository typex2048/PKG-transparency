//extern crate merkle_tree;

#![feature(test)]
extern crate test;
use typex::MerkleTree;
use typex::Proof;

//use map_vec::Map; // Provides a Map-like API but with smaller constant factors.
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
    parameters: Vec<String>,
}

struct proofs {
    pub proof: Proof<String>,
    pub used_id: String,
}

type Result<T> = std::result::Result<T, String>; 

impl TypeX{
    
    // new construction
    pub fn new(ctx: &Context) -> Self {
        Self {
            user_ids: Vec::new(),
            admin: ctx.sender(),
            parameters: vec![], 
        }
    }

    // register the parameters 
    // p,q,n are in the parameters.
    pub fn register_param(&mut self, ctx: &Context, parameters: Vec<String>) -> Result<()> {
        if self.admin != ctx.sender() {
            return Err("Only the amdin can register the parameters.".to_string());
        }
        self.parameters = parameters;
        Ok(())
    }

    // obtain the parameters 
    pub fn get_param(&self, _ctx: &Context) -> Vec<&str> {
        self.parameters.iter().map(String::as_ref).collect()
    }

    // register the parameters 
    pub fn register_user(&mut self, _ctx: &Context, usedid: String) -> Result<Vec<String>> {
        // ensure there is no ambiguous id
        if self.user_ids.contains(&usedid){
			return Err("The user has already registered.".to_string());
        }
	    self.user_ids.push(usedid);
        Ok(self.user_ids.clone())
    }

    // register the parameters 
    pub fn get_all_users(&mut self, _ctx: &Context) -> Result<Vec<String>> {
        if self.admin != _ctx.sender() {
            return Err("Only the amdin can get all the users.".to_string());
        }
        Ok(self.user_ids.clone())
    }
    
    // get the proofs
    pub fn get_proof(&mut self, _ctx: &Context, usedid: String) -> Result<String> {
        let mut db = MerkleTree::new();
        for uid in self.user_ids.iter(){
	       db.push(uid);
        }
        let userid_proof = db.get_proof(&usedid);
        let pfs = format!("{:?}",userid_proof).to_string(); 
        Ok(pfs)
    }
}

fn main() {
    oasis_std::service!(TypeX);
}

#[cfg(test)]
mod tests {
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
    fn test_register_param(){
       let (_admin, admin_ctx) = create_account();
       let mut typex = TypeX::new(&admin_ctx);

       let start = PreciseTime::now();
       // code start

       let parameters = vec!["1F1tAaz5x1HUXrCNLbtMDqcw6o5GNn4xqX".to_string(), "3P3QsMVK89JBNqZQv5zMAKG8FK3kJM4rjt".to_string()];
       typex.register_param(&admin_ctx,parameters);

       //code end
       let end = PreciseTime::now();
       println!("{} seconds for typex.register_param.", start.to(end));
    }

    #[test]
    fn test_users(){
       let (_admin, admin_ctx) = create_account();
       let mut typex = TypeX::new(&admin_ctx);
       
       // this is the assumption: there are already 100000000 users in the system
       let n = 100000000;
       for i in 1..n {
            let mut typex = TypeX::new(&admin_ctx);
            typex.register_user(&admin_ctx, i.to_string());
       }

       let start = PreciseTime::now();
       // code start
       typex.register_user(&admin_ctx, "test".to_string());
       //code end
       let end = PreciseTime::now();
       println!("{} seconds for typex.register_user for {} users", start.to(end),n);
    }

    #[test]
    fn functionality() {
        let (_admin, admin_ctx) = create_account();
        let (_voter, voter_ctx) = create_account();
       
        let mut typex = TypeX::new(&admin_ctx);

        let start = PreciseTime::now();
        let mut n = 1000;
        for i in 1..n {
           typex.register_user(&admin_ctx, i.to_string());
        }  
        let end = PreciseTime::now();
        println!("{} seconds for typex.register_user {}.", start.to(end), n);

        //typex.register_user(&admin_ctx, "yogurt".to_string()); 
        //typex.register_user(&admin_ctx, "test".to_string());
        //let test_ok0 = typex.register_user(&admin_ctx, "mytest".to_string());
        //let test_ok1 = typex.register_user(&admin_ctx, "mytest".to_string());
        
       // println!("{:?}", test_ok0);
        //println!("{:?}", test_ok1);
        
        //println!("{:?}", typex.get_proof(&admin_ctx, "mytest".to_string()));
       
        //println!("{:?}", ok);

        //let parameters0 = typex.get_param(&admin_ctx);
       // let parameters1 = typex.get_param(&voter_ctx);

        //println!("{:?}", parameters0);
        //println!("{:?}", parameters1);	
		//assert_eq!(12, db.len());
    }

    //#[bench]
    //fn benchmark_good_validation(b: &mut Bencher) {
    //    let data = (0..10000).collect::<Vec<_>>();
    //    let db = MerkleTree::from_vec(data);
    //    let root_hash = db.root_hash();
     //   let proof = db.get_proof(557);

     //   b.iter(|| { proof.validate(root_hash.unwrap()); })
    //}

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
