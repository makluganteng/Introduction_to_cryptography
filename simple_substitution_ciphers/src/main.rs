fn encrypt(input: &str, shift: usize) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let encrypted = ((c as u8 - base + shift as u8) % 26) + base;
                encrypted as char
            } else {
                c
            }
        })
        .collect()
}

fn decrypt(input: &str, shift: usize) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let decrypted = ((c as u8 - base + 26 - shift as u8) % 26) + base;
                decrypted as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let plaintext = "Hello, World!";
    println!("Plaintext: {}", plaintext);

    let encrypted = encrypt(plaintext, 3);
    println!("Encrypted: {}", encrypted);

    let decrypted = decrypt(&encrypted, 3);
    println!("Decrypted: {}", decrypted);
}
