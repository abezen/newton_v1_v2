
pub mod root;
// use num::{Float, FromPrimitive, Signed, abs};
// use std::env;



fn main() {
    let op: u128 = 200000;
    let ap: u128 = 300000;
    let of: u128 = 1050;
    
    let sol1 = root::curve_v1(op, ap, of);
    let sol2 = root::curve_v2(op, ap, of);
     println!("Solution v1 = {:?}", sol1);
     println!("Solution v2 = {:?}", sol2);
  
}
