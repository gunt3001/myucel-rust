# Myucel

A simple CLI tool to search Reddit's /r/anime episode discussion threads, written in Rust.

## Features

- Search for episode discussion threads on Reddit's /r/anime.
- Display results in the terminal.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:
    ```sh
    git clone https://github.com/gunt3001/myucel-rust.git
    cd myucel-rust
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

Run the CLI tool with the name of the anime series you want to search for:

```sh
myucel <anime_name>
```

For example:

```sh
myucel "Outbreak Company"
```

## Example

```sh
myucel Dandadan
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
