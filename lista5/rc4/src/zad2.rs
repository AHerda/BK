use super::utils::*;

/// Check if two ciphers are encrypted with the same key
pub fn check_same_key(cipher1: &String, cipher2: &String) -> bool {
    let cipher1 = bytes_from_hex_string(cipher1);
    let cipher2 = bytes_from_hex_string(cipher2);

    for i in 0..cipher1.len().min(cipher2.len()) {
        if cipher1[i] ^ cipher2[i] >= 0x80 {
            return false;
        }
    }

    true
}

/// Check if two ciphers are encoded with the same key knowing plain text from which they were encrypted
pub fn check_same_key_with_plain_text(
    plain_text1: &String,
    cipher1: &String,
    plain_text2: &String,
    cipher2: &String,
) -> bool {
    let cipher1 = bytes_from_hex_string(cipher1);
    let cipher2 = bytes_from_hex_string(cipher2);

    let len = plain_text1.len().min(plain_text2.len());

    !plain_text1[0..len]
        .chars()
        .zip(cipher1[0..len].iter())
        .zip(plain_text2[0..len].chars())
        .zip(cipher2[0..len].iter())
        .any(|(((s1, c1), s2), c2)| s1 == s2 && c1 != c2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_same_key1() {
        let cipher1 = String::from("B2 39 63");
        let cipher2 = String::from("B2 39 63");
        assert!(check_same_key(&cipher1, &cipher2));
    }

    #[test]
    fn test_check_same_key2() {
        let cipher1 = String::from("B2 39 63");
        let cipher2 = String::from("02 39 64");
        assert!(!check_same_key(&cipher1, &cipher2));
    }

    #[test]
    fn test_check_same_key_with_plain_text1() {
        let text1 = String::from("abc");
        let text2 = String::from("abc");
        let cipher1 = String::from("B2 39 63");
        let cipher2 = String::from("B2 39 63");
        assert!(check_same_key_with_plain_text(
            &text1, &cipher1, &text2, &cipher2
        ));
    }

    #[test]
    fn test_check_same_key_with_plain_text2() {
        let text1 = String::from("abc");
        let text2 = String::from("zbc");
        let cipher1 = String::from("B2 39 64");
        let cipher2 = String::from("02 39 64");
        assert!(check_same_key_with_plain_text(
            &text1, &cipher1, &text2, &cipher2
        ));
    }

    #[test]
    fn test_check_same_key_with_plain_text3() {
        let text1 = String::from("abc");
        let text2 = String::from("abc");
        let cipher1 = String::from("B2 39 63");
        let cipher2 = String::from("02 39 63");
        assert!(!check_same_key_with_plain_text(
            &text1, &cipher1, &text2, &cipher2
        ));
    }
}
