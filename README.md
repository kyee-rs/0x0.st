# 0x0.st 🦄
## This is my first rust project, so it's probably not the best code.

## What is this?
This is a wrapper for [0x0.st](https://0x0.st) that allows you to upload files from the command line.

### How do I use it?
You can either download the binary from the [releases](https://github.com/voxelin/0x0.st/releases) page, or build it yourself.

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