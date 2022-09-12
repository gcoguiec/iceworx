# blinky-amaranth

Makes the two LEDs available on the board blink.

What this `amaranth` example covers:

- input mapping
- always-block
- clock division
- logical complement

## Dependencies

- python3
- [yosys](https://github.com/YosysHQ/yosys)
- [nextpnr](https://github.com/YosysHQ/nextpnr)
- [icestorm](https://github.com/YosysHQ/icestorm)
- [iceworx](https://github.com/gcoguiec/iceworx/)
- [just](https://just.systems/man/en/chapter_4.html)
- gtkwave

If you have an **Apple Silicon** Mac, you're in luck! I happen to maintain the necessary Brew formulas in a dedicated tap:

```sh
brew tap gcoguiec/tap
brew install just yosys gtkwave gcoguiec/tap/nextpnr-ice40 gcoguiec/tap/icestorm gcoguiec/tap/iceworx
```

You might have to [fix GTKWave on macOS](https://ughe.github.io/2018/11/06/gtkwave-osx).

## Getting Started

```
just install
```

## Quick Usage

```sh
just flash /dev/<device> # Flash the device.
```

## License

This example is licensed under [BSD 2-Clause](https://spdx.org/licenses/BSD-2-Clause.html).
