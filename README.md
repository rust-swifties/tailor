# Tailor

**Tailor** is a command-line tool inspired by the utility of the UNIX `tail` command and the logical inclusivity of the `or` operator. It allows you to extract and manipulate the final lines of text files with precision and flexibility. Unlike an "Anti-Hero," Tailor is here to save your day without any identity crisis!

## Features

- **Tail-like Functionality**: Extract the last `n` lines of a file, similar to the UNIX `tail` command.
- **Pattern Matching**: Filter lines using regex patterns for precise selection.
- **Logical OR Operations**: Combine multiple conditions for line selection with an `or`-style logic.
- **Custom Output Formatting**: Style your output with customizable formats, such as JSON or plain text.

## Installation

Clone the repository and install the dependencies:

```bash
git clone https://github.com/rust-swifties/tailor.git
cd tailor
cargo run
```

## Usage

Run `tailor` with various options to suit your needs:

```bash
tailor [options] <file>
```

### Options

- `-n <number>`: Specify the number of lines to display from the end of the file (default: 10).
- `-p <pattern>`: Filter lines matching a regex pattern.
- `-o <format>`: Customize output format (e.g., `json`, `plain`).
- `-v`: Enable verbose mode for detailed output.

### Examples

1. Display the last 5 lines of a file:
   ```bash
   tailor -n 5 data.txt
   ```

2. Filter lines containing "error" or "warning" using regex:
   ```bash
   tailor -p "error|warning" logfile.txt
   ```

3. Output the last 10 lines in JSON format:
   ```bash
   tailor -o json output.txt
   ```

## Why Tailor?

- **UNIX `tail` Power**: Builds on the simplicity and utility of `tail` for robust file processing.
- **Logical `or` Flexibility**: Allows inclusive filtering to capture exactly what you need.
- **Customizable Output**: Formats output to suit your preferences, making it versatile for various use cases.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes. Ensure your code follows the project's style guidelines and includes tests.

## License

This project is licensed under the WTFPL License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- The UNIX `tail` command, for its timeless utility.
- The logical `or` operator, for teaching us inclusivity.
