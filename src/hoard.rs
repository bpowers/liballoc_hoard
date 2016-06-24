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

extern crate libc;

use core::cmp;
use core::ptr;

use libc::{c_int, c_void, size_t};

// Linkage directives to pull in hoard and its dependencies.
//
// On some platforms we need to be sure to link in `pthread` which Hoard
// depends on, and specifically on android we need to also link to libgcc.
// Currently Hoard is compiled with gcc which will generate calls to
// intrinsics that are libgcc specific (e.g. those intrinsics aren't present in
// libcompiler-rt), so link that in to get that support.
#[cfg_attr(target_os = "android", link(name = "gcc"))]
#[cfg_attr(all(not(windows),
               not(target_os = "android"),
               not(target_env = "musl")),
           link(name = "pthread"))]
extern "C" {
    fn hoard_malloc(size: size_t) -> *mut c_void;
    fn hoard_posix_memalign(ptr: *mut *mut c_void, size: size_t, align: size_t) -> c_int;
    fn hoard_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn hoard_free(ptr: *mut c_void);
}


// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values. In practice, the alignment is a
// constant at the call site and the branch will be optimized out.
#[cfg(all(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "mipsel",
              target_arch = "powerpc",
              target_arch = "powerpc64",
              target_arch = "powerpc64le")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86_64",
              target_arch = "aarch64")))]
const MIN_ALIGN: usize = 16;

pub unsafe fn allocate(size: usize, align: usize) -> *mut u8 {
    if align <= MIN_ALIGN {
        hoard_malloc(size as libc::size_t) as *mut u8
    } else {
        let mut out = ptr::null_mut();
        let ret = hoard_posix_memalign(&mut out, align as libc::size_t, size as libc::size_t);
        if ret != 0 {
            ptr::null_mut()
        } else {
            out as *mut u8
        }
    }
}

pub unsafe fn reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
    if align <= MIN_ALIGN {
        hoard_realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
    } else {
        let new_ptr = allocate(size, align);
        ptr::copy(ptr, new_ptr, cmp::min(size, old_size));
        deallocate(ptr, old_size, align);
        new_ptr
    }
}

pub unsafe fn reallocate_inplace(_ptr: *mut u8,
                                 old_size: usize,
                                 _size: usize,
                                 _align: usize)
                                 -> usize {
    old_size
}

pub unsafe fn deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    hoard_free(ptr as *mut libc::c_void)
}

pub fn usable_size(size: usize, _align: usize) -> usize {
    size
}
