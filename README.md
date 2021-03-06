liballoc_hoard - Use Hoard as rust's allocator
==============================================

This library allows users to use [Hoard](http://www.hoard.org/)
instead of either [jemalloc](http://www.canonware.com/jemalloc/) or
libc's malloc when building rust binaries and shared libraries.

Additional crate information is available
[here](https://crates.io/crates/alloc_hoard/), and the idea behind
this library is the [Custom
Allocotors](https://doc.rust-lang.org/book/custom-allocators.html)
section of the Rust book.

Usage
-----

Simply add

    alloc_hoard = "0.4"

To the dependencies of your crate in a project's `Cargo.toml` file,
and then in a library or program add:

```rust
extern crate alloc_hoard;
```

To use Hoard for to satisfy all allocations for both the rust runtime
and your program/library.


Licensing
---------

The code in this repository is derived from liballoc_jemalloc and
liballoc_system from the rust repository, and as such falls under
either the MIT or Apache v2 licenses.  Hoard itself is licensed under
the GPLv2, so that also applies when linking against liballoc_hoard.
