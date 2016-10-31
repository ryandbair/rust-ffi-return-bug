# Rust FFI hack

This is a demonstration of a suspected bug with Rust's C FFI.


### Update
The cause has been identified as a mismatch between C and C++ calling conventions.
I would have thought that an extern C block would result in C compatible code, but
taking C++ classes as arguments or returning them as results, can result in 
generated functions which are not C ABI compatible. In this particular case,
the `cv::Vec3b` return was expected to be returned via a pointer specified
in the first argument to the function. Rust's C FFI was not aware of this
which caused the issue described below.

From Rust-users: https://users.rust-lang.org/t/incorrect-ffi-argument-positions-with-specific-return-type/7757/2

### Status
This presents a case, `call_me3`, where calling a function with the `cv::Vec3b` return type
results in the first argument seemingly being used in the C function as the return
memory and the other arguments being shifted one position left. In this example,
the code segfaults.
 
There is another case, `call_me2` which attempts to replicate the error without
using OpenCV types, but currently fails at that goal. 

My knowledge of Rust's FFI, calling conventions, and asm are all fairly limited.
I've spent some time attempting to interpret what makes Vec3b special, but so far have
not figured it out. 

### Tested Platform

+ Fedora 24
+ x86_64
+ rustc 1.14.0-nightly (f09420685 2016-10-20)
+ OpenCV from Fedora repos version 2.4.12.3-3