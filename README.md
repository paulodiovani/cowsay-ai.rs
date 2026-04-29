# cowsay-ai

A Rust-powered `cowsay` implementation that integrates with OpenAI-compatible LLMs to generate witty, random messages when no input is provided.

```
  _______________________________________
 / A penny saved is a penny earned, but   \
 / with my tips! 🌿                        \
                                      _
       \   ^__^
       \   (oo)\_______
           (__)\       \\
               ||  ||    \\
               ||  ||     \\
```

## Features

- 🐄 Custom ASCII cow renderer.
- 🤖 LLM integration for automated message generation.
- 🛠️ Configurable via environment variables.
- 🛡️ Graceful fallbacks when LLM services are unavailable.

## Installation

Ensure you have [Rust](https://rustup.rs/) installed, then:

```bash
git clone <repository-url>
cd cowsay-ai
cargo build --release
```

The binary will be located at `./target/release/cowsay-ai`.

## Configuration

The application is configured using the following environment variables:

| Variable       | Description                              | Example                       |
| -------------- | ---------------------------------------- | ----------------------------- |
| `LLM_BASE_URL` | Base URL of the OpenAI-compatible server | `https://api.groq.com/openai` |
| `LLM_MODEL`    | The model ID to use for generation       | `llama-3.3-70b-versatile`     |
| `LLM_API_KEY`  | Your API key for the LLM provider        | `sk-your-key-here`            |

## Usage

### With a manual message:

```bash
cargo run -- "Hello from Rust!"
```

### With LLM generation:

Set your environment variables and run without arguments:

```bash
export LLM_BASE_URL=http://localhost:11434
export LLM_MODEL=qwen2.5:1.5b
export LLM_API_KEY=
cargo run
```

### Without LLM configuration:

If the environment variables are not set, the cow will simply say:
`"There is nothing to say."`

## License

MIT
