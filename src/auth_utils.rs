use data_encoding::{BASE64URL, DecodeError};

pub fn check_access_token<T: AsRef<str>>(token: T, salt: T, hash: T) -> bool {
    use constant_time_eq::constant_time_eq;

    let hashed_token = salt_and_hash(salt, token);
    let should_be = BASE64URL.decode(hash.as_ref().as_bytes());

    match (hashed_token, should_be) {
        (Ok(ref a), Ok(ref b)) if constant_time_eq(a, b) => true,
        _ => false,
    }
}

fn salt_and_hash<T: AsRef<str>>(salt: T, token: T) -> Result<Vec<u8>, DecodeError> {
    use blake2::{Blake2s, Digest};

    let salt_bytes = BASE64URL.decode(salt.as_ref().as_bytes())?;
    let token_bytes = BASE64URL.decode(token.as_ref().as_bytes())?;

    let mut hasher = Blake2s::default();
    hasher.input(&salt_bytes[..]);
    hasher.input(&token_bytes[..]);

    Ok(hasher.result().into_iter().collect())
}
