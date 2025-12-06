# Halfabet validator

A validator for entries to a [writing challenge on Writing Codidact](https://writing.codidact.com/posts/295120).

## :link: Webpage

[Access the validator directly](https://trichoplax.github.io/halfabet-validator/)

## :wrench: Development

### View
To access the webpage locally for development, you will need an HTTP server because the `.wasm` file will be blocked from loading if the HTML file is opened directly. For example, if you have Python installed, you can enter the following from the `web` directory:

```text
python -m http.server
```

You can then view the page in your browser at:

http://127.0.0.1:8000

### Compile
Changes to the [Rust](https://rust-lang.org/) code require 2 compilation steps due to targeting the web. If you do not already have `wasm-bindgen-cli` installed you can do so with:

```bash
cargo install -f wasm-bindgen-cli
```

#### Step 1
Compile with Cargo to produce the WASM file in the `target` directory:

```bash
cargo build --target=wasm32-unknown-unknown --release
```

#### Step 2
Use `wasm-bindgen` to optimise that WASM file and include it along with a JavaScript file to make use of it, in the `web` directory:

```bash
wasm-bindgen ./target/wasm32-unknown-unknown/release/halfabet_validator.wasm --out-dir web --no-typescript --target web
```

GitHub pages is currently configured to serve the contents of the `web` directory. It does no compilation. Pushing changes to the Rust code will have no effect on GitHub pages unless you first compile (both the Cargo and `wasm-bindgen` steps) so that the `web` directory reflects those changes.
