use rc4::{zad1, zad2};

#[test]
fn test_check_same_key_0() {
    let text1 = String::from("Hello, world!  ajkskjdhASHDASJHaksjhdkasasdaskgfhsdffsadfgshdjkASJDANSNBNgtfdrsahjyjngfftr7854nmdASGGHAJSJAStg hba3we4ry 56ok789o6wdc hu;ol'hk36w4rgv  dgfdgfasffgfg");
    let text2 = String::from("ajkskjdhaksjhdkasasdas\ndasffgfgdasdasdasslgdklajsdaajksk\njdhaksjhdkasasdasda q45erghbheagjn 4w5yy3 4rgv sffgfgdasdasdasslgdklajsda");
    let key = String::from("Key");

    let encrypted_text1 = zad1::encrypt(&key, &text1);
    let encrypted_text2 = zad1::encrypt(&key, &text2);

    assert!(zad2::check_same_key_with_plain_text(
        &text1,
        &encrypted_text1,
        &text2,
        &encrypted_text2
    ));
}

#[test]
fn test_check_same_key_1() {
    let text1 = String::from("Hello, world!  ajkskjdZJDASHhdgfdgfasffgfg");
    let text2 = String::from(
        "ajkskjdhaksjhdkasasdas\ndasffgfgdasdasdasslgdklajsdaajksk\njdhakssslgdklajsda",
    );
    let key = String::from("Key");

    let encrypted_text1 = zad1::encrypt(&key, &text1);
    let encrypted_text2 = zad1::encrypt(&key, &text2);

    assert!(zad2::check_same_key_with_plain_text(
        &text1,
        &encrypted_text1,
        &text2,
        &encrypted_text2
    ));
}

#[test]
fn test_check_same_key_2() {
    let text1 = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus auctor magna at finibus porttitor. Duis blandit, justo quis tempus dignissim, lacus sem tempor mi, eu consectetur eros lectus id dolor.");
    let text2 = String::from("Cras non leo a eros ultrices maximus quis in dolor. Sed vitae justo consectetur, volutpat mauris vel, ornare justo. Integer interdum lectus sit amet mi varius accumsan. Phasellus a eu.");
    let key = String::from("Key");

    let encrypted_text1 = zad1::encrypt(&key, &text1);
    let encrypted_text2 = zad1::encrypt(&key, &text2);

    assert!(zad2::check_same_key_with_plain_text(
        &text1,
        &encrypted_text1,
        &text2,
        &encrypted_text2
    ));
}

#[test]
fn test_check_not_same_key_0() {
    let text1 = String::from("Hello, world!  ajkskjdhASHDASJHaksjhdkasasdaskgfhsdffsadfgshdjkASJDANSNBNgtfdrsahjyjngfftr7854nmdASGGHAJSJAStg hba3we4ry 56ok789o6wdc hu;ol'hk36w4rgv  dgfdgfasffgfg");
    let text2 = String::from("ajkskjdhaksjhdkasasdas\ndasffgfgdasdasdasslgdklajsdaajksk\njdhaksjhdkasasdasda q45erghbheagjn 4w5yy3 4rgv sffgfgdasdasdasslgdklajsda");
    let key = String::from("Key");
    let key2 = String::from("Key2");

    let encrypted_text1 = zad1::encrypt(&key, &text1);
    let encrypted_text2 = zad1::encrypt(&key2, &text2);

    assert!(!zad2::check_same_key_with_plain_text(
        &text1,
        &encrypted_text1,
        &text2,
        &encrypted_text2
    ));
}

#[test]
fn test_check_not_same_key_1() {
    let text1 = String::from("Hello, world!  ajkskjdZJDASHhdgfdgfasffgfg");
    let text2 = String::from(
        "ajkskjdhaksjhdkasasdas\ndasffgfgdasdasdasslgdklajsdaajksk\njdhakssslgdklajsda",
    );
    let key = String::from("Key");
    let key2 = String::from("Key2");

    let encrypted_text1 = zad1::encrypt(&key, &text1);
    let encrypted_text2 = zad1::encrypt(&key2, &text2);

    assert!(!zad2::check_same_key_with_plain_text(
        &text1,
        &encrypted_text1,
        &text2,
        &encrypted_text2
    ));
}

#[test]
fn test_check_not_same_key_2() {
    let text1 = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus auctor magna at finibus porttitor. Duis blandit, justo quis tempus dignissim, lacus sem tempor mi, eu consectetur eros lectus id dolor.");
    let text2 = String::from("Cras non leo a eros ultrices maximus quis in dolor. Sed vitae justo consectetur, volutpat mauris vel, ornare justo. Integer interdum lectus sit amet mi varius accumsan. Phasellus a eu.");
    let key = String::from("Key");
    let key2 = String::from("Key2");

    let encrypted_text1 = zad1::encrypt(&key, &text1);
    let encrypted_text2 = zad1::encrypt(&key2, &text2);

    assert!(!zad2::check_same_key_with_plain_text(
        &text1,
        &encrypted_text1,
        &text2,
        &encrypted_text2
    ));
}

#[test]
fn test_check_not_same_key_with_same_msg() {
    let text1 = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus auctor magna at finibus porttitor. Duis blandit, justo quis tempus dignissim, lacus sem tempor mi, eu consectetur eros lectus id dolor.");
    let text2 = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus auctor magna at finibus porttitor. Duis blandit, justo quis tempus dignissim, lacus sem tempor mi, eu consectetur eros lectus id dolor.");
    let key = String::from("Key");
    let key2 = String::from("Key2");

    let encrypted_text1 = zad1::encrypt(&key, &text1);
    let encrypted_text2 = zad1::encrypt(&key2, &text2);

    assert!(!zad2::check_same_key_with_plain_text(
        &text1,
        &encrypted_text1,
        &text2,
        &encrypted_text2
    ));
}
