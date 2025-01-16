# c-builder
A simple C project builder for Linux.

## Features
 - [x] Initialize a new C project
 - [x] Build the project
 - [x] Run the project
 - [x] Run the project with memory checks
 - [x] Clean the project
 - [ ] Run tests on the project

## Installation
```bash
git clone https://github.com/Huggepugge1/c-builder.git c-builder
cd c-builder
cargo install --path .
```

## Usage
### Initializing a new project
```bash
c-builder init <project-name> <options>
```

### Building the project
```bash
c-builder build <options>
```

### Running the project
```bash
c-builder run <options>
```

### Running the project with memory checks
```bash
c-builder memory-run <options>
```

### Cleaning the project
```bash
c-builder clean <options>
```

### Help
```bash
c-builder --help
c-builder help
```

## Contributing
Contributions are welcome! Please read the [contribution guidelines](CONTRIBUTING.md) first.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```
