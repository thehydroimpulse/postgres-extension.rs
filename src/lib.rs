#![feature(macro_rules)]

extern crate libc;
extern crate core;

use core::cell::UnsafeCell;
use core::kinds::marker::InvariantLifetime;
use libc::{c_int, c_void, size_t};
use core::mem::{transmute, size_of};

extern {
    static no_such_variable: c_int;
    pub fn pg_malloc(size: size_t) -> *mut c_void;
    pub fn pg_free(ptr: *mut c_void);
}

/// A wrapper around a Postgres `Datum`. A datum is simply
/// a pointer-sized unsigned integer that acts like
/// a pointer.
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

/// The magic metadata that Postgres will ready by calling
/// the `Pg_magic_func` which returns a pointer to
/// this record.
#[repr(C)]
pub struct Pg_magic_struct {
    pub len: c_int,
    pub version: c_int,
    pub funcmaxargs: c_int,
    pub indexmaxkeys: c_int,
    pub nameddatalen: c_int,
    pub float4byval: c_int,
    pub float8byval: c_int
}

/// Postgres has a macro called `PG_MODULE_MAGIC` that is supposed
/// to be called within extensions. This generates a bunch
/// of metadata structures that Postgres reads to determine
/// the compatibility of the extension.
///
/// `Pg_magic_func` is the function Postgres will call
/// to check compatibility with memcmp, so there can't be
/// any alignment differences.
#[macro_export]
macro_rules! pg_module_magic {
    ($vers:expr) => {

        #[no_mangle]
        pub extern fn Pg_magic_func() -> *const postgres::Pg_magic_struct {
            use std::mem::size_of;
            use libc::{c_int};
            let data = postgres::Pg_magic_struct {
                len: size_of::<postgres::Pg_magic_struct>() as c_int,
                version: $vers / 100,
                funcmaxargs: 100,
                indexmaxkeys: 32,
                nameddatalen: 64,
                float4byval: 1,
                float8byval: 1
            };

            &data
        }
    }
}

