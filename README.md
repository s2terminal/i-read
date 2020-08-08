# I read
Extracting commands from README markdown file. This is Rust version of ["I read you (ireadu)"](https://github.com/s2terminal/i-read-u).

Supported on Linux and Windows Subsystems for Linux.

## Install
```bash
$ curl --silent https://raw.githubusercontent.com/s2terminal/i-read/master/install.sh | sudo /bin/sh
```

or [get binary](https://github.com/s2terminal/i-read/releases).

### Uninstall
```bash
$ rm --interactive /usr/local/bin/iread
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

### Testing (wip)
```bash
$ cargo check
```

## License
[MIT](LICENSE).
