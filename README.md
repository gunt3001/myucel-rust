# Myucel (Rust edition)

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

### Output

```
Searching Reddit /r/anime for Dandadan discussion threads...
Dandadan - Episode 8 discussion: https://www.reddit.com/r/anime/comments/1gwio6p/dandadan_episode_8_discussion/
Dandadan - Episode 7 discussion: https://www.reddit.com/r/anime/comments/1gr8662/dandadan_episode_7_discussion/
Dandadan - Episode 6 discussion: https://www.reddit.com/r/anime/comments/1gltuuu/dandadan_episode_6_discussion/
Dandadan - Episode 5 discussion: https://www.reddit.com/r/anime/comments/1ggi485/dandadan_episode_5_discussion/
Dandadan - Episode 4 discussion: https://www.reddit.com/r/anime/comments/1gb66al/dandadan_episode_4_discussion/
Dandadan - Episode 3 discussion: https://www.reddit.com/r/anime/comments/1g5ucaq/dandadan_episode_3_discussion/
Dandadan - Episode 2 discussion: https://www.reddit.com/r/anime/comments/1g0m8i3/dandadan_episode_2_discussion/
Dandadan - Episode 1 discussion: https://www.reddit.com/r/anime/comments/1fvbbz3/dandadan_episode_1_discussion/
```

## History & motivations

This is my personal go-to simple project when learning new programming languages.

It started off as a simple Python script back in 2013 (hence the project name), the source code of which is lost to time.

You can also find .NET/C# version here: https://github.com/gunt3001/Myucel.NET

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
