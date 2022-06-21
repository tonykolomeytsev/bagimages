#  bagimages 🤖 👜 🖼️ export images from rosbag files

![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/tonykolomeytsev/bagimages?label=version) 
![GitHub license](https://img.shields.io/github/license/tonykolomeytsev/bagimages)

A multi-platform tool for exporting images from rosbag files.

## Features

- Export from several topics at the same time.
- Specify from what second (in bag's time) to start the export.
- Specify number of images to export.

## How to install?

> Installation via package managers will be available later.

### Installation on Ubuntu

Just run on terminal:

* `curl -o- https://raw.githubusercontent.com/tonykolomeytsev/fxa/master/install.sh | bash` — for Ubuntu

* Or download suitable executable from the[latest release](https://github.com/tonykolomeytsev/fxa/releases/latest).

### Installation on Windows / MacOS

Download and run from terminal suitable executable from the [latest release](https://github.com/tonykolomeytsev/fxa/releases/latest).

### Build source code

Install Rust: https://www.rust-lang.org/tools/install

And then clone and build the project with `cargo`:

```bash
cargo build --release
```

And then take the compiled program `{project_root}/target/release/bagimages`

## How to use?

TODO

## Limitations

At the moment, the utility is guaranteed to correctly convert SVG icons to XML only if the icons do not have gradient fills and all elements (such as `<rect>`) have already been converted to `<path>`.

## Project status

The project is in progress and is being developed just for fun. Additional features will be added in the future.
