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

// rust_lib/src/lib.rs
#[no_mangle]
pub extern "C" fn my_rust_function(x: i32) -> i32 {
    x + 2
}
