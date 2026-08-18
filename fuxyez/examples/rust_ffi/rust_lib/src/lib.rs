#[no_mangle]
pub extern "C" fn calculate_fibonacci(n: i32) -> u64 {
    if n <= 1 {
        n as u64
    } else {
        let mut a = 0u64;
        let mut b = 1u64;
        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}
