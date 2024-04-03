# ⚠️ This projet is now archived

The tool still work if you need it but I don't plan to maintain it anymore.

---

<h1 align="center">iceworx</h1>
<p align="center">
  <a href="https://github.com/gcoguiec/iceworx/actions/workflows/ci.yml"><img src="https://github.com/gcoguiec/iceworx/actions/workflows/ci.yml/badge.svg" /></a>
</p>
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
cargo install --git https://github.com/gcoguiec/iceworx --tag v0.3.0
```

## Install from pre-compiled binaries

| OS             | Arch    | Link                                                                                                                                       |
| -------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Linux (gnu)    | x86_64  | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-x86_64-unknown-linux-gnu.tar.gz)       |
| Linux (musl)   | x86_64  | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-x86_64-unknown-linux-musl.tar.gz)      |
| Linux (gnu)    | aarch64 | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-aarch64-unknown-linux-gnu.tar.gz)      |
| Linux (musl)   | aarch64 | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-aarch64-unknown-linux-musl.tar.gz)     |
| Linux (gnu)    | armv7   | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-armv7-unknown-linux-gnueabihf.tar.gz)  |
| Linux (musl)   | armv7   | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-armv7-unknown-linux-musleabihf.tar.gz) |
| macOS          | aarch64 | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-aarch64-apple-darwin.zip)              |
| macOS          | x86_64  | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-x86_64-apple-darwin.zip)               |
| Windows (msvc) | x86_64  | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-x86_64-pc-windows-msvc.zip)            |
| Windows (gnu)  | x86_64  | [Download Pre-Compiled](https://github.com/gcoguiec/iceworx/releases/download/v0.3.0/iceworx-v0.3.0-x86_64-pc-windows-gnu.zip)             |

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
