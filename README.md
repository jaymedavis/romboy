# romman

Romman is a rom manager that manages extracting your roms, and putting them in your library under the correct platform.

## Run Source

* [Install Rust](https://rustup.rs/)

* Open `settings.toml` and set `zips` and `roms` under `[path]`.
    - zips is where your downloaded files are (_~/Downloads_ by default, but I would recommend moving them into their own folder such as _~/Downloads/zips_)
    - roms is the location of your rom library.  This could be your [RomM Library](https://romm.app), syncthing path, or an SD card.  For this release, the path is expected to be in the `roms/platform/game` format.  e.g.  roms/nes/Legend of Zelda, The.nes

* ``cargo run``
