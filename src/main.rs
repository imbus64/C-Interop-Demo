#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::CString;

fn main() {
    let sum: i32 = unsafe { add(1, 2) };
    println!("Hello, world! C says that 1 + 2 = {}", sum);

    let animal: *mut Animal =
        unsafe { create_animal(10, CString::new("pelle").unwrap().into_raw()) };
    unsafe { free_animal(animal) };
}
