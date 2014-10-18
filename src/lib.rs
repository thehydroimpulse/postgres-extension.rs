#![no_std]
#![feature(lang_items, intrinsics, macro_rules)]

extern crate libc;
extern crate core;

use core::cell::UnsafeCell;
use core::kinds::marker::InvariantLifetime;
use libc::{c_int, c_void, size_t};
use core::mem::{transmute, size_of};

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "fail_fmt"] extern fn fail_fmt() {}

pub type c_datum = uint;

pub struct Datum {
    val: uint
}

impl Datum {
    pub fn new_str(value: &str) -> Datum {
        // We need to allocate our string onto the heap
        // and with the custom `palloc` allocator. `palloc`
        // allocates memory into contexts such that they
        // can simply drop a while context without incurring
        // any memory leaks (i.e., some extension forgetting to
        // free their memory).
        // let mut mem = unsafe { pg_malloc(value.len() as size_t) };
        Datum {
            val: 0
        }
    }
}

pub struct DatumPtr<'a> {
    ptr: UnsafeCell<Datum>,
    marker: InvariantLifetime<'a>
}

#[repr(C)]
pub struct Pg_magic_struct {
    len: c_int,
    version: c_int,
    funcmaxargs: c_int,
    indexmaxkeys: c_int,
    nameddatalen: c_int,
    float4byval: c_int,
    float8byval: c_int
}

extern {
    static no_such_variable: c_int;

    pub fn pg_malloc(size: size_t) -> *mut c_void;
    pub fn pg_free(ptr: *mut c_void);
}

#[no_mangle]
pub extern fn Pg_magic_func() -> *const Pg_magic_struct {
    let data = Pg_magic_struct {
        len: size_of::<Pg_magic_struct>() as c_int,
        version: 90500 / 100,
        funcmaxargs: 100,
        indexmaxkeys: 32,
        nameddatalen: 64,
        float4byval: 1,
        float8byval: 1
    };

    &data
}

#[no_mangle]
pub extern fn is_zero(a: c_int) -> c_int {
    if a == 0 {
        return 1
    } else {
        return 0
    }
}
