# Emoji feedback

[![Build Status](https://travis-ci.org/mp4096/emoji-feedback.svg?branch=master)](https://travis-ci.org/mp4096/emoji-feedback)

## Quickstart
1. Install the latest Rust nightly
1. Clone the repo and `cd` into it
1. `cargo run -- ./examples/en.toml`
1. Open [`localhost:8000`](http://localhost:8000) in your browser
1. Check out the results in `./feedback.csv`

## Runtime dependencies

```
$ cargo build --release
$ ldd target/release/emoji-feedback
    linux-vdso.so.1 =>  (0x00007fffec5b5000)
    libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007f0a6fa2e000)
    librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007f0a6f826000)
    libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007f0a6f609000)
    libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007f0a6f3f3000)
    libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007f0a6f029000)
    /lib64/ld-linux-x86-64.so.2 (0x00007f0a702b2000)
    libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007f0a6ed20000)
```

## Disclaimer
* Only modern browsers (Chromium >= 42 on Linux, Chrome >= 42 on Android)
  are supported (Fetch API required).
* This software is undocumented, untested and unsecure. :construction: :boom: Use at your own risk.


## Deployment on a Raspberry Pi
First, install the cross-compilation toolchain:

```sh
rustup target add armv7-unknown-linux-gnueabihf
sudo apt-get install gcc-arm-linux-gnueabihf
```

Now build the binary and prepare the deployment bundle:

```sh
make xcompile-arm
make prepare-deployment-bundle
```

Copy files to the Raspberry Pi:

```sh
scp -r './deployment-ef' pi@<rpi ip address>:
```

Finally, add the following lines to `/etc/rc.local` (`sudo vim /etc/rc.local`):

```sh
export ROCKET_ENV=production
cd /home/pi/deployment-ef && ./emoji-feedback en.toml &
```

## Remotely accessing the log file

You have to generate an access token with `python generate_auth.py` (you will need Python >= 3.6).
Add the `auth.toml` snippet to the configuration file and use the token
in a GET request to `/log_file/<token>`, e.g. for the example config files use:

```sh
curl -X GET localhost:8000/log_file/9nmU49b8yqybNjUmkHDrPZNn
```

You can reset the log file by sending a DELETE request to `/log_file/<token>`.
The log file will be moved to the location specified as `backup_file` in the config file.

```sh
curl -X DELETE localhost:8000/log_file/9nmU49b8yqybNjUmkHDrPZNn
```
