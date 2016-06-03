pub fn fixed_xor(a: &str, b: &str) -> String {
    let digits_a = a.chars().map(|c| c.to_digit(16).unwrap());
    let digits_b = b.chars().map(|c| c.to_digit(16).unwrap());
    digits_a
        .zip(digits_b).map(|(a, b)| a ^ b)
        .fold("".to_owned(), |acc, d| acc + &format!("{:x}", d))
}
