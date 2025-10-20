# handmade-rs

![demo](https://github.com/user-attachments/assets/0939cfa8-5c78-4358-ad13-c7f956f6d72e)

`handmade-rs` is a command-line tool to interactively create handmade test cases for competitive programming, designed to work smoothly with environments like `online-judge-tools`.

## Description

When you're working on a competitive programming problem, you often need to create your own test cases to verify your solution. This tool simplifies that process.

It automatically finds the next available test case number in your test directory, prompts you to enter the input and expected output, and saves them as correctly named `.in` and `.out` files.

## Installation

Install the project using Cargo:

```sh
cargo install --git https://github.com/e6nlaq/handmade-rs.git
```


## Usage

Run the executable from your project's root directory.

```sh
handmade
```

The tool will guide you through creating a new test case. It will first ask for the input line by line. To finish the input, enter an empty line. After that, it will ask for the output in the same way.

### Command-Line Options

You can customize the tool's behavior with the following options:

-   `-t, --test <FOLDER>`
    -   Specifies the directory where your test cases are stored.
    -   Defaults to `test`.

-   `-c, --case-prefix <PREFIX>`
    -   Sets the prefix for the generated test case files.
    -   Defaults to `z_handmade`.
