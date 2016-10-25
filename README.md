# Rust FFI hack

This is a demonstration of a suspected bug with Rust's C FFI.

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