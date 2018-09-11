extern crate openssl;
extern crate base64;
extern crate hex;

mod hex_to_base64;
mod fixed_xor;
mod single_byte_xor_cipher;
mod repeating_key_xor;

#[cfg(test)]
mod challenges {

    mod set_1 {
        use base64;
        use openssl;
        use hex;

        use hex_to_base64::hex_to_base64;
        use fixed_xor::fixed_xor;
        use single_byte_xor_cipher;
        use repeating_key_xor::{self, hamming_distance};

        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;
        use std::collections::HashSet;

        #[test]
        fn challenge_1() {
            assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
                       "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        }

        #[test]
        fn challenge_2() {
            assert_eq!(fixed_xor("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965"), "746865206b696420646f6e277420706c6179".to_owned());
        }

        #[test]
        fn challenge_3() {
            assert_eq!(single_byte_xor_cipher::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736").1, "Cooking MC\'s like a pound of bacon".to_owned());
        }

        #[test]
        fn challenge_4() {
            let mut file = File::open("4.txt").unwrap();
            let mut reader = BufReader::new(file);
            let mut highest = (0.0, String::new());
            let mut buf = String::new();
            while reader.read_line(&mut buf).unwrap() != 0 {
                let solution = single_byte_xor_cipher::decode(&buf.replace("\n", ""));
                buf.clear();
                if solution.0 > highest.0 {
                    highest = solution;
                }
            }
            assert_eq!(highest.1, "Now that the party is jumping\n".to_owned());
        }

        #[test]
        fn challenge_5() {
            assert_eq!(repeating_key_xor::encode("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal", "ICE"), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_owned());
        }

        #[test]
        fn challenge_6() {
            assert_eq!(hamming_distance("this is a test", "wokka wokka!!!"), 37);
            //let strings = single_byte_xor_cipher::to_hex_bytes("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").iter().fold(String::new(), |mut acc, b| {
                    //acc.push(*b as char);
                    //acc
                //});
            //assert_eq!(repeating_key_xor::decode(&strings.as_str()).0, "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_owned());
            let mut file = File::open("6.txt").unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            let buf = buf.replace("\n", "");
            let data = base64::decode(&buf).unwrap();
            assert_eq!(repeating_key_xor::decode(&data).1, "Terminator X: Bring the noise".to_owned());
        }

        #[test]
        fn challenge_7() {
            let mut file = File::open("7.txt").unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf);
            let buf = buf.replace("\n", "");
            let data = base64::decode(&buf).unwrap();
            let cipher = openssl::symm::Cipher::aes_128_ecb();
            let key = "YELLOW SUBMARINE";
            let text = String::from_utf8(openssl::symm::decrypt(cipher, key.as_bytes(), None, &data).unwrap()).unwrap();

            assert!(text.starts_with("I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell"));
        }

        #[test]
        fn challenge_8() {
            let mut file = File::open("8.txt").unwrap();
            let mut buf = String::new();
            let mut candidates = vec![];
            file.read_to_string(&mut buf);
            for line in buf.lines() {
                let mut set = HashSet::new();
                let mut i = 0;
                while i < line.len() - 4 {
                    set.insert(line[i..i + 3].to_owned());
                    i += 4;
                }
                if set.len() < (line.len() / 4 - 10) {
                    candidates.push((line, set.len()));
                }
                //println!("...{}: {}, {}", line.replacen(|_| true, "", line.len() - 20), set.len(), line.len() / 4);
            }
            assert_eq!(candidates[0].1, 55);
        }
    }
}
