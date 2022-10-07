use std::collections::HashSet;
use std::time::Instant;

#[macro_use]
extern crate lazy_static;

macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}

lazy_static! {
    static ref METHODS_HS: HashSet<&'static str> = collection! {
        "GET",
        "HEAD",
        "POST",
        "PUT",
        "DELETE",
        "CONNECT",
        "OPTIONS",
        "TRACE",
        "PATCH",
    };
    static ref METHODS_VEC: Vec<&'static str> = collection! {
        "GET",
        "HEAD",
        "POST",
        "PUT",
        "DELETE",
        "CONNECT",
        "OPTIONS",
        "TRACE",
        "PATCH",
    };
}

fn main() {
    let t0 = Instant::now();
    let mut count = 0;
    let key = "GET";
    for _ in 0..1_000_000 {
        if let Some(_) = METHODS_VEC.iter().find(|&&s| s == key) {
            count += 1;
        }
    }
    let elapsed = t0.elapsed().as_secs_f64();
    println!("vector  : {} - The sum is: {}", elapsed, count);

    for _ in 0..1_000_000 {
        if let Some(_) = METHODS_HS.get(key) {
            count += 1;
        }
    }

    let elapsed = t0.elapsed().as_secs_f64();
    println!("hash set: {} - The sum is: {}", elapsed, count);
}
