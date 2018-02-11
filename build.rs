// Copyright 2016 Bobby Powers. All rights reserved.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


extern crate gcc;

fn main() {
    gcc::Build::new()
	.cpp(true)
	.flag("-std=c++11")
	.flag("-fno-builtin-malloc")
	.flag("-Bsymbolic")
        .define("NDEBUG", None)
	.define("_REENTRANT", Some("1"))
	.opt_level(3)
        .include("./Hoard/src")
        .include("./Hoard/src/include")
        .include("./Hoard/src/include/util")
        .include("./Hoard/src/include/hoard")
        .include("./Hoard/src/include/superblocks")
        .include("./Hoard/src/Heap-Layers")
        .file("Hoard/src/source/libhoard.cpp")
	.file("Hoard/src/source/unixtls.cpp")
	.file("Hoard/src/Heap-Layers/wrappers/gnuwrapper.cpp")
        .compile("libhoard.a");
}
