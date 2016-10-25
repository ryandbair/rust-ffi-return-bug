extern crate libc;
extern crate opencv;

use libc::c_void;

#[repr(C)]
#[derive(Debug)]
struct Bytes {
    pub data: [u8; 3],
}

#[link(name="test")]
extern "C" {
    fn call_me2(ptr: *const c_void, param1: u8, param2: u8, param3: u8) -> Bytes;
    fn call_me3(ptr: *const c_void, param1: u8, param2: u8, param3: u8) -> Bytes;
}

fn main() {
    let something = "test";
    let bytes = unsafe { call_me2(something as *const _ as *const c_void, 42, 142, 242) };
    println!("result is: {:?}", bytes);

    let z = unsafe { call_me3(something as *const _ as *const c_void, 42, 142, 242) };
    println!("result z is: {:?}", z);

    let x = opencv::mat();
    let out = x.at_vec3b_2(42, 142);
    println!("out is {:?}", out);
}