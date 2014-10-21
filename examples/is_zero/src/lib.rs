#![feature(phase)]

extern crate postgres_extension;

#[phase(plugin)]
extern crate postgres_extension_macros;

extern crate libc;

use libc::{c_int};

pg_module!(version: 90500)

#[pg_export]
pub fn is_zero(a: c_int) -> c_int {
    if a == 0 {
        return 0
    } else {
        return 0
    }
}


