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

## Usage
### Initializing a new project
```bash
c-builder [options] init [project-name]
```

### Building the project
```bash
c-builder [options] build
```

### Help
```bash
c-builder --help
c-builder help
```

### Configuration
The configuration file is located at `~/.config/c-builder/config.toml`. The default configuration is:
```toml
[package]
name = "Your Project"
version = "0.1.0"
authors = ["Your Name"]
src = "src"

[debug]
debug = true
optimization = 0
warnings = true
pedantic = true
std = "c2x"

[release]
debug = false
optimization = 3
warnings = true
pedantic = false
std = "c2x"

[memory]
leak_check = "full"
show_leak_kinds = "all"
track_origins = true
```

## Contributing
Contributions are welcome! Please read the [contribution guidelines](CONTRIBUTING.md) first.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Dependencies
 - [gcc](https://gcc.gnu.org/)
 - [valgrind](https://www.valgrind.org/)
 - [cargo](https://doc.rust-lang.org/cargo/)
 - [rust](https://www.rust-lang.org/)
