InterSpace
==========
**The graphical interface pipeline visualized**

_[open the site](https://erithax.com)_
![image](https://github.com/Erithax/interspace/assets/64774344/15a0881a-24b1-42b8-982a-f55438895e7c)

# Tech
InterSpace is made using [Dioxus](https://github.com/dioxuslabs/dioxus) (_React, but in Rust_) and deployed using Github Pages. The data is entered in Rust and preprocessed and serialized at compile time. A proc macro is provides convenient syntax to enter the tree data. A custom layout algorithm had to be written for the dynamic partition chart, which uses wasm_bindgen's [web_sys](https://github.com/rustwasm/wasm-bindgen/tree/main/crates/web-sys). Find all dependencies in `./Cargo.toml`.

# Contribute 
1) Install the stable [Rust toolchain](https://rust-lang.org)
2) (Recommended) Install Rust LSP support in your code-editor/IDE
3) Install Dioxus CLI with `cargo install dioxus-cli`
4) Fork the repo and clone it locally
5) Serve the website locally with `dx serve` 
6) After editing the constellation, or to refetch Github stars, run `cargo run --bin regen_data` before serving again
7) Build release with `dx build --release`
8) Create pull request

# Todo
- [ ] Improve data
- [ ] Improve guide
- [ ] Universal popularity metric
- [ ] Selection sorting
- [ ] Manual selection and comparison tool
- [ ] Sharable chart settings
- [ ] ? Include shell component in chart?
- [ ] ? UI benchmark info
