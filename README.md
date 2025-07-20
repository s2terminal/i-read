# I read
Extracting commands from README markdown file. This is Rust version of ["I read you (ireadu)"](https://github.com/s2terminal/i-read-u).

Supported on Linux and Windows Subsystems for Linux.

## Install
```bash
$ curl --silent https://raw.githubusercontent.com/s2terminal/i-read/main/install.sh | sudo /bin/sh
```

or [get binary](https://github.com/s2terminal/i-read/releases).

### Uninstall
```bash
$ rm --interactive /usr/local/bin/iread
```

### ARM Support (Experimental)
```bash
$ rustup target add `uname --machine`-unknown-linux-musl
$ cargo build --release --target `uname --machine`-unknown-linux-musl
$ sudo mv ./target/`uname --machine`-unknown-linux-musl/release/i-read /usr/local/bin/iread
```

## Usage
```bash
$ iread
$ iread ./CONTRIBUTING.md
```
More usage information can be obtained from `$ iread --help`.

### Sample Commands (Try ireadu command and hit this)
```bash
$ pwd
$ ls -la
> w
>id
```

## Developing
git clone and run this.
```bash
$ cargo run
```

You can use also [Docker](https://www.docker.com/) and [VS Code Remote Container](https://code.visualstudio.com/docs/remote/containers) for developing without installing Rust in your environment.

### Test and Lint
```bash
$ cargo check
$ cargo test
$ cargo fmt --check
$ cargo clippy
```

### Check outdated dependencies
```bash
$ cargo outdated --depth 1
```

## License
[MIT](LICENSE).
