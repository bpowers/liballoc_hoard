
extern crate gcc;

fn main() {
    gcc::Config::new()
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
