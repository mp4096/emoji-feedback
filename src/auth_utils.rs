use rustc_serialize::base64::FromBase64Error;

pub fn check_access_token(token: &str, salt: &str, hash: &str) -> bool {
    use rustc_serialize::base64::FromBase64;

    let hashed_token = salt_and_hash(salt, token);
    let should_be = hash.from_base64();

    match (hashed_token, should_be) {
        (Ok(ref a), Ok(ref b)) if &a[..] == &b[..] => true,
        _ => false,
    }
}

fn salt_and_hash(salt: &str, token: &str) -> Result<Vec<u8>, FromBase64Error> {
    use rustc_serialize::base64::FromBase64;
    use blake2::{Blake2s, Digest};

    let salt_bytes = salt.from_base64()?;
    let token_bytes = token.from_base64()?;

    let mut hasher = Blake2s::default();
    hasher.input(&salt_bytes[..]);
    hasher.input(&token_bytes[..]);

    Ok(hasher.result().into_iter().collect())
}
