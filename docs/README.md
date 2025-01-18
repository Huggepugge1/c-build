# c-builder
`c-builder` aims to simplify the process of building, running, testing and benchmarking C projects.

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
### Initializing a new project
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

### Running
```bash
c-builder [options] run
```
#### Options available
 - `-r --release`: Run the project in release mode
 - `-b --benchmark`: Run the benchmarks
 - `-h --help`: Display help information

### Testing
```bash
c-builder [options] test
```
#### Options available
 - `-r --release`: Run the tests in release mode
 - `-h --help`: Display help information

## Configuration
The configuration file is located at `~/.config/c-builder/config.toml`.
All fields shown are required.
The default configuration is:
```toml
[package]
name = "Your Project"
version = "0.1.0"
authors = ["Your Names"]
src = "src"
benchmarks = "benchmarks"
    
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
