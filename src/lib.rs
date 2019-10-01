// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// Hoard itself is licensed under the GPLv2.

#![crate_name = "alloc_hoard"]
#![crate_type = "rlib"]

#![no_std]


mod hoard;

extern crate alloc;
extern crate libc;


#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe { hoard::allocate(size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe { hoard::deallocate(ptr, old_size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    old_size: usize,
                                    size: usize,
                                    align: usize)
                                    -> *mut u8 {
    unsafe { hoard::reallocate(ptr, old_size, size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(ptr: *mut u8,
                                            old_size: usize,
                                            size: usize,
                                            align: usize)
                                            -> usize {
    unsafe { hoard::reallocate_inplace(ptr, old_size, size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, align: usize) -> usize {
    hoard::usable_size(size, align)
}
