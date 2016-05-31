mod hex_to_base64;

#[cfg(test)]
mod challenges {

    mod set_1 {
        use hex_to_base64::hex_to_base64;

        #[test]
        fn challenge_1() {
            assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
                       "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
        }
    }
}
