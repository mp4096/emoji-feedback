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
