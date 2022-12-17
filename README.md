# 0x0.st 🦄

> This is my first rust project. It's probably not the best code, but it works. If you have any suggestions, feel free to open an issue or a pull request.

## What is this?

This is a wrapper for [0x0.st](https://0x0.st) that allows you to upload files from the command line.

### Installation

```bash
$ curl -sL https://raw.githubusercontent.com/voxelin/0x0/master/install.sh | sh
```

### Building

You need to have rust installed. You can get it from [here](https://www.rust-lang.org/tools/install).

Then, you can build it with `cargo build --release`.

### Usage

You can upload a file with `0x0 <file>`. This will print the URL to stdout.
Example:

```
$ 0x0 ./README.md
- Uploading file...
- Done!
https://0x0.st/uZ7n.md
```
