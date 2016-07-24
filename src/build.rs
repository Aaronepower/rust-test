#[cfg(feature = "serde_feature")]
extern crate serde_codegen;

use std::env;
use std::path::Path;

pub fn main() {
    expand();
}


#[cfg(feature = "serde_feature")]
fn expand() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src = Path::new("src/lib.rs.in");
    let dst = Path::new(&out_dir).join("lib.rs");

    serde_codegen::expand(&src, &dst).unwrap();
}
#[cfg(not(feature = "serde_feature"))]
fn expand() {}
