# Terminal Opacity Configurator

This Rust project allows you to modify the opacity of a terminal window by editing the configuration file (`.wezterm.lua`). It replaces the `config.window_background_opacity` setting with the desired value provided via a command-line argument.

## Features

- Parse command-line arguments using the `clap` library.
- Modify the terminal opacity in the `.wezterm.lua` configuration file.
- Handle optional opacity input with a default fallback.

## Requirements

- Rust (version 1.56.0 or later)

## Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/yourusername/terminal-opacity-configurator.git
    cd terminal-opacity-configurator
    ```

2. **Build the project**:
    ```bash
    cargo build --release
    ```

3. **Run the executable**:
    ```bash
    ./target/release/terminal-opacity-configurator --opacity 0.8
    ```

## Usage

The program modifies the `config.window_background_opacity` setting in the specified configuration file. 

### Command-line Arguments

- `--opacity`, `-o`: (Optional) The desired opacity for the terminal window (e.g., `0.8`).

### Example

To set the terminal window opacity to `0.85`, run:

```bash
./target/release/terminal-opacity-configurator --opacity 0.85
```

If `--opacity` is not specified, then it will change opacity to 1 or 0.9(if opacity was 1).

```bash
./target/release/terminal-opacity-configurator
```
