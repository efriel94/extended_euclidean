use euclid_divmod::*;

fn main() {
    let a = 2147483647;
    let b = -1234567891;

    println!("gcd({}, {}): {}", a, b, gcd(a, b));

    let (result,s,t) = extended_gcd(a, b); 
    
    let a: i64 = a.try_into().expect("Cannot convert");
    let b: i64 = b.try_into().expect("Cannot convert");
    let s: i64 = s.try_into().expect("Cannot convert");
    let t: i64 = t.try_into().expect("Cannot convert");
    let result: i64 = result.try_into().expect("Cannot convert");

    let bezouts_check= s*a + t*b;
    println!("bezouts check = {}", bezouts_check);

    assert!(s*a + t*b == result);
    println!("extended_gcd({}, {}) = {}, s = {}, t = {} ", a, b, result, s, t);
}

/// Euclidean algorithm computes the gcd(a,b)
/// where a = bq + r therefore r = a - bq 
/// q is quotient
/// r is remainder
fn gcd(mut a: i64, mut b: i64) -> i64 {

    while b != 0 {

        let q = euclid_divmod::div(a, b);
        //let q = a / b;
        let r = a - (b * q);
        
        //next iteration
        a = b;
        b = r;
    }
    a
}

/// Extended euclidean algorithm which computes gcd(a,b) as well as the coefficents of bezouts identity
/// of the form: a*s + b*t = gcd(a,b)
/// Returns -> (gcd(a,b): i32, s: i32, t:i32)
fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {

    let mut s1 = 1;
    let mut s2 = 0;
    let mut t1 = 0;
    let mut t2 = 1;

    while b != 0 {
        let q = euclid_divmod::div(a, b);
        let r = a - q * b;

        let s3 = s1 - q * s2;
        let t3 = t1 - q * t2;

        // Update for next iteration
        a = b;
        b = r;
        s1 = s2;
        s2 = s3;
        t1 = t2;
        t2 = t3;
    }

    return (a, s1, t1);
}


// create function for calculating the multiplicative inverse using extended euclidean algorithm

// create function for calculating the polynominal gcd of two univariate polynominals
