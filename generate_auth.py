import codecs
from getpass import getpass
from hashlib import blake2s
from secrets import token_bytes
import sys

if __name__ == "__main__":
    # Generate salt
    salt = token_bytes(blake2s.SALT_SIZE)
    # Initialise hasher object
    h = blake2s(salt=salt)

    # Read and hash password
    pswd = getpass(prompt="Input new password:")
    h.update(pswd.encode(sys.stdin.encoding))

    # Write salt and hash to a TOML stub
    with codecs.open("auth.toml", 'w', encoding="utf-8") as f:
        f.write("[auth]\n")
        f.write('salt="{:s}"\n'.format(salt.hex()))
        f.write('hash="{:s}"\n'.format(h.digest().hex()))
