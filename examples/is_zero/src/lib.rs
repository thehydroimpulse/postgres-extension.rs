#![feature(phase)]

#[phase(plugin, link)]
extern crate postgres_extension;

#[phase(plugin)]
extern crate postgres_macros;

extern crate libc;

use libc::{c_int};

pg_module!(90500)

#[no_mangle]
pub extern fn is_zero(a: c_int) -> c_int {
    if a == 0 {
        return 1
    } else {
        return 0
    }
}
