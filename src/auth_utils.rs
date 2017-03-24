use rustc_serialize::base64::FromBase64Error;

pub fn check_access_token<T: AsRef<str>>(token: T, salt: T, hash: T) -> bool {
    use rustc_serialize::base64::FromBase64;

    let hashed_token = salt_and_hash(salt, token);
    let should_be = hash.as_ref().from_base64();

    match (hashed_token, should_be) {
        (Ok(ref a), Ok(ref b)) if &a[..] == &b[..] => true,
        _ => false,
    }
}

fn salt_and_hash<T: AsRef<str>>(salt: T, token: T) -> Result<Vec<u8>, FromBase64Error> {
    use rustc_serialize::base64::FromBase64;
    use blake2::{Blake2s, Digest};

    let salt_bytes = salt.as_ref().from_base64()?;
    let token_bytes = token.as_ref().from_base64()?;

    let mut hasher = Blake2s::default();
    hasher.input(&salt_bytes[..]);
    hasher.input(&token_bytes[..]);

    Ok(hasher.result().into_iter().collect())
}
