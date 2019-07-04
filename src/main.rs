use openssl::symm::{decrypt, encrypt, Cipher, Crypter, Mode};

pub fn decrypt_data_1(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    let mut decrypted = Crypter::new(cipher, Mode::Decrypt, key, None).unwrap();
    let mut output = vec![0 as u8; data.len() + Cipher::aes_128_cbc().block_size()];

    let decrypted_result = decrypted.update(&data, &mut output);

    match decrypted_result {
        Ok(_) => output,
        Err(e) => panic!("Error decrypting text: {}", e),
    }
}

pub fn decrypt_data_2(encrypted: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher = Cipher::aes_128_ecb();

    decrypt(cipher, key, None, encrypted).unwrap()
}

fn main() {
    let key = "YELLOW SUBMARINE";

    let line = vec![
        9, 18, 48, 170, 222, 62, 179, 48, 219, 170, 67, 88, 248, 141, 42, 108,
    ];

    println!(
        "{:?}",
        // This works just fine
        &decrypt_data_1(&line, key.as_bytes())
    );

    // This does *not* work
    println!("{:?}", decrypt_data_2(&line, key.as_bytes()));
}
