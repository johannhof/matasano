pub fn to_hex_bytes(input: &str) -> Vec<u8> {
    let mut digits = input.chars().map(|c| c.to_digit(16).expect(&format!("{:?} is not a hex digit", c)));
    let mut out = Vec::new();
    while let Some(next) = digits.next() {
        let mut byte = next as u8;
        byte = byte << 4;
        byte = byte | digits.next().unwrap_or(0) as u8;
        out.push(byte);
    }
    out
}

pub fn score(input: &str) -> f32 {
    input.to_lowercase().chars().fold(0.0, |acc, x| {
        acc + match x {
            ' ' => 14.0,
            '\n' => 2.0,
            '\'' => 1.0,

            'e' => 12.07,
            't' => 9.056,
            'a' => 8.167,
            'o' => 7.507,
            'i' => 6.966,
            'n' => 6.749,
            's' => 6.327,
            'h' => 6.094,
            'r' => 5.987,
            'd' => 4.253,
            'l' => 4.025,
            'c' => 2.782,
            'u' => 2.758,
            'm' => 2.406,
            'w' => 2.360,
            'f' => 2.228,
            'g' => 2.015,
            'y' => 1.974,
            'p' => 1.929,
            'b' => 1.492,
            'v' => 0.978,
            'k' => 0.772,
            'j' => 0.153,
            'x' => 0.150,
            'q' => 0.095,
            'z' => 0.074,
            _ => -24.0,
        }
    })
}

pub fn decode(input: &str) -> (f32, String) {
    let bytes = to_hex_bytes(input);
    let mut highscore = (0.0, String::new());
    for x in 0..255 {
        let xored = bytes.iter().map(|b| b ^ x).collect();
        let text = String::from_utf8(xored).unwrap_or_else(|_| "".to_owned());
        let score = score(&text);
        if score > highscore.0 {
            highscore = (score, text);
        }
    }

    highscore
}
