extern crate num;

use round::round_down;

use num::{Float, FromPrimitive, Signed, abs};
/* ----------------------------------------------------------- */
/// A trait for things that can be approximately equal.
pub trait Epsilon {
    type RHS;
    type Precision;

    /// Return true if self and `other` differ no more than by a given amount.
    fn close(&self, other: Self::RHS, precision: Self::Precision) -> bool;

    /// Return true if self is close to zero.
    fn near_zero(&self, precision: Self::Precision) -> bool;
}

impl<T: Float + Signed> Epsilon for T {
    type RHS = T;
    type Precision = T;

    fn close(&self, other: T, precision: T) -> bool {
        abs(other - *self) < abs(precision)
    }

    fn near_zero(&self, precision: T) -> bool {
        abs(*self) < abs(precision)
    }
}
//pub mod epsilon;
//use epsilon::Epsilon;

/* ---------- Newton's method (single root) ---------- */

/// Configuration structure for the Newton's method (one root version).
#[derive(Debug, Clone, Copy)]
pub struct OneRootNewtonCfg<T> {
    /// The real root, if any, is most likely to be within this distance from
    /// the reported root, but this is not guaranteed.
    pub precision: T,
    /// A limit on the number of iterations to perform. Pass `None` if you
    /// don't want a limit.
    pub max_iters: Option<u32>
}

pub fn newton_one<T, F, D>(config: OneRootNewtonCfg<T>,
                           left: T,
                           right: T,
                           first_approx: T,
                           target: &F,
                           derivative: &D)
    -> Option<T>
    where T: Float + Epsilon<RHS=T, Precision=T>,
          F: Fn(T) -> T,
          D: Fn(T) -> T
{
    let mut left = left;
    let mut right = right;
    let mut left_val = target(left);
    let mut right_val = target(right);
    let mut root = first_approx;
    let mut prev_root = None;
    let mut iter = 0;
    while prev_root.map_or(true, |old| !root.close(old, config.precision))
        && config.max_iters.map_or(true, |max| iter < max) {
            iter += 1;
            if let Some(next) = next_newton_iter(config.precision,
                                                 left, 
                                                 right, 
                                                 root, 
                                                 target, 
                                                 derivative) {
                prev_root = Some(root);
                root = next;
            } else if let Some(fallback_root) 
                = linear_fallback(left, right, left_val, right_val) {
                    prev_root = Some(root);
                    root = fallback_root;
            } else {
                return None
            }
            let val_at_root = target(root);
            if left_val * val_at_root <= T::zero() {
                right = root;
                right_val = val_at_root;
            } else {
                left = root;
                left_val = val_at_root;
            }
    }
    Some(root)
}

fn next_newton_iter<T, F, D>(prec: T,
                             left: T,
                             right: T,
                             old: T,
                             target: &F,
                             derivative: &D)
    -> Option<T>
    where T: Float + Epsilon<RHS=T, Precision=T>,
          F: Fn(T) -> T,
          D: Fn(T) -> T
{
    let d = derivative(old);
    if d.near_zero(prec) {
        return None
    }
    let res = old - target(old) / d;
    if res < left {
        None
    } else if res > right {
        None
    } else {
        Some(res)
    }
}

fn linear_fallback<T: Float>(x1: T , x2: T, y1: T, y2: T) -> Option<T>
{
    let res = ((y2 - y1) * x1 - (x2 - x1) * y1) / (y2 - y1);
    if res < x1 {
        None
    } else if res > x2 {
        None
    } else {
        Some(res)
    }
}

/* --------  Curve v1 ---------
offer_pool -> offer_pool_amount
ask_pool -> ask_pool_amount
offer -> offer_asset_amount
*/


pub fn curve_v1(_offer_pool: u128, _ask_pool: u128, _offer: u128)  -> u128
{
    let prec = 1e-2;
    let _a: f64 = 85.0;
    let _d: f64 = 5.0;
    let op = _offer_pool as f64;
    let ap = _ask_pool as f64;
    let of = _offer as f64;
    let sum: f64 = op + ap;
    let prod: f64 =  op * ap;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let _target_d = |x: f64| x * (4.0 * _a - 1.0) +  x* x* x / (4.0 * prod) - 4.0 * _a * sum;
    let _der_d =  |x: f64| 4.0 * _a * - 1.0 + 3.0 * x * x / (4.0 * prod);

    let prec = 1.0;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let sol = newton_one(_cfg, 0.0, 2000000.0, 60.0, &_target_d, &_der_d);

    let z: f64;

    match sol {
        Some(ss) => z = ss,
        None => panic!(),
    };

   let rslt =  Some(sol).unwrap();

  println!("z = {:?}", z);

   println!("Result {:?}", rslt);

   let d1 = z.floor() as u128;
   println!("Result t = {:?}", d1);
   /*
   match sol {
       Some(rslt) => rslt,
       None,
   };
   */

   println!("A = {:?}", _a);

   let x1 = op + of;

  let _target_y = |x: f64| 4.0 * _a * z + z * z * z / (4.0 * x1 * x) - 
        4.0 * _a * ( x1 + x) - z;
  let _der_y = |x: f64| (- z) * z * z / (4.0 * x1 * x *x) - 4.0 * _a;

  let sol_y = newton_one(_cfg, 0.0, 100000.0, 1000.0, &_target_y, &_der_y);

  let y: u128;

  match sol_y {
      Some(t) => y = t.floor() as u128,
        None => panic!(),
  }

  println!("offer = {:?}, new D = {:?}, x+ dx = {:?}, Ask pool = {:?}", y, z, x1, _ask_pool);
    return (_ask_pool - y);
}


pub fn curve_v2(_offer_pool: u128, _ask_pool: u128, _offer: u128)  -> u128
{
    let prec = 1e-2;
    let _a: f64 = 85.0;
    let _d: f64 = 5.0;
    let _gam = 1e-4;
    let beta = _gam  + 1.0;
    let op = _offer_pool as f64;
    let ap = _ask_pool as f64;
    let of = _offer as f64;
    let sum: f64 = op + ap;
    let prod: f64 =  op * ap;
    let apg: f64 = 4.0 * _a * prod * _gam * _gam;
    let d0: f64 = 2.0 * prod.sqrt();
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let _target_d = |x: f64| apg * (sum / x - 1.0) / ((beta - 4.0 * prod/(x * x)) *(beta - 4.0 * prod/(x * x))) +
        prod - x * x / 4.0 ;
    let _der_d =  |x: f64| apg/((beta - 4.0 * prod/(x * x)) *(beta - 4.0 * prod/(x * x)))  *
        16.0 * prod * x * x * x * x * x * (sum / x - 1.0) / (beta * x * x - 4.0 * prod) - sum / (x * x);

    let prec = 1.0;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let sol = newton_one(_cfg, 0.0, 2000000.0, d0, &_target_d, &_der_d);

    let z: f64;


   let rslt =  Some(sol).unwrap();
   match sol {
    Some(ss) => z = ss,
    None => panic!(),
};

let rslt =  Some(sol).unwrap();

println!("z = {:?}", z);

println!("Result {:?}", rslt);

let x1 = op + of;

let x0: f64 = z / (2.0 * x1);

let d1 = z.floor() as u128;
println!("Result t = {:?}", d1);

/* --------------   find  offer ----------------- */
let x1 = op + of;
let s1 = 4.0 * _a * x1 * _gam * _gam;
let s2 = z * x1 - 1.0;
let s3 = 4.0 * x1 / (z * z);
let s4 = s2 + z * x1;
let s5 = 8.0 * x1 * ( s2 + z );
let s6 = s1 * s4;
let s7 = s1 * s5;
let s8 = z * z / 4.0;
let s9 = s1 * s2;
let s10 = s1 * z;
let x_start = z / ( 2.0 * x1);

let _target_y = |x: f64| (s9 * x + s10 * x * x) / ((beta - s3 *x) *(beta - s3 *x))+ x1 * x - s8;
let _der_y = |x: f64| (s6 + s7)/((beta - s3 *x) *(beta - s3 *x) * (beta - s3 *x) ) + x1;

let sol_y = newton_one(_cfg, 0.0, 100000.0, x_start, &_target_y, &_der_y);

let y: u128;

match sol_y {
    Some(t) => y = t.floor() as u128,
      None => panic!(),
}

println!("offer = {:?}, new D = {:?}, x+ dx = {:?}, Ask pool = {:?}", y, z, x1, _ask_pool);
  return (_ask_pool - y);


}

