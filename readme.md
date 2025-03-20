# Agent QWEN

## Requirements

This project is build in Rust and uses Ollama to run the AI.
Make sure both are installed for your system:

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Ollama](https://ollama.com/)
- [qwen2.5-coder (AI Model)](https://ollama.com/library/qwen2.5-coder)

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Using a custom AI Model

The project currently uses the model qwen2.5-coder.
This model is specifically trained for code generation, code reasoning, and code repair.

You can use another model if you want but make sure it has `tools` enabled.

1. Search for models at [ollama](https://ollama.com/search)

> [!NOTE]
> Make sure you have checked the `Tools` badge!

2. Install the model locally:

```bash
ollama pull <Model-Name>
```

3. Adjust the model in the `src/main.rs`:

```rust
let model = "<your model name>".to_string();
```
