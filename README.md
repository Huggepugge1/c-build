# c-builder
A simple C project builder for Linux.

## Features
 - [x] Initialize a new C project
 - [x] Build the project with parallel jobs
 - [x] Run the project
 - [x] Clean the project
 - [x] Run tests on the project
 - [x] Run benchmarks on the project

## Installation
```bash
git clone https://github.com/Huggepugge1/c-builder.git c-builder
cd c-builder
cargo install --path .
```

## Basic Usage
```bash
c-builder [options] init [project-name]
c-builder [options] build
c-builder [options] run
```

See more detailed information in [documentation](./docs/README.md).

### Configuration
The configuration file is located at `~/.config/c-builder/config.toml`. See [configuration](./docs/README.md#configuration) for more information.

## Contributing
Contributions are welcome! Please read the [contribution guidelines](CONTRIBUTING.md) first.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Dependencies
 - [gcc](https://gcc.gnu.org/)
 - [valgrind](https://www.valgrind.org/)
 - [cargo](https://doc.rust-lang.org/cargo/)
 - [rust](https://www.rust-lang.org/)
 - [gprof](https://sourceware.org/binutils/docs/gprof/)
