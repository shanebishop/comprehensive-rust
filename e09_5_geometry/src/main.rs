// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
fn magnitude(v: &[f64]) -> f64 {
    let mut sum = 0.0;
    for x in v {
        sum += x * x;
    }
    sum.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(v: &mut [f64]) {
    let mag = magnitude(v);
    for x in v {
        *x /= mag;
    }
}

// Use the following `main` to test your work.
fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}

#[test]
fn test_magnitude() {
    assert_eq!(magnitude(&[0.0, 1.0, 0.0]), 1.0);
}

#[test]
fn test_noramlize() {
    let mut v = [1.0, 2.0, 9.0];
    normalize(&mut v);
    assert_eq!(magnitude(&v), 1.0);
}
