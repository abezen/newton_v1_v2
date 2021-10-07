
pub mod root;

// The main function testing curves v1 and v2


fn main() {
    let op: u128 = 300000;
    let ap: u128 = 400000;
    let of: u128 = 2150;
    
    let sol1 = root::curve_v1(op, ap, of);
    let sol2 = root::curve_v2(op, ap, of);
     println!("Solution v1 = {:?}", sol1);
     println!("Solution v2 = {:?}", sol2);
  
}
