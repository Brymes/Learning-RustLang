fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m!= 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    greatest_common_divisor(10, 2);
}

#[test]
fn test_greatest_common_divisor() {
    assert_eq!(greatest_common_divisor(200, 2), 1);
    assert_eq!(greatest_common_divisor(2 * 3 * 4, 7 * 11 * 13), 3 * 11);
}