## chaintrak

events from our currently most important contracts on [CloudWalk Mainnet](https://explorer.mainnet.cloudwalk.io/) are mapped into one-shot samples to produce vibes.

just pick one:

> jazz 🎷

> ambient 😶‍🌫️


### install
you can either download the binary or build it from source yourself

##### build from source
- install dependencies. the main one is ALSA, check the name of the libs for your OS.

no support for Windows sorry.

on GNU/Linux Ubuntu, for example, run:

``` sh
sudo apt update && sudo apt install -y libasound2-dev pkg-config
```

- install the Rust toolchain
- run `cargo build --release`

##### listen
- start the application with cargo or directly running the binary and pick your vibe:

``` sh
cargo run -- --vibe jazz
```
or

``` sh
./target/release/chaintrak --vibe ambient
```

#### TODO
- lazy load of samples depending on the vibe
- normalize volumes 

#### license
[![License: WTFPL](https://img.shields.io/badge/License-WTFPL-brightgreen.svg)](http://www.wtfpl.net/about/)

© 2022 WTFPL – Do What the Fuck You Want to Public License.
