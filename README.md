# romboy

romboy extracts your roms from zip files, and puts them in your library in the correct platform.

## Run Source

* [Install Rust](https://rustup.rs/)

* Open `settings.toml` and set `zips` and `roms` under `[path]`.
    - zips is where your downloaded files are (e.g. _~/Downloads/rom_zips_)
    - roms is the location of your rom library.  This could be your [RomM Library](https://romm.app), syncthing library, or an SD card.  For now, the path is expected to be in the `roms/platform/game` format.  e.g.  roms/nes/Legend of Zelda, The.nes

* ``cargo run``
