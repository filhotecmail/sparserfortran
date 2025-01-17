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

1. **Rust:** This project is developed in Rust. To install Rust, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Building the Project

To build the project, use the `cargo` (Rust package manager) in the root directory of the project:

```bash
cargo build --release
