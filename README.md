liballoc_hoard - Use Hoard as rust's allocator
==============================================

This library allows users to use Hoard instead of either jemalloc or
libc's malloc when building rust binaries and shared libraries.

Licensing
---------

The code in this repository is derived from liballoc_jemalloc and
liballoc_system from the rust repository, and as such falls under
either the MIT or Apache v2 licenses.  Hoard itself is licensed under
the GPLv2, so that also applies when linking against liballoc_hoard.
