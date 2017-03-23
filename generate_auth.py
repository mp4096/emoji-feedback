from hashlib import blake2s
from secrets import token_bytes

TOKEN_SIZE = 16

if __name__ == "__main__":
    # Generate salt
    salt = token_bytes(blake2s.SALT_SIZE)
    # Generate access token
    token = token_bytes(TOKEN_SIZE)
    # Initialise a hasher object and hash the token
    h = blake2s(salt=salt)
    h.update(token)

    # Write the token to a file
    with open("token", 'w') as f:
        f.write("{:s}\n".format(token.hex()))

    # Write salt and hash to a TOML stub
    with open("auth.toml", 'w') as f:
        f.write("[auth]\n")
        f.write('salt="{:s}"\n'.format(salt.hex()))
        f.write('hash="{:s}"\n'.format(h.digest().hex()))
