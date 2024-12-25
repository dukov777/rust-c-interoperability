pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

extern "C" {
    fn c_function(x: i32) -> i32; // Declare the C function
}


// rust_lib/src/lib.rs
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    unsafe {
        c_function(x) // Call the C function
    }
}
