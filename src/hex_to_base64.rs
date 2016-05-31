const BASE_64: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
                             'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
                             'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                             'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3',
                             '4', '5', '6', '7', '8', '9', '+', '/'];

pub fn hex_to_base64(str: &str) -> String {
    let mut digits = str.chars().map(|c| c.to_digit(16).unwrap());
    let mut bytes = vec![];
    while let Some(next) = digits.next() {
        let mut byte: u32;
        byte = next << 20;
        byte = byte | (digits.next().unwrap_or(0) << 16);
        byte = byte | (digits.next().unwrap_or(0) << 12);
        byte = byte | (digits.next().unwrap_or(0) << 8);
        byte = byte | (digits.next().unwrap_or(0) << 4);
        byte = byte | digits.next().unwrap_or(0);
        bytes.push(byte);
    }
    let mut indices = vec![];
    for byte in bytes {
        indices.push((byte & (0b111111 << 18)) >> 18);
        indices.push((byte & (0b111111 << 12)) >> 12);
        indices.push((byte & (0b111111 << 6)) >> 6);
        indices.push(byte & 0b111111);
    }
    indices.iter().map(|i| BASE_64[*i as usize]).collect()
}

