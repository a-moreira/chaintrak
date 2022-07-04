## chaintrak
[![License: WTFPL](https://img.shields.io/badge/License-WTFPL-brightgreen.svg)](http://www.wtfpl.net/about/)


events from our currently most important contracts on [CloudWalk Mainnet](https://explorer.mainnet.cloudwalk.io/) are mapped into one-shot samples to produce vibes.

just pick one:

> jazz 🎷

> ambient 😶‍🌫️


### listen
you can either download the binary or build it from source yourself

##### build from source
- install dependencies. on Ubuntu, for example, run:

``` sh
sudo apt update && sudo apt install -y libasound2-dev pkg-config
```

- install the Rust toolchain
- run `cargo build --release`
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
