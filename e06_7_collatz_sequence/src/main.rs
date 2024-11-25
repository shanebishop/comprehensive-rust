/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: u32) -> u32 {
    // If n_i is 1, then the sequence terminates at n_i.
    // If the input is 1, then the sequence terminates
    // right away, and the length is 1.
    if n == 1 {
        return 1;
    }

    let mut len = 1; // n_1 is the first value
    while n != 1 {
        // Compute next n
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        // Add one to the count
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
    assert_eq!(collatz_length(1), 1);
    assert_eq!(collatz_length(3), 8);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}
