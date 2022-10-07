use std::io::prelude::*;

use crate::constants::METHODS;

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

macro_rules! crack {
    ( $a:expr, $b:expr ) => {
        match $a {
            Some(x) => x,
            None => return Err($b),
        }
    };
}

pub fn is_http_method(m: &str) -> bool {
    METHODS.iter().find(|&&x| m == x).is_some()
}

pub fn get_reason(status: i32) -> &'static str {
    match status {
        200 => "OK",
        201 => "CREATED",
        400 => "BAD REQUEST",
        403 => "UNAUTHORIZED",
        404 => "NOT FOUND",
        500 => "INTERNAL SERVER ERROR",
        _ => "UNKNOWN",
    }
}

use concat_idents::concat_idents;

macro_rules! setter {
    ($name:ident, $type:ty) => {
        concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, val: $type) {
                self.$name = val;
            }
        });
    }
}

pub(crate) use collection;
pub(crate) use crack;
pub(crate) use setter;
