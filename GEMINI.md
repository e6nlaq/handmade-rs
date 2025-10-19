# Gemini Workspace Context: Handmade Test Case Generator

## Project Overview

This project is a Rust-based command-line tool named "handmade" designed to streamline the creation of manual test cases for competitive programming platforms, likely for use with `online-judge-tools` (`oj`).

The tool interactively prompts the user to provide input and expected output for a new test case. It then automatically determines the next available test case number and saves the provided data into corresponding `.in` and `.out` files within a designated test directory.

## Key Technologies

-   **Language:** Rust (2024 Edition)
-   **Core Libraries:**
    -   `clap`: For parsing command-line arguments.
    -   `dialoguer`: For creating interactive command-line prompts.

## Building and Running

Standard Cargo commands are used to build and run the project.

-   **Build (Debug):**
    ```sh
    cargo build
    ```

-   **Build (Release):** The project is configured for an optimized release build.
    ```sh
    cargo build --release
    ```

-   **Run:**
    ```sh
    cargo run
    ```

    You can also pass arguments when running:
    ```sh
    cargo run -- --test my_tests --case-prefix custom
    ```

## Development Conventions

-   **File Structure:** The main application logic is located in `src/main.rs`.
-   **Command-Line Interface:** The tool provides the following options:
    -   `-t, --test <folder>`: Specifies the folder where test cases are stored (defaults to `"test"`).
    -   `-c, --case-prefix <prefix>`: Sets a custom prefix for the generated test case files (defaults to `"z_handmade"`).
-   **Workflow:** The tool reads the specified test directory to find existing handmade tests, determines the next sequential number, and then prompts the user for the input and output for the new test case.
