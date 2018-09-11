use single_byte_xor_cipher::{score, to_hex_bytes};
use std::f32;
use std::cmp;

pub fn encode(input: &str, key: &str) -> String {
    input.bytes()
        .zip(key.bytes().cycle())
        .map(|(a, b)| a ^ b)
        .fold("".to_owned(), |acc, d| acc + &format!("{:02x}", d))
}

pub fn hamming_distance<T: ?Sized + AsRef<[u8]>>(a: &T, b: &T) -> u8 {
    a.as_ref().iter()
        .zip(b.as_ref().iter())
        .map(|(a, b)| a ^ b)
        .fold(0, |mut acc, d| {
            for i in 0..8 {
                acc += (d >> i) & 0b1
            }
            acc
        })
}

pub fn decode<T: ?Sized + AsRef<[u8]>>(bytes: &T) -> (String, String) {
    let bytes = bytes.as_ref();
    // First, guess the key length.
    let mut distances = vec![];
    let mut max = 42;
    if bytes.len() / 4 < max {
        max = bytes.len() / 4;
    }
    for i in 1..max {
        let distance1 = hamming_distance(&bytes[0..i], &bytes[i..i * 2]) as f32 / i as f32;
        let distance2 = hamming_distance(&bytes[i * 2..i * 3], &bytes[i * 3..i * 4]) as f32 / i as f32;
        let avg = (distance1 + distance2) / 2.0;
        println!("{:?}, {:?}, {:?}, {:?}", i, avg, distance1, distance2);
        distances.push((avg, i));
    }

    distances.sort_by(|a, b|  a.0.partial_cmp(&b.0).unwrap_or(cmp::Ordering::Equal));

    let mut proposals = vec![];
    for distance in distances.iter().take(10) {
        let mut blocks = vec![];

        let &(_, keysize) = distance;
        for i in 0..keysize {
            let mut it = bytes.iter();
            let mut block = vec![];
            block.push(it.nth(i).unwrap());
            while let Some(byte) = it.nth(keysize - 1) {
                block.push(byte);
            }
            blocks.push(block);
        }

        let mut key = String::new();
        let mut solution = String::new();

        for block in blocks {
            let mut highscore = (f32::NEG_INFINITY, 0, String::new());
            for x in 0..255 {
                let text = block.iter().map(|b| *b ^ x).fold(String::new(), |mut acc, b| {
                    acc.push(b as char);
                    acc
                });
                let score = score(&text);
                if score > highscore.0 {
                    highscore = (score, x as u8, text);
                }
            }
            key.push(highscore.1 as char);
        }

        let solution = bytes.iter()
            .zip(key.chars().cycle())
            .map(|(a, b)| a ^ b as u8)
            .fold(String::new(), |mut acc, b| {
                    acc.push(b as char);
                    acc
                });

        let score = score(&solution);
        proposals.push((score, solution, key));
    }

    proposals.sort_by(|a, b|  a.0.partial_cmp(&b.0).unwrap_or(cmp::Ordering::Equal));

    println!("{:?}", proposals);

    let winning = proposals.last().unwrap();
    (winning.1.clone(), winning.2.clone())
}

