<h1 align="center">iceworx</h1>
<br>
<p align="center">
  <img src="https://raw.githubusercontent.com/gcoguiec/iceworx/main/.github/icewerx.png" width="180" alt=""/>
</p>

<p align="center">
  Flasher utility for the iceWerx iCE40 FPGA boards.
</p>

<hr>

## Table of Contents

- [Install from sources](#install-from-sources)
- [Install from pre-compiled binaries](#install-from-pre-compiled-binaries)
- [Getting Started](#getting-started)
- [Examples](#examples)
- [License](#license)

## Install from sources

```
cargo install --git https://github.com/gcoguiec/iceworx --tag v0.1.0
```

## Install from pre-compiled binaries

| OS      | Arch   | URL                                                                                               |
| ------- | ------ | ------------------------------------------------------------------------------------------------- |
| Linux   | x86_64 | https://github.com/gcoguiec/iceworx/releases/latest/download/iceworx-x86_64-unknown-linux-gnu.zip |
| macOS   | arch64 | https://github.com/gcoguiec/iceworx/releases/latest/download/iceworx-arch64-apple-darwin.zip      |
| macOS   | x86_64 | https://github.com/gcoguiec/iceworx/releases/latest/download/iceworx-x86_64-apple-darwin.zip      |
| Windows | x86_64 | https://github.com/gcoguiec/iceworx/releases/latest/download/iceworx-x86_64-pc-windows-msvc.zip   |

## Getting Started

```bash
➜ iceworx boards
┌────────────────┬────────────┬────────────────────────────┐
│ Vendor         │ Board Name │ Device Path                │
├────────────────┼────────────┼────────────────────────────┤
│ Devantech Ltd. │ iceFUN     │ /dev/<device>              │
└────────────────┴────────────┴────────────────────────────┘

➜ iceworx flash --device /dev/<device> ./fpga.bin
```

## Examples

A bunch of working examples are available in the [examples/icewerx](https://github.com/gcoguiec/iceworx/tree/main/examples/icewerx) folder.

## License

This project is licensed under [BSD 2-Clause](https://spdx.org/licenses/BSD-2-Clause.html).
