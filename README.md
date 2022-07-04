## chaintrak

events from `BRLC`, `PixCashier`, `SpinMachine` and `CErc20Delegate` contracts and each new block from [CloudWalk Mainnet](https://explorer.mainnet.cloudwalk.io/) are mapped into one-shot samples to produce vibes

pick one:

> jazz 🎷

> ambient 😶‍🌫️


### install
you can either [download the binary](https://github.com/arthur-cw/chaintrak/releases) or build it from source yourself

##### build from source
- install dependencies. the main one is ALSA, check the name of the libs for your OS. no Windows support for now, sorry.

on GNU/Linux Ubuntu, for example, run:

``` sh
sudo apt update && sudo apt install -y libasound2-dev pkg-config
```

- install the Rust toolchain
- run `cargo build --release`

##### listen
- start the application by picking your current vibe:

``` sh
./target/release/chaintrak jazz
./target/release/chaintrak ambient
```

#### TODO
- normalize volumes
- improve logs (show contract name)
- add more vibez

#### license
[![License: WTFPL](https://img.shields.io/badge/License-WTFPL-brightgreen.svg)](http://www.wtfpl.net/about/)

© 2022 WTFPL – Do What the Fuck You Want to Public License.
