# FMT

`fmt` is a command-line utility for formatting text and tables. It offers various formatting options such as cleaning input text, aligning text, wrapping text to specified widths, truncating text, checking if text is numeric or hexadecimal, and formatting tables.

## Features

- **Text Cleaning**: Remove leading and trailing blank lines and whitespace.
- **Text Alignment**: Align text to the left, right, or center.
- **Text Wrapping**: Wrap text to a specified width.
- **Text Truncation**: Truncate text to a specified width.
- **Numeric and Hexadecimal Checking**: Verify if the input text is a valid numeric or hexadecimal string.
- **Table Formatting**: Format tabular data into a well-structured table.

## Installation

To install `fmt`, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/fmt.git
cd fmt
cargo build --release
```

Alternatively, you can add it to your Cargo.toml:

```toml
[dependencies]
fmt = { git = "https://github.com/yourusername/fmt.git" }
```

## Usage

### Command Structure

The basic command structure for using `fmt` is:

```bash
fmt <subcommand> [options]
```

### Subcommands

- **Clean**: Sanitizes the input text.
  ```bash
  fmt clean --text "   Your text here   "
  ```

- **Align Right**: Aligns text to the right.
  ```bash
  fmt right --text "Align this text" --width 20
  ```

- **Align Left**: Aligns text to the left.
  ```bash
  fmt left --text "Align this text" --width 20
  ```

- **Center**: Centers text according to specified width.
  ```bash
  fmt center --text "Center this text" --width 20
  ```

- **Wrap**: Wraps text to a specified width.
  ```bash
  fmt wrap --text "This is a long text that needs to be wrapped." --width 30
  ```

- **Truncate**: Truncates text to a specified width.
  ```bash
  fmt truncate --text "This text will be truncated." --width 20
  ```

- **Format Check**: Check if the format is numeric or hexadecimal.
  ```bash
  fmt is hex --text "1A3F"
  fmt is numeric --text "12345"
  ```

- **Table Format**: Formats tabular data into a structured table.
  ```bash
  fmt table --text "Header1, Header2, Header3\nRow1Col1, Row1Col2, Row1Col3\nRow2Col1, Row2Col2, Row2Col3"
  ```

## Examples

### Clean Text
```bash
fmt clean --text "   Hello, World!   "
```

### Align Right
```bash
fmt right --text "Hello" --width 10
```

### Check if Hex
```bash
fmt is hex --text "A0B1C2"
```

### Format a Table
```bash
fmt table --text "Name, Age, City\nAlice, 30, New York\nBob, 25, Los Angeles"
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
