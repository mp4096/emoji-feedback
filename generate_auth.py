from base64 import urlsafe_b64encode
from hashlib import blake2s
from secrets import token_bytes

SALT_SIZE = 18
TOKEN_SIZE = 18

if __name__ == "__main__":
    # Generate salt
    salt = urlsafe_b64encode(token_bytes(SALT_SIZE))
    # Generate access token
    token = urlsafe_b64encode(token_bytes(TOKEN_SIZE))
    # Initialise a hasher object and hash the token
    # We don't use blake2s's native salt handling here since it's not
    # implemented by Rust's blake2 crate
    h = blake2s()
    h.update(salt + token)
    digest = urlsafe_b64encode(h.digest())

    # Write the token to a file
    with open("token", "wb") as f:
        f.write(token + b"\n")

    # Write salt and hash to a TOML stub
    with open("auth.toml", "wb") as f:
        f.write(b"[auth]\n")
        f.write(b'salt="' + salt + b'"\n')
        f.write(b'hash="' + digest + b'"\n')
