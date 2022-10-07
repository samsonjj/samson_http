use crate::util::collection;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref METHODS: Vec<&'static str> = collection! {
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
