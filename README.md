# TerminusDM: Terminal-UI Download Manager

TerminusDM is a powerful terminal application download manager built with Rust. It offers a user-friendly terminal interface for managing your downloads efficiently.

### Features

- Download files the command line.
- Maintain a history of your downloads - with terminal ui.
- Splitting files into smaller parts and downloading them concurrently
- Resume interrupted downloads seamlessly.
- Built with Rust, ensuring performance and reliability.

### Installation

- **\*Prerequisites:**

Rust and Cargo installed on your system. You can find installation instructions at [https://www.rust-lang.org/](https://www.rust-lang.org/).

- **\*Install it with cargo:**

  ```bash
  cargo install terminusdm
  ```

### Usage

1. Open a terminal.

2. Run the `terminusdm`

   ```bash
   terminusdm
   ```

TerminusDM will display a user-friendly interface within the terminal window, showing download history and other relevant information.

### Credits

TerminusDM would not be possible without the following amazing Rust crates:

- [**ratatui**](https://ratatui.rs/) for its powerful terminal UI capabilities.
- [**trauma**](https://crates.io/crates/trauma) rust library for download.
- [**tokio**](https://tokio.rs/) for handling asynchronous operations seamlessly.
- [**reqwest**](https://crates.io/crates/reqwest) for making HTTP requests to download files.
