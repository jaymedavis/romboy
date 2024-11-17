<img src="assets/logo.png" alt="drawing" width="150"/>

extracts roms from [zip files](https://myrient.erista.me/files/), and puts them in your library under the correct platform

<br/>

## Quick Start

* Download the [latest release](https://github.com/jaymedavis/romboy/releases)

* * Open `settings.toml` and set `zips` and `roms` under `[path]`.
    - zips is where your downloaded roms are (e.g. _~/Downloads/rom_zips_)
    - roms is the location of your rom library.  This could be your [RomM Library](https://romm.app), syncthing library, or an SD card.  

For now, the path is expected to be in the `roms/platform/game` format.  e.g.  roms/nes/Legend of Zelda, The.nes

A command window will launch in the background, so you can see the logs

## Run Source

* [Install Rust](https://rustup.rs/)

* ``cargo run``

<br/>

## v0.1.0
![romboy screen](assets/romboy.png)
