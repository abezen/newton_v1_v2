use ta::indicators::ExponentialMovingAverage;
use ta::Next;

extern crate num;
const GAM: f64 = 1e-6;
const A: f64 = 85.0;
// const COMMISION_RATE: f64 = 0.05; 
const COMMISION_RATE: f64 = 0.003; 
const COEFFS:[f64;5] = [1.0, 0.5, 0.013888888888888888, 0.000992063492063492, 0.00003306878306878307];
const LOG_E_2: f64 = 0.69314718055995;

 use num::{Float,  Signed, abs};
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

/* ---------------------------------
    IMPLEMENTATION OF THE CURVE V1 ALGORITHM ACCORDING TO 
    THE PAPER BY MICHAEL EGOROV ON 10 NOVEMBER 2019
*/

pub fn compute_d(op: f64, ap: f64, d0: f64) -> f64
{
    let sum: f64 = op + ap;
    let prod: f64 =  op * ap;
    let a4: f64 = 4.0 * A;
    let prod4: f64 = 4.0 * prod;
    let a4_1: f64 = a4 - 1.0;
    let a4_sum: f64 = a4 * sum;
    let prod4_3: f64 = 3.0 / prod4;
    let prec = 1.0e-1;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let _target_d = |x: f64| x * a4_1 +  x* x* x / prod4 - a4_sum;
    let _der_d =  |x: f64| a4  - 1.0 + prod4_3 * x * x ;

    let sol = newton_one(_cfg, 0.0, 10e25, d0, &_target_d, &_der_d);

    let d: f64;

    match sol {
        Some(ss) => d = ss,
        None => panic!(),
    };

    return d;
}

pub fn get_ask_amount(op: f64, of: f64, d: f64) -> f64
{
    let a4 = 4.0 * A;
    let x1: f64 = op + of;
    let _target_y = |x: f64| a4 * d + d * d * d / (4.0 * x1 * x) -  a4 * ( x1 + x) - d;
    let _der_y = |x: f64| (- d) * d * d / (4.0 * x1 * x *x) - a4;

    let prec = 1.0;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let sol = newton_one(_cfg, 0.0, 10e25, of, &_target_y, &_der_y);

    let ask_pool: f64;

    match sol {
        Some(ss) => ask_pool = ss,
        None => panic!(),
    };

    return ask_pool;
}

pub fn curve_v1(_offer_pool: u128, _ask_pool: u128, _offer: u128)  -> u128
{
    let op = _offer_pool as f64;
    let ap = _ask_pool as f64;
    let of = _offer as f64;
    let d0 = 1000.0;
    let d = compute_d(op, ap, d0);

    println!("d version 2 = {0}", d);

    let ask_f = get_ask_amount(op, of, d);
    let ask_amnt: u128 = ask_f as u128;
    return _ask_pool - ask_amnt;
}




/* ---------------------------------
    IMPLEMENTATION OF THE CURVE V2 ALGORITHM ACCORDING TO 
    THE PAPER BY MICHAEL EGOROV ON 9 JUNE 2021
    CURRENTLY ONLY CurveCrypto INVARIANT IS IMPLEMENTED.
    THE REPEGGING ALGORITHM IS NOT IMPLEMENTED AS IT REQUIRES
    A KNOWLEDGE OF HISTORICAL DATA
*/

pub fn curve_v2(_offer_pool: u128, _ask_pool: u128, _offer: u128)  -> u128
{
    /* Before the Newton's method to be called, 
    numerical coefficients of the polinomials are calculated
    to avoid same calculations in iterations
    */
    let prec = 1e-2;
    let _d: f64 = 5.0;
    let beta = GAM  + 1.0;
    let op = _offer_pool as f64;
    let ap = _ask_pool as f64;
    let of = _offer as f64;
    let sum: f64 = op + ap;
    let prod: f64 =  op * ap;
    let s1: f64 = 4.0 * A * prod * GAM * GAM;
    let d0: f64 = op + ap;
    let prod4: f64 = 4.0 * prod;
    let beta_3: f64 = (-3.0) * beta;
    let beta_4_sum: f64 = 4.0 * sum * beta;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let _target_d = |x: f64| s1*(sum-x)/((beta*x*x-prod4)*(beta*x*x-prod4)) +prod-x*x/4.0;
    let _der_d =  |x: f64| (-s1)*(beta_3*x*x+beta_4_sum*x-prod4)/ ((beta*x*x-prod4)*(beta*x*x-prod4)*(beta*x*x-prod4)) - x/2.0;

    let prec = 1.0;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let sol = newton_one(_cfg, 0.0, 10e9, d0, &_target_d, &_der_d);

    let z: f64;

   match sol {
    Some(ss) => z = ss,
    None => panic!(),
};

let rslt =  Some(sol).unwrap();

println!("D_v2 = {:?}", z);

println!("Result {:?}", rslt);

let d1 = z.floor() as u128;
println!("Result v2 = {:?}", d1);

/* --------------   find  offer ----------------- */

let x0 = op + of;
let x_start = z / ( 2.0 * x0);
let b1: f64 = 4.0 * x0 /(z*z);
let b2: f64 = A * b1 * GAM * GAM;
let b3: f64 = x0 - z;
let b4: f64 = z * z / 4.0;
let b5: f64 = b2 * z;

let _target_y = |x: f64| b5 * x * (b3 + x)/((beta - b1 * x)*(beta - b1 * x)) + x0 * x - b4;
let _der_y = |x: f64| b5 * (b1 * b3 * x + 2.0 * beta * x + b3 * beta) /((beta - b1 *x)*(beta - b1 *x)*(beta - b1 *x))+ x0;

let sol_y = newton_one(_cfg, 0.0, 10e9, x_start, &_target_y, &_der_y);

let y: u128;

match sol_y {
    Some(t) => y = t.floor() as u128,
      None => panic!(),
}

println!("offer = {:?}, new D = {:?}, x+ dx = {:?}, Ask pool = {:?}", y, z, x0, _ask_pool);
  return _ask_pool - y;
}


pub fn get_offer_amount(ap: f64, ask_amnt: f64, d: f64) -> f64
{
    let a4 = 4.0 * A;
    let x1: f64 = ap - ask_amnt;
    let _target_y = |x: f64| a4 * d + d * d * d / (4.0 * x1 * x) -  a4 * ( x1 + x) - d;
    let _der_y = |x: f64| (- d) * d * d / (4.0 * x1 * x *x) - a4;

    let prec = 1.0;
    let _cfg = OneRootNewtonCfg {
        precision: prec,
        max_iters: None
    };

    let sol = newton_one(_cfg, 0.0, 10e25, ask_amnt, &_target_y, &_der_y);

    let offer_pool: f64;

    match sol {
        Some(ss) => offer_pool = ss,
        None => panic!(),
    };

    return offer_pool;
}

// These 2 functions replace the code in lines 598- 601
pub fn compute_offer_amount_curve_v1(ask_pool: u128, offer_pool: u128, ask_amount: u128)  -> u128
{   
    let op = offer_pool as f64;
    let ap = ask_pool as f64;
    let am = ask_amount as f64;
    let d0 = 1000.0;

    let d = compute_d(op, ap, d0);
    let offer_f = get_offer_amount(ap, am, d);
    let offer_amnt: u128 = offer_f as u128;

    println!("A = {0}", A);
    return offer_amnt - offer_pool;
}

pub fn compute_offer_amount_curve_v2(ask_pool: u128, offer_pool: u128, ask_amount: u128)  -> u128
{
    let ask_amnt = ask_amount as f64;
    let ask_amnt_with_rate: u128 = (ask_amnt * 1.0 / (1.0 - COMMISION_RATE)).round() as u128;
    let offer_amount: u128 = curve_v2(offer_pool, ask_pool, ask_amnt_with_rate);

    return offer_amount;
}

pub fn calculate_alpha(x: f64) -> f64
{
    let mut sum_plus = 1.0;
    let mut sum_minus = 0.0;
    let lngth = COEFFS.len();

    let mut arr_x: [f64; COEFFS.len()] = [1.0; COEFFS.len()];

    for i in 1..lngth {
        arr_x[i] = x * LOG_E_2 * arr_x[i-1];
    }

    for i in 1..COEFFS.len()-1 {
        if i % 2 == 0 {
            sum_plus += arr_x[i] * COEFFS[i];
        }
        else {
            sum_minus += arr_x[i] * COEFFS[i];
        }
    }

    let numer = sum_plus + sum_minus;
    let denom = sum_plus - sum_minus;

    return numer / denom;
}


