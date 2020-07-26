use data_encoding::{DecodeError, BASE64URL};

pub fn check_access_token<T: AsRef<str>>(token: T, salt: T, hash: T) -> bool {
    use constant_time_eq::constant_time_eq;

    let hashed_token = salt_and_hash(salt, token);
    let should_be = BASE64URL.decode(hash.as_ref().as_bytes());

    matches!((hashed_token, should_be), (Ok(ref a), Ok(ref b)) if constant_time_eq(a, b))
}

fn salt_and_hash<T: AsRef<str>>(salt: T, token: T) -> Result<Vec<u8>, DecodeError> {
    use blake2::{Blake2s, Digest};

    let salt_bytes = BASE64URL.decode(salt.as_ref().as_bytes())?;
    let token_bytes = BASE64URL.decode(token.as_ref().as_bytes())?;

    let mut hasher = Blake2s::default();
    hasher.update(&salt_bytes[..]);
    hasher.update(&token_bytes[..]);

    Ok(hasher.finalize().into_iter().collect())
}
