# MiniGrep

MiniGrep is a simple command-line utility for searching text within files. It allows users to specify a search query and a file path, and it prints the lines in the file that contain the search query.

## Usage

To use MiniGrep, follow these steps:

1. Ensure you have Rust installed on your system. If not, you can install it from [rustup.rs](https://rustup.rs/).

2. Clone the MiniGrep repository to your local machine:
   ```
   git clone https://github.com/weldonkipchirchir/minigrep.git
   ```

3. Navigate to the `minigrep` directory:
   ```
   cd minigrep
   ```

4. Build and run MiniGrep using Cargo:
   ```
   cargo run -- <search_query> <file_path>
   ```

   Replace `<search_query>` with the string you want to search for and `<file_path>` with the path to the file you want to search within.

5. MiniGrep will display the lines from the file that contain the search query.

## Features

- **Case Sensitivity**: By default, MiniGrep performs case-sensitive searches. It will only match lines that exactly match the search query.
- **Case Insensitivity**: MiniGrep supports case-insensitive searches. To enable this feature, set the `IGNORE_CASE` environment variable before running MiniGrep.
- **Error Handling**: MiniGrep provides error messages for invalid command-line arguments or file read errors.ep.

## Example

Suppose you have a file named `example.txt` with the following contents:

```
Rust:
safe, fast, productive.
Pick three.
Duct tape.
```

To search for the string "safe" in a case-sensitive manner, run the following command:

```
cargo run -- safe example.txt
```

This will output:

```
safe, fast, productive.
```

To perform a case-insensitive search for the string "rUsT", run the following command:

```
IGNORE_CASE=true cargo run -- rUsT example.txt
```

This will output:

```
Rust:
```

## Contributing

Contributions to MiniGrep are welcome! If you encounter any issues or have suggestions for improvements, please open an issue or submit a pull request on the [GitHub repository](https://github.com/weldonkipchirchir/minigrep).

## Author
MiniGrep was created by Weldon.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
