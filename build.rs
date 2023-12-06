extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Compile the C source code
    cc::Build::new().file("c-src/add.c").compile("add");

    // Tell bindgen to generate the bindings.
    // You can look at these bindings after compilation
    let bindings = bindgen::Builder::default()
        .header("c-src/add.h")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // $OUT_DIR is set by cargo, and is usually target/debug or target/release
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
