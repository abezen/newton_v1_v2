
pub mod root;


// The main function testing curves v1 and v2


fn main() {
    let op: u128 = 7011000;
    let ap: u128 = 5011000;
    let of: u128 = 5000;
    let aa: u128 = 5000;
    
    let sol1 = root::curve_v1(op, ap, of);
    let sol2 = root::curve_v2(op, ap, of);
     println!("Solution v1 = {:?}", sol1);
     println!("Solution v2 = {:?}", sol2);

     let sol3 = root::compute_offer_amount_curve_v1(ap, op, aa);
     println!("Offer amount v1 = {:?}", sol3); 

     let sol4 = root::compute_offer_amount_curve_v2(ap, op, aa);
     println!("Offer amount v2 = {:?}", sol4); 
  
}
