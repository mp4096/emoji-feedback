# Emoji feedback

[![Build Status](https://travis-ci.org/tum-rt/emoji-feedback.svg?branch=master)](https://travis-ci.org/tum-rt/emoji-feedback)

## Quickstart
1. Install the latest Rust nightly (>= 2017-02-26)
2. Clone the repo and `cd` into it
3. `cargo run -- ./examples/en.toml`

## Disclaimer
* Only modern browsers (Chromium on Linux, Chrome on Android) are supported (Fetch API required).
* This software is undocumented, untested and unsecure. :construction: :boom: Use at your own risk.


## Deployment on a Raspberry Pi
First, install the cross-compilation toolchain:

```sh
rustup target add arm-unknown-linux-gnueabihf
sudo apt-get install gcc-arm-linux-gnueabihf
```

Now build the binary:

```sh
cargo build --release --target=arm-unknown-linux-gnueabihf
```

Copy files to the Raspberry Pi:

```
mkdir ./target/deployment_ef
cp ./target/arm-unknown-linux-gnueabihf/release/emoji-feedback ./target/deployment_ef
cp ./examples/en.toml ./target/deployment_ef
cp -r ./static/ ./target/deployment_ef

scp -r ./target/deployment_ef pi@<Raspberry Pi's IP address>:
```

Finally, add the following lines to `/etc/rc.local` (`sudo vim /etc/rc.local`):

```sh
export ROCKET_ENV=production
cd /home/pi/deployment_ef && ./emoji-feedback en.toml
```
