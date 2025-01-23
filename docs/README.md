# c-builder
`c-builder` aims to simplify the process of building, running, testing and benchmarking C projects.

For information about the custom testing framework, see [here](./testing.md)

## Table of contents
 - [Features](#features)
 - [Installation](#installation)
 - [Usage](#usage)
   - [Initializing](#initializing-a-new-project)
   - [Building](#building)
   - [Running](#running)
   - [Testing](#testing)
 - [Configuration](#configuration)

## Features
 - Initializing a new C project
 - Building the project with parallel jobs
 - Running the project
 - Run tests on the project
 - Run benchmarks on the project
 - Clean the project
 - Customization of the build configuration

## Installation
```bash
git clone
cd c-builder
cargo install --path .
```

## Usage
### Debug vs Release Mode
The project can be built in either debug or release mode.
Debug mode is the default mode and is used for development.
Release mode is used for production and has optimizations enabled and strips debug information.
You can specify the release mode with the `-r` or `--release` flag.

#### Notes
 - Memory leak detection is available for both debug and release mode.
 - Debug mode is used by default in the tests but this can be changed with the `-r` or `--release` flag.
 - The benchmarks are always built in release mode.

### Initializing a New Project
```bash
c-builder [options] init [project-name]
```
#### Options available
 - `-h --help`: Display help information

### Building
```bash
c-builder [options] build
```
#### Options available
 - `-r --release`: Build the project in release mode
 - `-b --benchmark`: Build the benchmarks
 - `-h --help`: Display help information

#### Notes
 - The `release` and `benchmark` flags are exclusive and cannot be used together.

### Running
```bash
c-builder [options] run
```
#### Options available
 - `-r --release`: Run the project in release mode
 - `-b --benchmark`: Run the benchmarks
 - `-h --help`: Display help information

#### Notes
 - The `release` and `benchmark` flags are exclusive and cannot be used together.

### Testing
```bash
c-builder [options] test
```
#### Options available
 - `[-s --single] <test_name>`: Run a single test
 - `-r --release`: Run the tests in release mode
 - `-h --help`: Display help information

## Configuration
The configuration file is located at `c-builder.toml`.
All fields shown are required.
The default configuration is:
```toml
[package]
name = "Your Project"
version = "0.1.0"
authors = ["Your Names"]
src = "src"
benchmark = "benchmark"
    
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
pedantic = true
std = "c2x"

[memory]
leak_check = "full"
show_leak_kinds = "all"
track_origins = true
```
