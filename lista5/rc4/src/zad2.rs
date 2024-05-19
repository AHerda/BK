use super::utils::*;

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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_check_same_key() {
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
}
