use anyhow::{anyhow, Result};
use chacha20poly1305::{aead::Aead, AeadCore, KeyInit, XChaCha20Poly1305};
use rand::{rngs::OsRng, RngCore};
use std::path::PathBuf;
use std::{
    fs::File,
    io::{Read, Write},
};
use zeroize::Zeroize;

pub fn encrypt(file: PathBuf, password: impl AsRef<str>) -> Result<Vec<u8>> {
    let argon2_config = argon2_config();

    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);

    let mut nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let mut key = argon2::hash_raw(password.as_ref().as_bytes(), &salt, &argon2_config)?;
    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());

    let mut plaintext = Vec::new();
    File::open(file)?.read_to_end(&mut plaintext)?;

    let encrypted = aead
        .encrypt(&nonce, plaintext.as_ref())
        .map_err(|err| anyhow!("Error encrypting file: {}", err))?;

    let mut out = Vec::new();

    out.write_all(&salt)?;
    out.write_all(&nonce)?;
    out.write_all(&encrypted)?;

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(out)
}

pub fn decrypt(file: PathBuf, password: impl AsRef<str>) -> Result<Vec<u8>> {
    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 24];

    let mut encrypted_file = File::open(file)?;

    let mut read_count = encrypted_file.read(&mut salt)?;
    if read_count != salt.len() {
        return Err(anyhow!("Error reading salt."));
    }

    read_count = encrypted_file.read(&mut nonce)?;
    if read_count != nonce.len() {
        return Err(anyhow!("Error reading nonce."));
    }

    let argon2_config = argon2_config();

    let mut key = argon2::hash_raw(password.as_ref().as_bytes(), &salt, &argon2_config)?;
    let aead = XChaCha20Poly1305::new(key[..32].as_ref().into());

    let mut ciphertext = Vec::new();
    encrypted_file.read_to_end(&mut ciphertext)?;

    let decrypted = aead
        .decrypt(nonce.as_ref().into(), ciphertext.as_ref())
        .map_err(|err| anyhow!("Error decrypting file: {}", err))?;

    salt.zeroize();
    nonce.zeroize();
    key.zeroize();

    Ok(decrypted)
}

fn argon2_config<'a>() -> argon2::Config<'a> {
    argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        mem_cost: 46 * 1024,
        // degree of parallelism
        lanes: 1,
        // number of iterations
        time_cost: 1,
        ..Default::default()
    }
}
