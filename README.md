# Triton Windstorm

A rate-my-day app.

## 1. Improvments (TODO-list)

- CSS design
- crypt data
- graphs and statistics

## 2. Edit / compile this App

This app is built in Rust and WebAssembly thanks to seed.rs framework.

### 1. Clone your new repository to your local machine

### 2. Install / check required tools

1. Make sure you have basic tools installed:

   - [Rust](https://www.rust-lang.org) 
     - Check: `$ rustc -V` => `rustc 1.43.1 (8d69840ab 2020-05-04)`
     - Install: https://www.rust-lang.org/tools/install
   - [cargo-make](https://sagiegurari.github.io/cargo-make/)
     - Check: `$ cargo make -V` => `cargo-make 0.30.7`
     - Install: `$ cargo install cargo-make`

1. Platform-specific tools like `ssl` and `pkg-config`:
    - Follow recommendations in build errors (during the next chapter).

### 3. Prepare the project for edit

1. Open the project in your favorite IDE (recommended are [VS Code](https://code.visualstudio.com/) + [Rust Analyzer](https://rust-analyzer.github.io/)).
1. Open a new terminal tab / window and run: `cargo make serve`
1. Open a second terminal tab and run: `cargo make watch`
1. If you see errors, try to fix them or write on our [chat](https://discord.gg/JHHcHp5) or [forum](https://seed.discourse.group/).
1. Modify files like `README.md` and `Cargo.toml` as you wish.

### 4. Edit the app

1. Open [localhost:8000](http://localhost:8000) in a browser (We recommend Mozilla Firefox or Google Chrome).
1. Modify source files (e.g. `/src/lib.rs` or `/index.html`).
1. Watch compilation in the terminal tab where you run `cargo make watch`.
1. You can watch dev-server responses in the tab where you run `cargo make serve`.
1. Refresh your browser and see changes.
1. Go to step 2.

### 5. Prepare your project for deploy

1. Run `cargo make verify` in your terminal to format and lint the code.
1. Run `cargo make build_release`.
1. Upload `index.html` and `pkg` into your server's public folder.
