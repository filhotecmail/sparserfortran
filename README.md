# SparserFortran

**SparserFortran** is a program designed to extract methods from Fortran files and allow their removal or modification. It facilitates the processing of Fortran code, extracting subroutines and modules, and offers an efficient way to modify the code for performance and clarity.

## Features

- **Module Extraction:** Allows the extraction of modules from Fortran files.
- **Function and Subroutine Removal:** Capable of removing functions and subroutines from Fortran files.
- **Backup and Modification Logging:** Creates backups before removing any code and adds a comment to the removed code with the timestamp.
- **Command-Line Interface:** Controlled via the command line, enabling automation and easy integration.

## Technologies

- **Rust** - The program is developed in Rust, ensuring performance and safety.
- **Regex** - Used for searching and manipulating the contents of Fortran files.
- **Chrono** - For handling dates and times, used in backup creation and modification logging.

## Installation

### Dependencies

The project uses the following dependencies to handle various tasks:

1. **`argh`** - `argh = "0.1.8"`
   - This crate is used for argument parsing. It simplifies handling command-line arguments and provides a declarative syntax to define argument parsers.
   
2. **`regex`** - `regex = "1.5.4"`
   - A powerful regular expression library used for searching and manipulating the contents of Fortran files. It allows efficient pattern matching to identify and extract code structures like functions, subroutines, and modules.

3. **`termcolor`** - `termcolor = "1.1.2"`
   - A crate for printing colored output to the terminal. It's used for providing user-friendly and visually distinct messages, like success, warnings, or errors.

4. **`strsim`** - `strsim = "0.8.0"`
   - A crate for computing string similarity metrics like Levenshtein distance. It could be used in the future for more sophisticated matching when searching for functions or subroutines based on user input.

5. **`ascii-art`** - `ascii-art = "0.1.0"`
   - Used for generating ASCII art. This crate adds a touch of visual appeal to the command-line interface, providing some artistic representation or headers.

6. **`toml`** - `toml = "0.5"`
   - A library for parsing and working with TOML files. It’s used to manage configuration files in the project, such as settings related to the program's behavior and options.

7. **`fancy-regex`** - `fancy-regex = "0.8"`
   - An alternative regular expression crate that provides more advanced features and performance. It's used alongside `regex` to ensure the program has robust and fast pattern matching.

8. **`chrono`** - `chrono = "0.4"`
   - A crate for date and time handling. It’s used to generate timestamps when modifying or removing code, allowing for backup tracking and logging.

### Building the Project

To build the project, use the `cargo` (Rust package manager) in the root directory of the project:

```bash
cargo build --release
