
pub mod root;


// The main function testing curves v1 and v2
// op - offer pool
// ap - ask pool

//  let op: u128 = 20000000000;
//  let ap: u128 = 20000000000;
//  let of: u128 = 1500000000;

// A = 10, return amount = 1494634750
// A = 20, return amount = 1497246422
// A = 30, return amount = 1498147950
// A = 40, return amount = 1498604755
// A = 50, return amount = 1498880804
// A = 60, return amount = 1499065662
// A = 70, return amount = 1499198110
// A = 80, return amount = 1499297670
// A = 85, return amount = 1499338721
// A = 90, return amount = 1499375238
// A = 100, return amount = 1499437377
// A = 110, return amount = 1499488273
// A = 120, return amount = 1499530725
// A = 130, return amount = 1499566672
// A = 140, return amount = 1499597505
// A = 150, return amount = 1499624241
// A = 160, return amount = 1499647646
// A = 170, return amount = 1499668307
// A = 180, return amount = 1499686679
// A = 190, return amount = 1499703123
// A = 200, return amount = 1499717926


//   let op: u128 = 30000000000;
//   let ap: u128 = 20000000000;
//   let of: u128 = 1500000000;

// A = 85, return amount = 1495753098

fn main() {
    let op: u128 = 20000000000;
    let ap: u128 = 20000000000;
    let of: u128 = 1500000000;
  
    
    let sol1 = root::curve_v1(op, ap, of);
 
     println!("Solution v1 = {:?}", sol1);
  
 // let ra = 1494634750;
 // let ra = 1494621299;
 let ra = 1496174605;

     let sol3 = root::compute_offer_amount_curve_v1(ap, op, ra);
     println!("Offer amount v1 = {:?}", sol3); 

     let fra: f64 = ra as f64;
     let commissn: f64 = fra * 0.003;
     println!("commission amount v1 = {:?}", commissn ); 

  
}
