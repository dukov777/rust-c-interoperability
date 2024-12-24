// build.rs

fn main() {
    // Tell Cargo to link to the C static library
    println!("cargo:rustc-link-lib=static=other_lib"); // Name of the C static library (without lib prefix or extension)
    
    // Tell Cargo where to find the C library
    println!("cargo:rustc-link-search=native={}", "../build/"); // Path to the directory containing libother_lib.a
}
