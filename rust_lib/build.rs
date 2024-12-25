// build.rs

fn main() {
    // Tell Cargo to link to the C static library
    println!("cargo:rustc-link-lib=static=static_c_lib"); // Name of the C static library (without lib prefix or extension)
    
    // Tell Cargo where to find the C library
    println!("cargo:rustc-link-search=native={}", "../c_lib/build"); // Path to the directory containing libother_lib.a
}
