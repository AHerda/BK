use rc4::{zad1, utils::*};

fn main() {
    let text = String::from("Hello, world!  ajkskjdhASHDASJHaksjhdkasasdaskgfhsdffsadfgshdjkASJDANSNBNgtfdrsahjyjngfftr7854nmdASGGHAJSJAStg hba3we4ry 56ok789o6wdc hu;ol'hk36w4rgv  dgfdgfasffgfg");
    let text2 = String::from("ajkskjdhaksjhdkasasdas\ndasffgfgdasdasdasslgdklajsdaajksk\njdhaksjhdkasasdasda q45erghbheagjn 4w5yy3 4rgv sffgfgdasdasdasslgdklajsda");
    let key = String::from("Key");
    let _key2 = String::from("Key2");

    let encrypted_text = zad1::encrypt(&key, &text);
    let encrypted_text2 = zad1::encrypt(&key, &text2);

    let encrypted_bytes = bytes_from_hex_string(&encrypted_text);
    let encrypted_bytes2 = bytes_from_hex_string(&encrypted_text2);

    let mut encrypted_bytes_xor = Vec::new();
    for i in 0..encrypted_bytes.len().min(encrypted_bytes2.len()) {
        encrypted_bytes_xor.push(encrypted_bytes[i] ^ encrypted_bytes2[i]);
    }
    let decrypted_text = zad1::decrypt(&key, &encrypted_text);

    println!("Text: {}", text);
    println!("Text2: {}", text2);
    println!("Key: {}\n", key);

    println!(" Encrypted text: {}", encrypted_text);
    println!("Encrypted text2: {}\n", encrypted_text2);

    print!("Encrypted bytes: ");
    for i in 0..encrypted_bytes_xor.len() {
        print!("{:02X} ", encrypted_bytes_xor[i]);
    }
    println!("Decrypted text: {}", decrypted_text);
}
