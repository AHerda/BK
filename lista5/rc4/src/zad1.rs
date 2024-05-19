use super::utils::*;

fn ksa(key: &[u8]) -> Vec<u8> {
	let n = key.len();
	let mut j = 0;
	let t = (0..256).map(|x| key[(x % n) as usize]).collect::<Vec<u8>>();
	let mut s = (0..256).map(|x| x as u8).collect::<Vec<u8>>();

	for i in 0..256 {
		j = (j + s[i] as usize + t[i] as usize) % 256;
		s.swap(i, j);
	}

	s
}

fn prga(s: &mut Vec<u8>, m: usize) -> Vec<u8> {
	let mut i = 0;
	let mut j = 0;
	let mut ks = Vec::new();

	for _ in 0..m {
		i = (i + 1) % 256;
		j = (j + s[i] as usize) % 256;
		s.swap(i, j);
		ks.push(s[(s[i] as usize + s[j] as usize) % 256]);
	}

	ks
}

fn rc4(key: &String, text: &[u8]) -> Vec<u8> {
	let mut s = ksa(key.as_bytes());
	let keystream = prga(&mut s, text.len());

	text
		.iter()
		.zip(keystream)
		.map(|(a, b)| a ^ b)
		.collect::<Vec<u8>>()
}

pub fn encrypt(key: &String, text: &String) -> String {
	rc4(key, text.as_bytes())
		.iter()
		.map(|x| format!("{:02X} ", x))
		.collect::<String>()
}

pub fn decrypt(key: &String, encrypted_text: &String) -> String {
	let encrypted_text = bytes_from_hex_string(encrypted_text);
	String::from_utf8(rc4(key, &encrypted_text)).unwrap()
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_encrypt() {
		let key = String::from("Key");
		let text = String::from("Plaintext");
		let encrypted_text = encrypt(&key, &text);
		assert_eq!(encrypted_text, String::from("BB F3 16 E8 D9 40 AF 0A D3 "));
	}

	#[test]
	fn test_decrypt() {
		let key = String::from("Key");
		let encrypted_text = String::from("BB F3 16 E8 D9 40 AF 0A D3 ");
		let text = decrypt(&key, &encrypted_text);
		assert_eq!(text, String::from("Plaintext"));
	}

	#[test]
	fn test_encrypt_decrypt() {
		let key = String::from("Key");
		let text = String::from("Plaintext");
		let encrypted_text = encrypt(&key, &text);
		let decrypted_text = decrypt(&key, &encrypted_text);
		assert_eq!(text, decrypted_text);
	}
}
