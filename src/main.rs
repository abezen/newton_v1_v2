
pub mod root;
use num::{Float, FromPrimitive, Signed, abs};
// use std::env;



fn main() {
    let op: u128 = 10000;
    let ap: u128 = 20000;
    let of: u128 = 200;
    
    let sol = root::curve_v2(op, ap, of);
     println!("Solution {:?}", sol);
  
}
