pub fn bytes_from_hex_string(hex_string: &String) -> Vec<u8> {
    hex_string
        .as_bytes()
        .chunks(3)
        .map(|x| u8::from_str_radix(std::str::from_utf8(&x[0..2]).unwrap(), 16).unwrap())
        .collect::<Vec<u8>>()
}
